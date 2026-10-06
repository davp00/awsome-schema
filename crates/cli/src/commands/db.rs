use anyhow::{Context, Result, bail};
use parser::{print_config_blocks, print_edge_block, print_model_block, print_schema};
use schema_core::{DbPullInput};

use crate::di::AppContext;
use crate::output::Printer;
use crate::schema_layout::{split_schema_dir, table_fragment_path, CONFIG_FILE, TABLES_DIR, TABLE_SUFFIX};

pub fn run_pull(context: &AppContext, split_by_table: bool, force: bool, printer: &Printer) -> Result<()> {
    let preserve = context.load_schema()?;
    ensure_can_write(context, split_by_table, force)?;

    let output = context
        .db_pull
        .execute(DbPullInput { preserve: preserve.clone() })
        .context("db pull failed")?;

    let written = if split_by_table {
        write_split_schema(context, &output.schema)?
    } else {
        write_single_schema(context, &output.schema)?
    };

    let message = if split_by_table {
        format!(
            "Pulled {} models, {} edges → {} ({} table files).",
            output.schema.models.len(),
            output.schema.edges.len(),
            written,
            output.schema.models.len() + output.schema.edges.len()
        )
    } else {
        format!(
            "Pulled {} models, {} edges → {}.",
            output.schema.models.len(),
            output.schema.edges.len(),
            written
        )
    };
    printer.success(&message);
    if output.lossy {
        printer.warn(
            "Some DSL-only details (@relation, edge endpoints, generators) may not round-trip from the database.",
        );
    }

    Ok(())
}

pub fn run_push(context: &AppContext, printer: &Printer) -> Result<()> {
    let schema = context.load_schema()?;
    let output =
        context.db_push.execute(schema_core::DbPushInput { datasource: schema.datasource })?;
    printer.success(&format!("Pushed schema to database ({} bytes).", output.statements.len()));
    Ok(())
}

fn ensure_can_write(context: &AppContext, split_by_table: bool, force: bool) -> Result<()> {
    if force {
        return Ok(());
    }

    if split_by_table {
        let dir = split_schema_dir(&context.schema_path);
        let tables_dir = format!("{dir}/{TABLES_DIR}");
        if context.filesystem.exists(&tables_dir) {
            let entries = context
                .filesystem
                .list_dir(&tables_dir)
                .map_err(|error| anyhow::anyhow!(error.to_string()))?;
            if entries.iter().any(|name| name.ends_with(TABLE_SUFFIX)) {
                bail!("refusing to overwrite `{tables_dir}` without --force");
            }
        }
        return Ok(());
    }

    if context.filesystem.exists(&context.schema_path)
        && !context.filesystem.is_directory(&context.schema_path)
    {
        let content = context
            .filesystem
            .read_to_string(&context.schema_path)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?;
        if !content.trim().is_empty() {
            bail!("refusing to overwrite `{}` without --force", context.schema_path);
        }
    }

    Ok(())
}

fn write_single_schema(context: &AppContext, schema: &schema_core::DatabaseSchema) -> Result<String> {
    let content = print_schema(schema);
    context
        .filesystem
        .write_string(&context.schema_path, &content)
        .map_err(|error| anyhow::anyhow!(error.to_string()))?;
    Ok(context.schema_path.clone())
}

fn write_split_schema(context: &AppContext, schema: &schema_core::DatabaseSchema) -> Result<String> {
    let dir = split_schema_dir(&context.schema_path);
    context.filesystem.create_dir_all(&dir).map_err(|error| anyhow::anyhow!(error.to_string()))?;
    context
        .filesystem
        .create_dir_all(&format!("{dir}/{TABLES_DIR}"))
        .map_err(|error| anyhow::anyhow!(error.to_string()))?;

    let config = print_config_blocks(schema);
    context
        .filesystem
        .write_string(&format!("{dir}/{CONFIG_FILE}"), &config)
        .map_err(|error| anyhow::anyhow!(error.to_string()))?;

    let mut expected_files = Vec::new();

    for model in &schema.models {
        let table = model.table_name(&schema.naming);
        let path = table_fragment_path(&dir, &table);
        expected_files.push(path.clone());
        context
            .filesystem
            .write_string(&path, &print_model_block(model))
            .map_err(|error| anyhow::anyhow!(error.to_string()))?;
    }

    for edge in &schema.edges {
        let table = edge.table_name(&schema.naming);
        let path = table_fragment_path(&dir, &table);
        expected_files.push(path.clone());
        context
            .filesystem
            .write_string(&path, &print_edge_block(edge))
            .map_err(|error| anyhow::anyhow!(error.to_string()))?;
    }

    let tables_dir = format!("{dir}/{TABLES_DIR}");
    if context.filesystem.is_directory(&tables_dir) {
        for name in context.filesystem.list_dir(&tables_dir).map_err(|error| anyhow::anyhow!(error.to_string()))? {
            if !name.ends_with(TABLE_SUFFIX) {
                continue;
            }
            let path = format!("{tables_dir}/{name}");
            if !expected_files.iter().any(|expected| expected.ends_with(&name)) {
                context.filesystem.remove_file(&path).map_err(|error| anyhow::anyhow!(error.to_string()))?;
            }
        }
    }

    Ok(dir)
}

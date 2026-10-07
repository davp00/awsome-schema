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
        if !context.filesystem.exists(&tables_dir) {
            return Ok(());
        }
        let entries = fs_result(context.filesystem.list_dir(&tables_dir))?;
        anyhow::ensure!(
            !entries.iter().any(|name| name.ends_with(TABLE_SUFFIX)),
            "refusing to overwrite `{tables_dir}` without --force"
        );
        return Ok(());
    }

    if !context.filesystem.exists(&context.schema_path)
        || context.filesystem.is_directory(&context.schema_path)
    {
        return Ok(());
    }
    let content = fs_result(context.filesystem.read_to_string(&context.schema_path))?;
    anyhow::ensure!(
        content.trim().is_empty(),
        "refusing to overwrite `{}` without --force",
        context.schema_path
    );
    Ok(())
}

fn write_single_schema(context: &AppContext, schema: &schema_core::DatabaseSchema) -> Result<String> {
    let content = print_schema(schema);
    fs_result(context.filesystem.write_string(&context.schema_path, &content))?;
    Ok(context.schema_path.clone())
}

fn write_split_schema(context: &AppContext, schema: &schema_core::DatabaseSchema) -> Result<String> {
    let dir = split_schema_dir(&context.schema_path);
    fs_result(context.filesystem.create_dir_all(&dir))?;
    fs_result(context.filesystem.create_dir_all(&format!("{dir}/{TABLES_DIR}")))?;

    let config = print_config_blocks(schema);
    fs_result(context.filesystem.write_string(&format!("{dir}/{CONFIG_FILE}"), &config))?;

    let mut expected_files = Vec::new();

    for model in &schema.models {
        let table = model.table_name(&schema.naming);
        let path = table_fragment_path(&dir, &table);
        expected_files.push(path.clone());
        fs_result(context.filesystem.write_string(&path, &print_model_block(model)))?;
    }

    for edge in &schema.edges {
        let table = edge.table_name(&schema.naming);
        let path = table_fragment_path(&dir, &table);
        expected_files.push(path.clone());
        fs_result(context.filesystem.write_string(&path, &print_edge_block(edge)))?;
    }

    let tables_dir = format!("{dir}/{TABLES_DIR}");
    // `create_dir_all` above guarantees `tables_dir` exists as a directory.
    for name in fs_result(context.filesystem.list_dir(&tables_dir))? {
        if !name.ends_with(TABLE_SUFFIX) {
            continue;
        }
        let path = format!("{tables_dir}/{name}");
        if !expected_files.iter().any(|expected| expected.ends_with(&name)) {
            fs_result(context.filesystem.remove_file(&path))?;
        }
    }

    Ok(dir)
}

fn fs_result<T>(result: std::result::Result<T, schema_core::DomainError>) -> Result<T> {
    result.map_err(|error| anyhow::anyhow!(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_context_with_introspector;
    use schema_core::{DatabaseSchema, DomainError, SchemaIntrospector};
    use std::sync::Arc;

    struct NoopPull;

    impl SchemaIntrospector for NoopPull {
        fn introspect(
            &self,
            _config: &schema_core::DatabaseConfig,
            preserve: &DatabaseSchema,
        ) -> Result<DatabaseSchema, DomainError> {
            Ok(preserve.clone())
        }
    }

    #[test]
    fn ensure_can_write_ok_for_empty_tables_dir_and_empty_file() {
        let temp = tempfile::tempdir().expect("tempdir");
        let schema_path = temp.path().join("awesome.schema");
        std::fs::write(
            &schema_path,
            r#"datasource db {
  provider = "surrealdb"
  url = "ws://127.0.0.1:8000"
  namespace = "test"
  database = "main"
}
model User { id @id }
"#,
        )
        .expect("schema");

        let schema_dir = temp.path().join("schema");
        let tables_dir = schema_dir.join("tables");
        std::fs::create_dir_all(&tables_dir).expect("tables");
        std::fs::write(tables_dir.join("notes.txt"), "x\n").expect("notes");

        let context = test_context_with_introspector(
            schema_path.to_string_lossy().into_owned(),
            temp.path().join("migrations").to_string_lossy().into_owned(),
            Arc::new(NoopPull),
        )
        .expect("context");

        ensure_can_write(&context, true, false).expect("empty tables dir");
        // Stale fragment removed by write_split after pull.
        std::fs::write(tables_dir.join("stale.schema"), "model Gone { id @id }\n").expect("stale");
        run_pull(&context, true, true, &crate::output::Printer::new()).expect("pull split");
        assert!(!tables_dir.join("stale.schema").exists());

        // Point schema_path at an empty file for the single-file branch.
        let empty_path = temp.path().join("empty.schema");
        std::fs::write(&empty_path, "   \n").expect("empty");
        let empty_ctx = test_context_with_introspector(
            empty_path.to_string_lossy().into_owned(),
            temp.path().join("migrations").to_string_lossy().into_owned(),
            Arc::new(NoopPull),
        )
        .expect("empty ctx");
        ensure_can_write(&empty_ctx, false, false).expect("empty schema file");

        // tables_dir missing → early Ok for split_by_table (isolated parent so
        // split_schema_dir does not reuse the pulled schema/ from above).
        let nested = temp.path().join("nested");
        std::fs::create_dir_all(&nested).expect("nested");
        let fresh = nested.join("fresh.schema");
        std::fs::write(
            &fresh,
            r#"datasource db {
  provider = "surrealdb"
  url = "ws://127.0.0.1:8000"
  namespace = "test"
  database = "main"
}
model User { id @id }
"#,
        )
        .expect("fresh");
        let fresh_ctx = test_context_with_introspector(
            fresh.to_string_lossy().into_owned(),
            temp.path().join("migrations2").to_string_lossy().into_owned(),
            Arc::new(NoopPull),
        )
        .expect("fresh ctx");
        ensure_can_write(&fresh_ctx, true, false).expect("no tables dir yet");

        // Missing schema file → early Ok for single-file write.
        let missing = temp.path().join("missing.schema");
        let missing_ctx = test_context_with_introspector(
            missing.to_string_lossy().into_owned(),
            temp.path().join("migrations3").to_string_lossy().into_owned(),
            Arc::new(NoopPull),
        )
        .expect("missing ctx");
        ensure_can_write(&missing_ctx, false, false).expect("missing file ok");

        let mapped = fs_result::<()>(Err(DomainError::WriteFailed("boom".into()))).expect_err("map");
        assert!(mapped.to_string().contains("boom"));
    }
}

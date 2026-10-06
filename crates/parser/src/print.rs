use std::fmt::Write as _;

use core::{
    DatabaseSchema, Datasource, Edge, Field, FieldType, Generator, Model, NamingCase,
    ObjectTypeDefinition, ObjectTypeField, TableMode,
};

pub fn print_schema(schema: &DatabaseSchema) -> String {
    let mut out = String::new();
    print_datasource(&mut out, &schema.datasource);
    if schema.naming.tables != NamingCase::SnakeCase || schema.naming.fields.is_some() {
        print_naming(&mut out, &schema.naming);
    }
    for generator in &schema.generators {
        print_generator(&mut out, generator);
    }
    for object_type in &schema.object_types {
        print_object_type(&mut out, object_type);
    }
    for model in &schema.models {
        print_model(&mut out, model);
    }
    for edge in &schema.edges {
        print_edge(&mut out, edge);
    }
    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

pub fn print_config_blocks(schema: &DatabaseSchema) -> String {
    let mut out = String::new();
    print_datasource(&mut out, &schema.datasource);
    if schema.naming.tables != NamingCase::SnakeCase || schema.naming.fields.is_some() {
        print_naming(&mut out, &schema.naming);
    }
    for generator in &schema.generators {
        print_generator(&mut out, generator);
    }
    for object_type in &schema.object_types {
        print_object_type(&mut out, object_type);
    }
    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

pub fn print_model_block(model: &Model) -> String {
    let mut out = String::new();
    print_model(&mut out, model);
    out
}

pub fn print_edge_block(edge: &Edge) -> String {
    let mut out = String::new();
    print_edge(&mut out, edge);
    out
}

fn print_datasource(out: &mut String, datasource: &Datasource) {
    let _ = writeln!(out, "datasource db {{");
    let _ = writeln!(out, "  provider = \"{}\"", datasource.provider);
    if let Some(url) = &datasource.url {
        let _ = writeln!(out, "  url      = {}", format_string_value(url));
    }
    if let Some(namespace) = &datasource.namespace {
        let _ = writeln!(out, "  namespace = \"{}\"", namespace);
    }
    if let Some(database) = &datasource.database {
        let _ = writeln!(out, "  database  = \"{}\"", database);
    }
    for (key, value) in &datasource.extra {
        let _ = writeln!(out, "  {key} = \"{value}\"");
    }
    let _ = writeln!(out, "}}\n");
}

fn print_naming(out: &mut String, naming: &core::NamingConvention) {
    let _ = writeln!(out, "naming {{");
    let _ = writeln!(out, "  tables = \"{}\"", naming_case_str(naming.tables));
    if let Some(fields) = naming.fields {
        let _ = writeln!(out, "  fields = \"{}\"", naming_case_str(fields));
    }
    let _ = writeln!(out, "}}\n");
}

fn print_generator(out: &mut String, generator: &Generator) {
    let _ = writeln!(out, "generator client {{");
    let _ = writeln!(out, "  provider = \"{}\"", generator.provider);
    let _ = writeln!(out, "  output   = \"{}\"", generator.output);
    for (key, value) in &generator.extra {
        let _ = writeln!(out, "  {key} = \"{value}\"");
    }
    let _ = writeln!(out, "}}\n");
}

fn print_object_type(out: &mut String, object_type: &ObjectTypeDefinition) {
    let _ = write!(out, "type {}", object_type.name);
    if object_type.flexible {
        let _ = write!(out, " @flexible");
    }
    let _ = writeln!(out, " {{");
    for field in &object_type.fields {
        print_object_type_field(out, field);
    }
    let _ = writeln!(out, "}}\n");
}

fn print_object_type_field(out: &mut String, field: &ObjectTypeField) {
    let _ = write!(out, "  {}", field.name);
    let _ = write!(out, " {}", print_field_type(&field.field_type));
    if field.optional {
        let _ = write!(out, "?");
    }
    let _ = writeln!(out);
}

fn print_model(out: &mut String, model: &Model) {
    let _ = writeln!(out, "model {} {{", model.name);
    let mut fields = model.fields.clone();
    fields.sort_by(|a, b| a.name.cmp(&b.name));
    for field in &fields {
        print_field(out, field, "  ", Some(model));
    }
    print_table_attributes(out, model.table_mode, model.permissions.as_deref());
    for index in &model.indexes {
        let fields = index.fields.join(", ");
        let _ = write!(out, "  @@index([{fields}])");
        if index.unique {
            let _ = write!(out, " @unique");
        }
        let _ = writeln!(out);
    }
    let _ = writeln!(out, "}}\n");
}

fn print_edge(out: &mut String, edge: &Edge) {
    let _ = writeln!(out, "edge {} {{", edge.name);
    let _ = writeln!(out, "  in  {}", edge.in_model);
    let _ = writeln!(out, "  out {}", edge.out_model);
    for field in &edge.fields {
        print_field(out, field, "  ", None);
    }
    print_table_attributes(out, edge.table_mode, edge.permissions.as_deref());
    let _ = writeln!(out, "}}\n");
}

fn print_table_attributes(out: &mut String, mode: TableMode, permissions: Option<&str>) {
    let mode_name = match mode {
        TableMode::Schemafull => "schemafull",
        TableMode::Schemaless => "schemaless",
    };
    let _ = writeln!(out, "  @@table({mode_name})");
    if let Some(permission) = permissions {
        let _ = writeln!(out, "  @@permissions(\"{permission}\")");
    }
}

fn print_field(out: &mut String, field: &Field, indent: &str, model: Option<&Model>) {
    let _ = write!(out, "{indent}{}", field.name);
    if field.is_id {
        let _ = write!(out, " @id");
    } else if let Some(type_name) = field_type_for_print(field, model) {
        let _ = write!(out, " {type_name}");
        if field.optional {
            let _ = write!(out, "?");
        }
    }
    if field.flexible {
        let _ = write!(out, " @flexible");
    }
    if field.unique {
        let _ = write!(out, " @unique");
    }
    if let Some(default) = &field.default_value {
        if field.default_always {
            let _ = write!(out, " @defaultAlways({default})");
        } else {
            let _ = write!(out, " @default({default})");
        }
    }
    if let Some(value) = &field.value_expression {
        let _ = write!(out, " @value({value})");
    }
    if field.readonly {
        let _ = write!(out, " @readonly");
    }
    if field.link_target.is_some() || matches!(field.field_type, FieldType::Model(_)) {
        let _ = write!(out, " @link");
    }
    if let Some(relation) = &field.relation_name {
        let _ = write!(out, " @relation(\"{relation}\")");
    }
    let _ = writeln!(out);
}

fn field_type_for_print(field: &Field, model: Option<&Model>) -> Option<String> {
    if field.is_id {
        return None;
    }
    match &field.field_type {
        FieldType::Model(name) => Some(name.clone()),
        FieldType::RecordId(name) => {
            if model.is_some_and(|m| m.name.eq_ignore_ascii_case(name)) {
                None
            } else {
                Some(format!("record<{name}>"))
            }
        }
        other => Some(print_field_type(other)),
    }
}

fn print_field_type(field_type: &FieldType) -> String {
    match field_type {
        FieldType::String => "string".into(),
        FieldType::Int => "int".into(),
        FieldType::Float => "float".into(),
        FieldType::Bool => "bool".into(),
        FieldType::Datetime => "datetime".into(),
        FieldType::Object => "object".into(),
        FieldType::Array(inner) => format!("{}[]", print_field_type(inner)),
        FieldType::RecordId(target) => format!("record<{target}>"),
        FieldType::Model(name) => name.clone(),
        FieldType::Custom(value) => value.clone(),
    }
}

fn naming_case_str(case: NamingCase) -> &'static str {
    match case {
        NamingCase::SnakeCase => "snake_case",
        NamingCase::CamelCase => "camelCase",
        NamingCase::PascalCase => "PascalCase",
        NamingCase::KebabCase => "kebab-case",
        NamingCase::Lowercase => "lowercase",
    }
}

fn format_string_value(value: &str) -> String {
    if value.starts_with("env(") {
        value.to_owned()
    } else {
        format!("\"{value}\"")
    }
}

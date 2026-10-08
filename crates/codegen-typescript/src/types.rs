#![allow(clippy::missing_errors_doc, clippy::too_many_lines)]

use std::fmt::Write as _;

use core::{DatabaseSchema, Edge, Field, FieldType, Model, NamingContext, ObjectTypeDefinition};

pub fn emit_schema_types(
    out: &mut Vec<String>,
    schema: &DatabaseSchema,
    naming: &NamingContext<'_>,
) {
    for object_type in &schema.object_types {
        emit_object_type(out, object_type, naming);
    }

    for model in &schema.models {
        emit_model_record(out, schema, model, naming);
        emit_model_selected(out, schema, model, naming);
        emit_model_inputs(out, schema, model, naming);
    }

    for edge in &schema.edges {
        emit_edge_record(out, schema, edge, naming);
        emit_edge_selected(out, schema, edge, naming);
        emit_edge_inputs(out, schema, edge, naming);
    }

    emit_select_type_utils(out);
    for model in &schema.models {
        emit_model_select_payload(out, schema, model, naming);
    }
    for edge in &schema.edges {
        emit_edge_select_payload(out, schema, edge, naming);
    }

    emit_where_shared_types(out);
    for model in &schema.models {
        emit_model_where_input(out, schema, model, naming);
        emit_model_where_unique_input(out, model, naming);
    }
    for edge in &schema.edges {
        emit_edge_where_input(out, schema, edge, naming);
        emit_edge_where_unique_input(out, edge, naming);
    }

    emit_order_by_shared_types(out);
    for model in &schema.models {
        emit_model_order_by_input(out, model, naming);
        emit_model_aggregate_types(out, model, naming);
    }
    for edge in &schema.edges {
        emit_edge_order_by_input(out, edge, naming);
        emit_edge_aggregate_types(out, edge, naming);
    }

    emit_tables_const(out, schema, naming);
    emit_select_meta_registry(out, schema, naming);
    emit_write_meta_registry(out, schema, naming);
}

fn record_id_type(table: &str) -> String {
    format!("RecordId<\"{table}\">")
}

fn emit_object_type(
    out: &mut Vec<String>,
    object_type: &ObjectTypeDefinition,
    naming: &NamingContext<'_>,
) {
    out.push(format!("export type {} = {{", object_type.name));
    for field in &object_type.fields {
        let ts = map_object_field_type(&field.field_type, naming);
        let optional = if field.optional { "?" } else { "" };
        out.push(format!("  {}{optional}: {};", field.name, ts));
    }
    if object_type.flexible {
        out.push("  [key: string]: unknown;".to_owned());
    }
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_model_record(
    out: &mut Vec<String>,
    schema: &DatabaseSchema,
    model: &Model,
    naming: &NamingContext<'_>,
) {
    out.push(format!("export type {} = {{", model.name));
    for field in model.fields.iter().filter(|f| !f.name.contains('.')) {
        if should_omit_on_record(field) {
            continue;
        }
        let ts = map_model_field_record(schema, model, field, naming);
        let optional = if field.optional { "?" } else { "" };
        let name = naming.field_name(field);
        out.push(format!("  {name}{optional}: {ts};"));
    }
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_model_selected(
    out: &mut Vec<String>,
    schema: &DatabaseSchema,
    model: &Model,
    naming: &NamingContext<'_>,
) {
    out.push(format!("export type {}Selected = {{", model.name));
    for field in model.fields.iter().filter(|f| !f.name.contains('.')) {
        let ts = map_model_field_selected(schema, model, field, naming);
        let optional =
            if field.optional || field.is_computed_link() || field.relation_name.is_some() {
                "?"
            } else {
                ""
            };
        let name = naming.field_name(field);
        out.push(format!("  {name}{optional}: {ts};"));
    }
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_model_inputs(
    out: &mut Vec<String>,
    schema: &DatabaseSchema,
    model: &Model,
    naming: &NamingContext<'_>,
) {
    out.push(format!("export type {}CreateScalars = {{", model.name));
    for field in model.fields.iter().filter(|f| !f.name.contains('.')) {
        if field.is_id || should_omit_on_record(field) {
            continue;
        }
        let ts = map_model_field_record(schema, model, field, naming);
        let optional = if field.optional { "?" } else { "" };
        let name = naming.field_name(field);
        out.push(format!("  {name}{optional}: {ts};"));
    }
    out.push("};".to_owned());
    out.push(String::new());

    let stored_overrides: Vec<(&Field, String)> = model
        .fields
        .iter()
        .filter(|f| !f.name.contains('.') && f.is_stored_link())
        .map(|f| (f, naming.field_name(f)))
        .collect();

    if stored_overrides.is_empty() {
        out.push(format!(
            "export type {}CreateInput = {}CreateScalars & {{",
            model.name, model.name
        ));
    } else {
        let omit_keys = stored_overrides
            .iter()
            .map(|(_, n)| format!("\"{n}\""))
            .collect::<Vec<_>>()
            .join(" | ");
        out.push(format!(
            "export type {}CreateInput = Omit<{}CreateScalars, {omit_keys}> & {{",
            model.name, model.name
        ));
        for (field, name) in &stored_overrides {
            let target = field
                .link_target
                .as_deref()
                .or_else(|| field.field_type.link_model_name())
                .unwrap_or("unknown");
            let table = table_for_model_name(schema, target, naming);
            let optional = if field.optional { "?" } else { "" };
            out.push(format!(
                "  {name}{optional}: RecordId<\"{table}\"> | string | {{ connect: {{ id: string }} }} | {{ create: {target}CreateScalars }};"
            ));
        }
    }
    emit_model_nested_write_bags(out, schema, model, naming, false);
    out.push("};".to_owned());
    out.push(String::new());

    out.push(format!("export type {}UpdateInput = {{", model.name));
    for field in model.fields.iter().filter(|f| !f.name.contains('.')) {
        if field.is_id || should_omit_on_record(field) {
            continue;
        }
        let name = naming.field_name(field);
        if field.is_stored_link() {
            let target = field
                .link_target
                .as_deref()
                .or_else(|| field.field_type.link_model_name())
                .unwrap_or("unknown");
            let table = table_for_model_name(schema, target, naming);
            if field.optional {
                out.push(format!(
                    "  {name}?: RecordId<\"{table}\"> | string | {{ connect: {{ id: string }} }} | {{ create: {target}CreateScalars }} | {{ disconnect: true }};"
                ));
            } else {
                out.push(format!(
                    "  {name}?: RecordId<\"{table}\"> | string | {{ connect: {{ id: string }} }} | {{ create: {target}CreateScalars }};"
                ));
            }
        } else {
            let ts = map_model_field_record(schema, model, field, naming);
            out.push(format!("  {name}?: {ts};"));
        }
    }
    emit_model_nested_write_bags(out, schema, model, naming, true);
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_model_nested_write_bags(
    out: &mut Vec<String>,
    schema: &DatabaseSchema,
    model: &Model,
    naming: &NamingContext<'_>,
    _for_update: bool,
) {
    for field in model.fields.iter().filter(|f| !f.name.contains('.')) {
        let name = naming.field_name(field);
        if let Some(relation) = &field.relation_name {
            let edge_name = resolve_edge_type_name(schema, relation);
            let Some(edge) = schema.edges.iter().find(|e| e.name == edge_name) else {
                continue;
            };
            let dir = if edge.in_model == model.name {
                "out"
            } else if edge.out_model == model.name {
                "in"
            } else {
                continue;
            };
            let far = dir;
            let far_model_name =
                if dir == "out" { edge.out_model.as_str() } else { edge.in_model.as_str() };
            let omit_keys = schema
                .models
                .iter()
                .find(|m| m.name == far_model_name)
                .map(|far_model| {
                    far_model
                        .fields
                        .iter()
                        .filter(|f| {
                            f.is_stored_link()
                                && f.link_target.as_deref() == Some(model.name.as_str())
                        })
                        .map(|f| format!("\"{}\"", naming.field_name(f)))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let far_create_ty = if omit_keys.is_empty() {
                format!("{far_model_name}CreateScalars")
            } else {
                format!("Omit<{far_model_name}CreateScalars, {}>", omit_keys.join(" | "))
            };
            let mut payload_fields = String::new();
            for ef in edge.fields.iter().filter(|f| !f.name.contains('.')) {
                let ef_name = naming.field_name(ef);
                let ts = map_scalar_or_object(None, &ef.field_type, ef, naming);
                let optional = if ef.optional { "?" } else { "" };
                let _ = write!(payload_fields, " {ef_name}{optional}: {ts};");
            }
            out.push(format!("  {name}?: {{"));
            out.push(format!(
                "    create?: Array<{{{payload_fields} {far}: string | {{ create: {far_create_ty} }}; }}>;"
            ));
            out.push(format!(
                "    connect?: Array<({{ {far}: string }} & {{{payload_fields}}}) | {{ id: string }}>;"
            ));
            out.push(format!("    disconnect?: Array<{{ {far}: string }} | {{ id: string }}>;"));
            out.push("  };".to_owned());
        } else if field.is_computed_link() {
            let target = field
                .link_target
                .as_deref()
                .or_else(|| field.field_type.link_model_name())
                .unwrap_or("unknown");
            let back = field.link_opposite_field.as_deref().unwrap_or("id");
            out.push(format!("  {name}?: {{"));
            out.push(format!("    create?: Array<Omit<{target}CreateScalars, \"{back}\">>;"));
            out.push("    connect?: Array<{ id: string }>;".to_owned());
            out.push("    disconnect?: Array<{ id: string }>;".to_owned());
            out.push("  };".to_owned());
        }
    }
}

fn emit_select_type_utils(out: &mut Vec<String>) {
    out.push(
        "/** `true` or nested `{ select, orderBy?, take?, skip? }` for a related entity. */"
            .to_owned(),
    );
    out.push("export type SelectArg<S, O = never> =".to_owned());
    out.push("  | boolean".to_owned());
    out.push("  | {".to_owned());
    out.push("      select?: S;".to_owned());
    out.push("      orderBy?: [O] extends [never] ? never : O | O[];".to_owned());
    out.push("      take?: number;".to_owned());
    out.push("      skip?: number;".to_owned());
    out.push("    };".to_owned());
    out.push(String::new());
    out.push(
        "/** Resolve one select entry: `true` → Default; `{ select: N }` → nested payload. */"
            .to_owned(),
    );
    out.push("export type ResolveSelectField<".to_owned());
    out.push("  V,".to_owned());
    out.push("  Default,".to_owned());
    out.push("  NestedSelect,".to_owned());
    out.push("  NestedPayload,".to_owned());
    out.push("> = V extends true".to_owned());
    out.push("  ? Default".to_owned());
    out.push("  : V extends { select?: NestedSelect }".to_owned());
    out.push("    ? NestedPayload".to_owned());
    out.push("    : never;".to_owned());
    out.push(String::new());
}

fn emit_where_shared_types(out: &mut Vec<String>) {
    out.push("export type StringFilter = {".to_owned());
    out.push("  equals?: string;".to_owned());
    out.push("  in?: string[];".to_owned());
    out.push("  contains?: string;".to_owned());
    out.push("  gt?: string;".to_owned());
    out.push("  gte?: string;".to_owned());
    out.push("  lt?: string;".to_owned());
    out.push("  lte?: string;".to_owned());
    out.push("};".to_owned());
    out.push(String::new());
    out.push("export type NumberFilter = {".to_owned());
    out.push("  equals?: number;".to_owned());
    out.push("  in?: number[];".to_owned());
    out.push("  gt?: number;".to_owned());
    out.push("  gte?: number;".to_owned());
    out.push("  lt?: number;".to_owned());
    out.push("  lte?: number;".to_owned());
    out.push("};".to_owned());
    out.push(String::new());
    out.push("export type AggregateFilter = {".to_owned());
    out.push("  equals?: number;".to_owned());
    out.push("  gt?: number;".to_owned());
    out.push("  gte?: number;".to_owned());
    out.push("  lt?: number;".to_owned());
    out.push("  lte?: number;".to_owned());
    out.push("};".to_owned());
    out.push(String::new());
    out.push("export type BooleanFilter = { equals?: boolean };".to_owned());
    out.push(String::new());
    out.push("export type DateFilter = {".to_owned());
    out.push("  equals?: Date | string;".to_owned());
    out.push("  in?: Array<Date | string>;".to_owned());
    out.push("  gt?: Date | string;".to_owned());
    out.push("  gte?: Date | string;".to_owned());
    out.push("  lt?: Date | string;".to_owned());
    out.push("  lte?: Date | string;".to_owned());
    out.push("};".to_owned());
    out.push(String::new());
    out.push("export type JsonFilter = { equals?: unknown };".to_owned());
    out.push(String::new());
    out.push("export type ArrayFilter = {".to_owned());
    out.push("  equals?: unknown[];".to_owned());
    out.push("  contains?: unknown;".to_owned());
    out.push("};".to_owned());
    out.push(String::new());
    out.push("export type IdFilter = { equals?: string; in?: string[] };".to_owned());
    out.push(String::new());
    out.push("export type ListRelationFilter<W> = {".to_owned());
    out.push("  some?: W;".to_owned());
    out.push("  every?: W;".to_owned());
    out.push("  none?: W;".to_owned());
    out.push("};".to_owned());
    out.push(String::new());
    out.push("export type RelationFilter<W> = {".to_owned());
    out.push("  is?: W;".to_owned());
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_model_where_input(
    out: &mut Vec<String>,
    schema: &DatabaseSchema,
    model: &Model,
    naming: &NamingContext<'_>,
) {
    out.push(format!("export type {}WhereInput = {{", model.name));
    out.push(format!("  AND?: {}WhereInput | {}WhereInput[];", model.name, model.name));
    out.push(format!("  OR?: {}WhereInput[];", model.name));
    out.push(format!("  NOT?: {}WhereInput | {}WhereInput[];", model.name, model.name));
    for field in model.fields.iter().filter(|f| !f.name.contains('.')) {
        let name = naming.field_name(field);
        if let Some(relation) = &field.relation_name {
            let edge = resolve_edge_type_name(schema, relation);
            out.push(format!(
                "  {name}?: {edge}WhereInput | ListRelationFilter<{edge}WhereInput>;"
            ));
        } else if field.is_link() {
            let target = field
                .link_target
                .as_deref()
                .or_else(|| field.field_type.link_model_name())
                .unwrap_or("unknown");
            if field.is_list_link() {
                out.push(format!(
                    "  {name}?: {target}WhereInput | ListRelationFilter<{target}WhereInput>;"
                ));
            } else {
                out.push(format!(
                    "  {name}?: {target}WhereInput | RelationFilter<{target}WhereInput>;"
                ));
            }
        } else if !should_omit_on_record(field) {
            let filter_ts = where_scalar_union(&field.field_type, field.is_id);
            out.push(format!("  {name}?: {filter_ts};"));
        }
    }
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_edge_where_input(
    out: &mut Vec<String>,
    _schema: &DatabaseSchema,
    edge: &Edge,
    naming: &NamingContext<'_>,
) {
    out.push(format!("export type {}WhereInput = {{", edge.name));
    out.push(format!("  AND?: {}WhereInput | {}WhereInput[];", edge.name, edge.name));
    out.push(format!("  OR?: {}WhereInput[];", edge.name));
    out.push(format!("  NOT?: {}WhereInput | {}WhereInput[];", edge.name, edge.name));
    out.push("  id?: string | IdFilter;".to_owned());
    out.push(format!(
        "  in?: {}WhereInput | RelationFilter<{}WhereInput>;",
        edge.in_model, edge.in_model
    ));
    out.push(format!(
        "  out?: {}WhereInput | RelationFilter<{}WhereInput>;",
        edge.out_model, edge.out_model
    ));
    for field in edge.fields.iter().filter(|f| !f.name.contains('.')) {
        let name = naming.field_name(field);
        let filter_ts = where_scalar_union(&field.field_type, false);
        out.push(format!("  {name}?: {filter_ts};"));
    }
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_model_where_unique_input(out: &mut Vec<String>, model: &Model, naming: &NamingContext<'_>) {
    out.push(format!("export type {}WhereUniqueInput = {{", model.name));
    for field in model.fields.iter().filter(|f| !f.name.contains('.')) {
        if field.is_link() || field.relation_name.is_some() {
            continue;
        }
        if field.is_id {
            let name = naming.field_name(field);
            out.push(format!("  {name}?: string | IdFilter;"));
        } else if field.unique {
            let name = naming.field_name(field);
            let filter_ts = where_scalar_union(&field.field_type, false);
            out.push(format!("  {name}?: {filter_ts};"));
        }
    }
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_edge_where_unique_input(out: &mut Vec<String>, edge: &Edge, naming: &NamingContext<'_>) {
    out.push(format!("export type {}WhereUniqueInput = {{", edge.name));
    out.push("  id?: string | IdFilter;".to_owned());
    for field in edge.fields.iter().filter(|f| !f.name.contains('.') && f.unique) {
        let name = naming.field_name(field);
        let filter_ts = where_scalar_union(&field.field_type, false);
        out.push(format!("  {name}?: {filter_ts};"));
    }
    out.push("};".to_owned());
    out.push(String::new());
}

const fn scalar_filter_kind(field_type: &FieldType, is_id: bool) -> &'static str {
    if is_id {
        return "id";
    }
    match field_type {
        FieldType::String => "string",
        FieldType::Int | FieldType::Float => "number",
        FieldType::Bool => "boolean",
        FieldType::Datetime => "datetime",
        FieldType::Array(_) => "array",
        FieldType::RecordId(_) => "id",
        FieldType::Object | FieldType::Model(_) | FieldType::Custom(_) => "json",
    }
}

fn where_scalar_union(field_type: &FieldType, is_id: bool) -> String {
    match scalar_filter_kind(field_type, is_id) {
        "string" => "string | StringFilter".to_owned(),
        "number" => "number | NumberFilter".to_owned(),
        "boolean" => "boolean | BooleanFilter".to_owned(),
        "datetime" => "Date | string | DateFilter".to_owned(),
        "array" => "unknown[] | ArrayFilter".to_owned(),
        "id" => "string | IdFilter".to_owned(),
        _ => "unknown | JsonFilter".to_owned(),
    }
}

fn emit_order_by_shared_types(out: &mut Vec<String>) {
    out.push("export type SortOrder = \"asc\" | \"desc\";".to_owned());
    out.push(String::new());
    out.push("/** Order parent rows by related list size (links / edges). */".to_owned());
    out.push("export type OrderByRelationCount = { _count?: SortOrder };".to_owned());
    out.push(String::new());
}

fn emit_model_order_by_input(out: &mut Vec<String>, model: &Model, naming: &NamingContext<'_>) {
    out.push(format!("export type {}OrderByInput = {{", model.name));
    for field in model.fields.iter().filter(|f| !f.name.contains('.')) {
        let name = naming.field_name(field);
        if field.relation_name.is_some() {
            let list = field.is_list_link() || matches!(field.field_type, FieldType::Array(_));
            if list {
                out.push(format!("  {name}?: OrderByRelationCount;"));
            }
            continue;
        }
        if field.is_link() {
            if field.is_list_link() {
                out.push(format!("  {name}?: OrderByRelationCount;"));
            }
            continue;
        }
        if should_omit_on_record(field) {
            continue;
        }
        out.push(format!("  {name}?: SortOrder;"));
    }
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_edge_order_by_input(out: &mut Vec<String>, edge: &Edge, naming: &NamingContext<'_>) {
    out.push(format!("export type {}OrderByInput = {{", edge.name));
    out.push("  id?: SortOrder;".to_owned());
    for field in edge.fields.iter().filter(|f| !f.name.contains('.')) {
        let name = naming.field_name(field);
        out.push(format!("  {name}?: SortOrder;"));
    }
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_model_aggregate_types(out: &mut Vec<String>, model: &Model, naming: &NamingContext<'_>) {
    let scalars = model_groupby_scalar_fields(model, naming);
    let numerics = model_groupby_numeric_fields(model, naming);
    emit_field_enum(out, &format!("{}ScalarFieldEnum", model.name), &scalars);
    emit_field_enum(out, &format!("{}NumericFieldEnum", model.name), &numerics);
    out.push(format!("export type {}GroupByOrderByInput = {{", model.name));
    for name in &scalars {
        out.push(format!("  {name}?: SortOrder;"));
    }
    out.push("  _count?: { _all?: SortOrder };".to_owned());
    out.push("};".to_owned());
    out.push(String::new());
    emit_model_having_input(out, model, naming);
}

fn emit_having_aggregate_fields(out: &mut Vec<String>, name: &str) {
    out.push(format!(
        "  _count?: {{ _all?: AggregateFilter }} & Partial<Record<{name}ScalarFieldEnum, AggregateFilter>>;"
    ));
    for kind in ["_sum", "_avg", "_min", "_max"] {
        out.push(format!("  {kind}?: Partial<Record<{name}NumericFieldEnum, AggregateFilter>>;"));
    }
}

fn emit_model_having_input(out: &mut Vec<String>, model: &Model, naming: &NamingContext<'_>) {
    out.push(format!("export type {}HavingInput = {{", model.name));
    for field in model.fields.iter().filter(|f| !f.name.contains('.')) {
        if field.is_link() || field.relation_name.is_some() || should_omit_on_record(field) {
            continue;
        }
        let name = naming.field_name(field);
        let filter_ts = where_scalar_union(&field.field_type, field.is_id);
        out.push(format!("  {name}?: {filter_ts};"));
    }
    emit_having_aggregate_fields(out, &model.name);
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_edge_aggregate_types(out: &mut Vec<String>, edge: &Edge, naming: &NamingContext<'_>) {
    let scalars = edge_groupby_scalar_fields(edge, naming);
    let numerics = edge_groupby_numeric_fields(edge, naming);
    emit_field_enum(out, &format!("{}ScalarFieldEnum", edge.name), &scalars);
    emit_field_enum(out, &format!("{}NumericFieldEnum", edge.name), &numerics);
    out.push(format!("export type {}GroupByOrderByInput = {{", edge.name));
    for name in &scalars {
        out.push(format!("  {name}?: SortOrder;"));
    }
    out.push("  _count?: { _all?: SortOrder };".to_owned());
    out.push("};".to_owned());
    out.push(String::new());
    out.push(format!("export type {}HavingInput = {{", edge.name));
    out.push("  id?: string | IdFilter;".to_owned());
    for field in edge.fields.iter().filter(|f| !f.name.contains('.')) {
        let name = naming.field_name(field);
        let filter_ts = where_scalar_union(&field.field_type, false);
        out.push(format!("  {name}?: {filter_ts};"));
    }
    emit_having_aggregate_fields(out, &edge.name);
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_field_enum(out: &mut Vec<String>, name: &str, fields: &[String]) {
    if fields.is_empty() {
        out.push(format!("export type {name} = never;"));
    } else {
        let union = fields.iter().map(|f| format!("\"{f}\"")).collect::<Vec<_>>().join(" | ");
        out.push(format!("export type {name} = {union};"));
    }
    out.push(String::new());
}

fn model_groupby_scalar_fields(model: &Model, naming: &NamingContext<'_>) -> Vec<String> {
    model
        .fields
        .iter()
        .filter(|f| !f.name.contains('.'))
        .filter(|f| !f.is_link() && f.relation_name.is_none() && !should_omit_on_record(f))
        .map(|f| naming.field_name(f))
        .collect()
}

fn model_groupby_numeric_fields(model: &Model, naming: &NamingContext<'_>) -> Vec<String> {
    model
        .fields
        .iter()
        .filter(|f| !f.name.contains('.'))
        .filter(|f| !f.is_link() && f.relation_name.is_none() && !should_omit_on_record(f))
        .filter(|f| matches!(f.field_type, FieldType::Int | FieldType::Float))
        .map(|f| naming.field_name(f))
        .collect()
}

fn edge_groupby_scalar_fields(edge: &Edge, naming: &NamingContext<'_>) -> Vec<String> {
    let mut fields = vec!["id".to_owned()];
    for field in edge.fields.iter().filter(|f| !f.name.contains('.')) {
        fields.push(naming.field_name(field));
    }
    fields
}

fn edge_groupby_numeric_fields(edge: &Edge, naming: &NamingContext<'_>) -> Vec<String> {
    edge.fields
        .iter()
        .filter(|f| !f.name.contains('.'))
        .filter(|f| matches!(f.field_type, FieldType::Int | FieldType::Float))
        .map(|f| naming.field_name(f))
        .collect()
}

fn emit_model_select_payload(
    out: &mut Vec<String>,
    schema: &DatabaseSchema,
    model: &Model,
    naming: &NamingContext<'_>,
) {
    let fields: Vec<&Field> = model
        .fields
        .iter()
        .filter(|field| !field.name.contains('.'))
        .filter(|field| {
            !should_omit_on_record(field) || field.is_link() || field.relation_name.is_some()
        })
        .collect();

    out.push(format!("export type {}Scalars = {{", model.name));
    for field in &fields {
        if field.is_link() || field.relation_name.is_some() {
            continue;
        }
        let name = naming.field_name(field);
        let ts = map_model_field_record(schema, model, field, naming);
        out.push(format!("  {name}: {ts};"));
    }
    out.push("};".to_owned());
    out.push(String::new());

    out.push(format!("export type {}Select = {{", model.name));
    for field in &fields {
        let name = naming.field_name(field);
        if field.is_link() || field.relation_name.is_some() {
            let target_select = relation_select_type_name(schema, field);
            let target_order = relation_order_by_type_name(schema, field);
            out.push(format!("  {name}?: SelectArg<{target_select}, {target_order}>;"));
        } else {
            out.push(format!("  {name}?: boolean;"));
        }
    }
    out.push("};".to_owned());
    out.push(String::new());

    for field in fields.iter().filter(|f| f.is_link() || f.relation_name.is_some()) {
        let name = naming.field_name(field);
        let alias = format!("{}{}Field", model.name, pascal_case(&name));
        let (default_ty, nested_select, list) = relation_payload_parts(schema, field);
        let default = if list { format!("{default_ty}[]") } else { default_ty.clone() };
        let nested_payload = if list {
            format!("{default_ty}GetPayload<N>[]")
        } else {
            format!("{default_ty}GetPayload<N>")
        };
        out.push(format!("type {alias}<V> = V extends true"));
        out.push(format!("  ? {default}"));
        out.push(format!(
            "  : V extends {{ select?: infer N extends {nested_select} | undefined }}"
        ));
        out.push(format!("    ? {nested_payload}"));
        out.push("    : never;".to_owned());
        out.push(String::new());
    }

    out.push(format!(
        "export type {}GetPayload<S extends {}Select | undefined = undefined> =",
        model.name, model.name
    ));
    out.push("  [S] extends [undefined]".to_owned());
    out.push(format!("    ? {}", model.name));
    out.push("    : {".to_owned());
    out.push("        [K in keyof S as S[K] extends false | undefined ? never : K]-?:".to_owned());
    out.push(format!("          K extends keyof {}Scalars", model.name));
    out.push(format!("            ? {}Scalars[K]", model.name));
    for field in fields.iter().filter(|f| f.is_link() || f.relation_name.is_some()) {
        let name = naming.field_name(field);
        let alias = format!("{}{}Field", model.name, pascal_case(&name));
        out.push(format!("            : K extends \"{name}\""));
        out.push(format!("              ? {alias}<S[K]>"));
    }
    out.push("            : never;".to_owned());
    out.push("      };".to_owned());
    out.push(String::new());
}

fn emit_edge_record(
    out: &mut Vec<String>,
    schema: &DatabaseSchema,
    edge: &Edge,
    naming: &NamingContext<'_>,
) {
    let in_table = table_for_model_name(schema, &edge.in_model, naming);
    let out_table = table_for_model_name(schema, &edge.out_model, naming);
    out.push(format!("export type {} = {{", edge.name));
    out.push(format!("  id: RecordId<\"{}\">;", edge.table_name(naming.convention())));
    out.push(format!("  in: RecordId<\"{in_table}\">;"));
    out.push(format!("  out: RecordId<\"{out_table}\">;"));
    for field in edge.fields.iter().filter(|f| !f.name.contains('.')) {
        let ts = map_scalar_or_object(None, &field.field_type, field, naming);
        let optional = if field.optional { "?" } else { "" };
        let name = naming.field_name(field);
        out.push(format!("  {name}{optional}: {ts};"));
    }
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_edge_selected(
    out: &mut Vec<String>,
    _schema: &DatabaseSchema,
    edge: &Edge,
    naming: &NamingContext<'_>,
) {
    out.push(format!("export type {}Selected = {{", edge.name));
    out.push(format!("  id: RecordId<\"{}\">;", edge.table_name(naming.convention())));
    out.push(format!("  in: {}Selected;", edge.in_model));
    out.push(format!("  out: {}Selected;", edge.out_model));
    for field in edge.fields.iter().filter(|f| !f.name.contains('.')) {
        let ts = map_scalar_or_object(None, &field.field_type, field, naming);
        let optional = if field.optional { "?" } else { "" };
        let name = naming.field_name(field);
        out.push(format!("  {name}{optional}: {ts};"));
    }
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_edge_inputs(
    out: &mut Vec<String>,
    schema: &DatabaseSchema,
    edge: &Edge,
    naming: &NamingContext<'_>,
) {
    let in_table = table_for_model_name(schema, &edge.in_model, naming);
    let out_table = table_for_model_name(schema, &edge.out_model, naming);
    out.push(format!("export type {}CreateInput = {{", edge.name));
    out.push(format!("  in: RecordId<\"{in_table}\"> | string;"));
    out.push(format!("  out: RecordId<\"{out_table}\"> | string;"));
    for field in edge.fields.iter().filter(|f| !f.name.contains('.')) {
        let ts = map_scalar_or_object(None, &field.field_type, field, naming);
        let optional = if field.optional { "?" } else { "" };
        let name = naming.field_name(field);
        out.push(format!("  {name}{optional}: {ts};"));
    }
    out.push("};".to_owned());
    out.push(String::new());

    out.push(format!("export type {}UpdateInput = {{", edge.name));
    out.push(format!("  in?: RecordId<\"{in_table}\"> | string;"));
    out.push(format!("  out?: RecordId<\"{out_table}\"> | string;"));
    for field in edge.fields.iter().filter(|f| !f.name.contains('.')) {
        let ts = map_scalar_or_object(None, &field.field_type, field, naming);
        let name = naming.field_name(field);
        out.push(format!("  {name}?: {ts};"));
    }
    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_edge_select_payload(
    out: &mut Vec<String>,
    schema: &DatabaseSchema,
    edge: &Edge,
    naming: &NamingContext<'_>,
) {
    let in_table = table_for_model_name(schema, &edge.in_model, naming);
    let out_table = table_for_model_name(schema, &edge.out_model, naming);

    out.push(format!("export type {}Scalars = {{", edge.name));
    out.push(format!("  id: RecordId<\"{}\">;", edge.table_name(naming.convention())));
    out.push(format!("  in: RecordId<\"{in_table}\">;"));
    out.push(format!("  out: RecordId<\"{out_table}\">;"));
    for field in edge.fields.iter().filter(|f| !f.name.contains('.')) {
        let name = naming.field_name(field);
        let ts = map_scalar_or_object(None, &field.field_type, field, naming);
        out.push(format!("  {name}: {ts};"));
    }
    out.push("};".to_owned());
    out.push(String::new());

    out.push(format!("export type {}Select = {{", edge.name));
    out.push("  id?: boolean;".to_owned());
    out.push(format!("  in?: SelectArg<{}Select, {}OrderByInput>;", edge.in_model, edge.in_model));
    out.push(format!(
        "  out?: SelectArg<{}Select, {}OrderByInput>;",
        edge.out_model, edge.out_model
    ));
    for field in edge.fields.iter().filter(|f| !f.name.contains('.')) {
        let name = naming.field_name(field);
        out.push(format!("  {name}?: boolean;"));
    }
    out.push("};".to_owned());
    out.push(String::new());

    out.push(format!("type {}InField<V> = V extends true", edge.name));
    out.push(format!("  ? {}", edge.in_model));
    out.push(format!(
        "  : V extends {{ select?: infer N extends {}Select | undefined }}",
        edge.in_model
    ));
    out.push(format!("    ? {}GetPayload<N>", edge.in_model));
    out.push("    : never;".to_owned());
    out.push(String::new());

    out.push(format!("type {}OutField<V> = V extends true", edge.name));
    out.push(format!("  ? {}", edge.out_model));
    out.push(format!(
        "  : V extends {{ select?: infer N extends {}Select | undefined }}",
        edge.out_model
    ));
    out.push(format!("    ? {}GetPayload<N>", edge.out_model));
    out.push("    : never;".to_owned());
    out.push(String::new());

    out.push(format!(
        "export type {}GetPayload<S extends {}Select | undefined = undefined> =",
        edge.name, edge.name
    ));
    out.push("  [S] extends [undefined]".to_owned());
    out.push(format!("    ? {}", edge.name));
    out.push("    : {".to_owned());
    out.push("        [K in keyof S as S[K] extends false | undefined ? never : K]-?:".to_owned());
    out.push(format!("          K extends \"in\" ? {}InField<S[K]>", edge.name));
    out.push(format!("          : K extends \"out\" ? {}OutField<S[K]>", edge.name));
    out.push(format!("          : K extends keyof {}Scalars ? {}Scalars[K]", edge.name, edge.name));
    out.push("          : never;".to_owned());
    out.push("      };".to_owned());
    out.push(String::new());
}

fn relation_select_type_name(schema: &DatabaseSchema, field: &Field) -> String {
    if let Some(relation) = &field.relation_name {
        return format!("{}Select", resolve_edge_type_name(schema, relation));
    }
    let target = field
        .link_target
        .as_deref()
        .or_else(|| field.field_type.link_model_name())
        .unwrap_or("unknown");
    format!("{target}Select")
}

fn relation_order_by_type_name(schema: &DatabaseSchema, field: &Field) -> String {
    if let Some(relation) = &field.relation_name {
        return format!("{}OrderByInput", resolve_edge_type_name(schema, relation));
    }
    let target = field
        .link_target
        .as_deref()
        .or_else(|| field.field_type.link_model_name())
        .unwrap_or("unknown");
    format!("{target}OrderByInput")
}

fn relation_payload_parts(schema: &DatabaseSchema, field: &Field) -> (String, String, bool) {
    let list = field.is_list_link() || matches!(field.field_type, FieldType::Array(_));
    if let Some(relation) = &field.relation_name {
        let edge = resolve_edge_type_name(schema, relation);
        return (edge.clone(), format!("{edge}Select"), list);
    }
    let target = field
        .link_target
        .as_deref()
        .or_else(|| field.field_type.link_model_name())
        .unwrap_or("unknown")
        .to_owned();
    (target.clone(), format!("{target}Select"), list)
}

fn pascal_case(name: &str) -> String {
    let mut chars = name.chars();
    chars
        .next()
        .map_or_else(String::new, |first| first.to_uppercase().collect::<String>() + chars.as_str())
}

fn should_omit_on_record(field: &Field) -> bool {
    field.is_computed_link() || field.relation_name.is_some()
}

fn map_model_field_record(
    schema: &DatabaseSchema,
    model: &Model,
    field: &Field,
    naming: &NamingContext<'_>,
) -> String {
    if field.is_id {
        if let Some(model_name) = field.field_type.link_model_name() {
            let table = table_for_model_name(schema, model_name, naming);
            return record_id_type(&table);
        }
        if let FieldType::RecordId(target) = &field.field_type {
            let table = table_for_model_name(schema, target, naming);
            return record_id_type(&table);
        }
    }

    if field.is_stored_link() {
        return map_link_as_record_id(schema, field, naming);
    }

    map_scalar_or_object(Some((schema, model)), &field.field_type, field, naming)
}

fn map_model_field_selected(
    schema: &DatabaseSchema,
    model: &Model,
    field: &Field,
    naming: &NamingContext<'_>,
) -> String {
    if field.is_id {
        return map_model_field_record(schema, model, field, naming);
    }

    if let Some(relation) = &field.relation_name {
        let edge_name = resolve_edge_type_name(schema, relation);
        let list = field.is_list_link() || matches!(field.field_type, FieldType::Array(_));
        return if list {
            format!("{edge_name}Selected[]")
        } else {
            format!("{edge_name}Selected")
        };
    }

    if field.is_link() {
        let target = field
            .link_target
            .as_deref()
            .or_else(|| field.field_type.link_model_name())
            .unwrap_or("unknown");
        let list = field.is_list_link();
        return if list { format!("{target}Selected[]") } else { format!("{target}Selected") };
    }

    map_scalar_or_object(Some((schema, model)), &field.field_type, field, naming)
}

fn map_link_as_record_id(
    schema: &DatabaseSchema,
    field: &Field,
    naming: &NamingContext<'_>,
) -> String {
    let target = field
        .link_target
        .as_deref()
        .or_else(|| field.field_type.link_model_name())
        .unwrap_or("unknown");
    let table = table_for_model_name(schema, target, naming);
    if field.is_list_link() {
        format!("{}[]", record_id_type(&table))
    } else {
        record_id_type(&table)
    }
}

fn map_scalar_or_object(
    ctx: Option<(&DatabaseSchema, &Model)>,
    field_type: &FieldType,
    field: &Field,
    naming: &NamingContext<'_>,
) -> String {
    match field_type {
        FieldType::Object => {
            if let Some((schema, model)) = ctx
                && let Some(name) = resolve_named_object_type(schema, model, field)
            {
                return name;
            }
            "Record<string, unknown>".to_owned()
        }
        FieldType::Model(name) if field.link_target.is_none() && field.relation_name.is_none() => {
            name.clone()
        }
        other => map_object_field_type(other, naming),
    }
}

fn resolve_named_object_type(
    schema: &DatabaseSchema,
    model: &Model,
    parent: &Field,
) -> Option<String> {
    let prefix = format!("{}.", parent.name);
    let nested: Vec<&Field> =
        model.fields.iter().filter(|field| field.name.starts_with(&prefix)).collect();

    schema.object_types.iter().find_map(|object_type| {
        if object_type.flexible != parent.flexible {
            return None;
        }
        if object_type.fields.len() != nested.len() {
            return None;
        }
        let matched = object_type.fields.iter().all(|candidate| {
            nested.iter().any(|field| {
                field.name == format!("{}.{}", parent.name, candidate.name)
                    && field.field_type == candidate.field_type
                    && field.optional == candidate.optional
            })
        });
        matched.then(|| object_type.name.clone())
    })
}

fn map_object_field_type(field_type: &FieldType, naming: &NamingContext<'_>) -> String {
    match field_type {
        FieldType::String => "string".to_owned(),
        FieldType::Int | FieldType::Float => "number".to_owned(),
        FieldType::Bool => "boolean".to_owned(),
        FieldType::Datetime => "Date".to_owned(),
        FieldType::Object => "Record<string, unknown>".to_owned(),
        FieldType::Array(inner) => format!("{}[]", map_object_field_type(inner, naming)),
        FieldType::RecordId(name) => {
            let table = naming.convention().tables.apply(name);
            record_id_type(&table)
        }
        FieldType::Model(name) => name.clone(),
        FieldType::Custom(value) => value.clone(),
    }
}

pub fn table_for_model_name(
    schema: &DatabaseSchema,
    model_name: &str,
    naming: &NamingContext<'_>,
) -> String {
    schema.models.iter().find(|model| model.name == model_name).map_or_else(
        || naming.convention().tables.apply(model_name),
        |model| model.table_name(naming.convention()),
    )
}

fn resolve_edge_type_name(schema: &DatabaseSchema, relation_name: &str) -> String {
    schema
        .edges
        .iter()
        .find(|edge| {
            edge.name == relation_name
                || edge.table_name(&schema.naming) == *relation_name
                || edge.attributes.get("map").is_some_and(|mapped| mapped == relation_name)
        })
        .map_or_else(|| relation_name.to_owned(), |edge| edge.name.clone())
}

fn emit_tables_const(out: &mut Vec<String>, schema: &DatabaseSchema, naming: &NamingContext<'_>) {
    out.push("export const Tables = {".to_owned());
    for model in &schema.models {
        out.push(format!("  {}: \"{}\",", model.name, model.table_name(naming.convention())));
    }
    for edge in &schema.edges {
        out.push(format!("  {}: \"{}\",", edge.name, edge.table_name(naming.convention())));
    }
    out.push("} as const;".to_owned());
    out.push(String::new());
}

fn emit_select_meta_registry(
    out: &mut Vec<String>,
    schema: &DatabaseSchema,
    naming: &NamingContext<'_>,
) {
    out.push("export type FieldSelectMeta =".to_owned());
    out.push(
        "  | { kind: \"scalar\"; filter: \"string\" | \"number\" | \"boolean\" | \"datetime\" | \"json\" | \"array\" | \"id\" }"
            .to_owned(),
    );
    out.push(
        "  | { kind: \"stored\" | \"computed\"; targetTable: string; list: boolean }".to_owned(),
    );
    out.push(
        "  | { kind: \"edge\"; edgeTable: string; dir: \"out\" | \"in\"; list: boolean };"
            .to_owned(),
    );
    out.push(String::new());
    out.push(
        "export const SelectMetaByTable: Record<string, Record<string, FieldSelectMeta>> = {"
            .to_owned(),
    );

    for model in &schema.models {
        let table = model.table_name(naming.convention());
        out.push(format!("  \"{table}\": {{"));
        for field in model.fields.iter().filter(|f| !f.name.contains('.')) {
            let name = naming.field_name(field);
            if let Some(relation) = &field.relation_name {
                let edge_name = resolve_edge_type_name(schema, relation);
                let Some(edge) = schema.edges.iter().find(|e| e.name == edge_name) else {
                    continue;
                };
                let edge_table = edge.table_name(naming.convention());
                let dir = if edge.in_model == model.name {
                    "out"
                } else if edge.out_model == model.name {
                    "in"
                } else {
                    continue;
                };
                let list = field.is_list_link() || matches!(field.field_type, FieldType::Array(_));
                out.push(format!(
                    "    {name}: {{ kind: \"edge\", edgeTable: \"{edge_table}\", dir: \"{dir}\", list: {list} }},"
                ));
            } else if field.is_link() {
                let target = field
                    .link_target
                    .as_deref()
                    .or_else(|| field.field_type.link_model_name())
                    .unwrap_or("unknown");
                let target_table = table_for_model_name(schema, target, naming);
                let kind = if field.is_computed_link() { "computed" } else { "stored" };
                let list = field.is_list_link();
                out.push(format!(
                    "    {name}: {{ kind: \"{kind}\", targetTable: \"{target_table}\", list: {list} }},"
                ));
            } else if !should_omit_on_record(field) {
                let filter = scalar_filter_kind(&field.field_type, field.is_id);
                out.push(format!("    {name}: {{ kind: \"scalar\", filter: \"{filter}\" }},"));
            }
        }
        out.push("  },".to_owned());
    }

    for edge in &schema.edges {
        let table = edge.table_name(naming.convention());
        let in_table = table_for_model_name(schema, &edge.in_model, naming);
        let out_table = table_for_model_name(schema, &edge.out_model, naming);
        out.push(format!("  \"{table}\": {{"));
        out.push("    id: { kind: \"scalar\", filter: \"id\" },".to_owned());
        out.push(format!(
            "    in: {{ kind: \"stored\", targetTable: \"{in_table}\", list: false }},"
        ));
        out.push(format!(
            "    out: {{ kind: \"stored\", targetTable: \"{out_table}\", list: false }},"
        ));
        for field in edge.fields.iter().filter(|f| !f.name.contains('.')) {
            let name = naming.field_name(field);
            let filter = scalar_filter_kind(&field.field_type, false);
            out.push(format!("    {name}: {{ kind: \"scalar\", filter: \"{filter}\" }},"));
        }
        out.push("  },".to_owned());
    }

    out.push("};".to_owned());
    out.push(String::new());
}

fn emit_write_meta_registry(
    out: &mut Vec<String>,
    schema: &DatabaseSchema,
    naming: &NamingContext<'_>,
) {
    out.push("export type FieldWriteMeta =".to_owned());
    out.push("  | { kind: \"scalar\" }".to_owned());
    out.push(
        "  | { kind: \"stored\"; targetTable: string; list: boolean; optional: boolean }"
            .to_owned(),
    );
    out.push(
        "  | { kind: \"computed\"; targetTable: string; list: boolean; backLinkField: string; backLinkOptional: boolean }"
            .to_owned(),
    );
    out.push(
        "  | { kind: \"edge\"; edgeTable: string; dir: \"out\" | \"in\"; list: boolean; inTable: string; outTable: string; payloadFields: string[] };"
            .to_owned(),
    );
    out.push(String::new());
    out.push(
        "export const WriteMetaByTable: Record<string, Record<string, FieldWriteMeta>> = {"
            .to_owned(),
    );

    for model in &schema.models {
        let table = model.table_name(naming.convention());
        out.push(format!("  \"{table}\": {{"));
        for field in model.fields.iter().filter(|f| !f.name.contains('.')) {
            let name = naming.field_name(field);
            if let Some(relation) = &field.relation_name {
                let edge_name = resolve_edge_type_name(schema, relation);
                let Some(edge) = schema.edges.iter().find(|e| e.name == edge_name) else {
                    continue;
                };
                let edge_table = edge.table_name(naming.convention());
                let dir = if edge.in_model == model.name {
                    "out"
                } else if edge.out_model == model.name {
                    "in"
                } else {
                    continue;
                };
                let in_table = table_for_model_name(schema, &edge.in_model, naming);
                let out_table = table_for_model_name(schema, &edge.out_model, naming);
                let payload = edge
                    .fields
                    .iter()
                    .filter(|f| !f.name.contains('.'))
                    .map(|f| format!("\"{}\"", naming.field_name(f)))
                    .collect::<Vec<_>>()
                    .join(", ");
                let list = field.is_list_link() || matches!(field.field_type, FieldType::Array(_));
                out.push(format!(
                    "    {name}: {{ kind: \"edge\", edgeTable: \"{edge_table}\", dir: \"{dir}\", list: {list}, inTable: \"{in_table}\", outTable: \"{out_table}\", payloadFields: [{payload}] }},"
                ));
            } else if field.is_computed_link() {
                let target = field
                    .link_target
                    .as_deref()
                    .or_else(|| field.field_type.link_model_name())
                    .unwrap_or("unknown");
                let target_table = table_for_model_name(schema, target, naming);
                let back = field
                    .link_opposite_field
                    .clone()
                    .or_else(|| find_stored_opposite_field_name(schema, field, naming))
                    .unwrap_or_else(|| "id".to_owned());
                let back_optional = find_stored_opposite_optional(schema, field).unwrap_or(false);
                let list = field.is_list_link();
                out.push(format!(
                    "    {name}: {{ kind: \"computed\", targetTable: \"{target_table}\", list: {list}, backLinkField: \"{back}\", backLinkOptional: {back_optional} }},"
                ));
            } else if field.is_stored_link() {
                let target = field
                    .link_target
                    .as_deref()
                    .or_else(|| field.field_type.link_model_name())
                    .unwrap_or("unknown");
                let target_table = table_for_model_name(schema, target, naming);
                let list = field.is_list_link();
                out.push(format!(
                    "    {name}: {{ kind: \"stored\", targetTable: \"{target_table}\", list: {list}, optional: {} }},",
                    field.optional
                ));
            } else if !field.is_id {
                out.push(format!("    {name}: {{ kind: \"scalar\" }},"));
            }
        }
        out.push("  },".to_owned());
    }

    out.push("};".to_owned());
    out.push(String::new());
}

fn find_stored_opposite_field_name(
    schema: &DatabaseSchema,
    computed: &Field,
    naming: &NamingContext<'_>,
) -> Option<String> {
    let link_name = computed.link_name.as_deref()?;
    let target =
        computed.link_target.as_deref().or_else(|| computed.field_type.link_model_name())?;
    let target_model = schema.models.iter().find(|m| m.name == target)?;
    target_model
        .fields
        .iter()
        .find(|f| f.is_stored_link() && f.link_name.as_deref() == Some(link_name))
        .map(|f| naming.field_name(f))
}

fn find_stored_opposite_optional(schema: &DatabaseSchema, computed: &Field) -> Option<bool> {
    let link_name = computed.link_name.as_deref()?;
    let target =
        computed.link_target.as_deref().or_else(|| computed.field_type.link_model_name())?;
    let target_model = schema.models.iter().find(|m| m.name == target)?;
    target_model
        .fields
        .iter()
        .find(|f| f.is_stored_link() && f.link_name.as_deref() == Some(link_name))
        .map(|f| f.optional)
}

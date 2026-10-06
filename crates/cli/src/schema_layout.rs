pub const CONFIG_FILE: &str = "_config.awesome.schema";
pub const TABLES_DIR: &str = "tables";
pub const TABLE_SUFFIX: &str = ".awesome.schema";

pub fn split_schema_dir(schema_path: &str) -> String {
    let path = std::path::Path::new(schema_path);
    if path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("schema")) {
        path.parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("schema")
            .to_string_lossy()
            .into_owned()
    } else {
        schema_path.to_owned()
    }
}

pub fn table_fragment_path(dir: &str, table_name: &str) -> String {
    format!("{dir}/{TABLES_DIR}/{table_name}{TABLE_SUFFIX}")
}

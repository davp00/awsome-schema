use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use crate::domain::{DatabaseSchema, require_surreal_provider};
use crate::errors::DomainError;
use crate::ports::{FileSystemPort, SchemaRenderer, SchemaSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerateCodeTarget {
    Schema,
    Rust,
    TypeScript,
}

pub struct GenerateCodeInput {
    pub target: GenerateCodeTarget,
    /// When true, print the generated client and do not write `generator.output`.
    pub stdout: bool,
}

pub struct GenerateCodeOutput {
    pub content: String,
    pub target: GenerateCodeTarget,
    /// Path written for a Rust or TypeScript client when not using `--stdout`.
    pub written_path: Option<String>,
}

pub struct GenerateCodeUseCase {
    schema_source: Arc<dyn SchemaSource>,
    schema_renderer: Arc<dyn SchemaRenderer>,
    rust_generator: Arc<dyn CodeGeneratorPort>,
    typescript_generator: Arc<dyn CodeGeneratorPort>,
    filesystem: Arc<dyn FileSystemPort>,
    schema_path: String,
}

pub trait CodeGeneratorPort: Send + Sync {
    fn generate(&self, schema: &crate::domain::DatabaseSchema) -> Result<String, DomainError>;
}

impl GenerateCodeUseCase {
    pub fn new(
        schema_source: Arc<dyn SchemaSource>,
        schema_renderer: Arc<dyn SchemaRenderer>,
        rust_generator: Arc<dyn CodeGeneratorPort>,
        typescript_generator: Arc<dyn CodeGeneratorPort>,
        filesystem: Arc<dyn FileSystemPort>,
        schema_path: String,
    ) -> Self {
        Self {
            schema_source,
            schema_renderer,
            rust_generator,
            typescript_generator,
            filesystem,
            schema_path,
        }
    }

    pub fn execute(&self, port: GenerateCodeInput) -> Result<GenerateCodeOutput, DomainError> {
        let schema = self.schema_source.load_schema()?;

        let content = match port.target {
            GenerateCodeTarget::Schema => {
                require_surreal_provider(&schema.datasource.provider)?;
                self.schema_renderer.render_schema(&schema)?
            }
            GenerateCodeTarget::Rust => self.rust_generator.generate(&schema)?,
            GenerateCodeTarget::TypeScript => self.typescript_generator.generate(&schema)?,
        };

        let written_path = if port.stdout {
            None
        } else {
            match port.target {
                GenerateCodeTarget::TypeScript => {
                    let path = resolve_typescript_output_path(
                        &schema,
                        &self.schema_path,
                        &*self.filesystem,
                    )?;
                    write_generated_file(&*self.filesystem, &path, &content)?;
                    if let Some(marker) = client_package_json_path(&path) {
                        self.filesystem.write_string(&marker, CLIENT_PACKAGE_JSON)?;
                    }
                    Some(path)
                }
                GenerateCodeTarget::Rust => {
                    let path =
                        resolve_rust_output_path(&schema, &self.schema_path, &*self.filesystem)?;
                    write_generated_file(&*self.filesystem, &path, &content)?;
                    Some(path)
                }
                GenerateCodeTarget::Schema => None,
            }
        };

        Ok(GenerateCodeOutput { content, target: port.target, written_path })
    }
}

/// Resolve `generator { provider = "typescript", output = "…" }` to a `.ts` file path.
pub fn resolve_typescript_output_path(
    schema: &DatabaseSchema,
    schema_path: &str,
    filesystem: &dyn FileSystemPort,
) -> Result<String, DomainError> {
    let generator = schema
        .generators
        .iter()
        .find(|generator| generator.provider.eq_ignore_ascii_case("typescript"))
        .ok_or_else(|| {
            DomainError::CodegenError(
                "no generator with provider = \"typescript\" found; add one or pass --stdout"
                    .to_owned(),
            )
        })?;

    let output = generator.output.trim();
    if output.is_empty() {
        let node_modules = nearest_node_modules(schema_path, filesystem)?;
        let client = node_modules.join(".awesome-schema").join("client").join("index.ts");
        return Ok(normalize_relative_path(client));
    }

    let base = output_base(schema_path, filesystem);
    Ok(normalize_relative_path(file_from_output(&base, output, "ts", "index.ts")))
}

/// Resolve `generator { provider = "rust", output = "…" }` to a `.rs` file path.
pub fn resolve_rust_output_path(
    schema: &DatabaseSchema,
    schema_path: &str,
    filesystem: &dyn FileSystemPort,
) -> Result<String, DomainError> {
    let output = generator_output(schema, "rust")?;
    if output.is_empty() {
        return Err(DomainError::CodegenError(
            "rust generator output is empty; set output or pass --stdout".to_owned(),
        ));
    }
    let base = output_base(schema_path, filesystem);
    Ok(normalize_relative_path(file_from_output(&base, output, "rs", "client.rs")))
}

fn generator_output<'a>(
    schema: &'a DatabaseSchema,
    provider: &str,
) -> Result<&'a str, DomainError> {
    schema
        .generators
        .iter()
        .find(|generator| generator.provider.eq_ignore_ascii_case(provider))
        .map(|generator| generator.output.trim())
        .ok_or_else(|| {
            DomainError::CodegenError(format!(
                "no generator with provider = \"{provider}\" found; add one or pass --stdout"
            ))
        })
}

fn output_base(schema_path: &str, filesystem: &dyn FileSystemPort) -> PathBuf {
    if filesystem.is_directory(schema_path) {
        PathBuf::from(schema_path)
    } else {
        Path::new(schema_path).parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf)
    }
}

fn file_from_output(base: &Path, output: &str, extension: &str, directory_file: &str) -> PathBuf {
    let output_path = Path::new(output);
    let resolved =
        if output_path.is_absolute() { output_path.to_path_buf() } else { base.join(output_path) };
    let has_extension = resolved
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case(extension));
    if has_extension { resolved } else { resolved.join(directory_file) }
}

fn write_generated_file(
    filesystem: &dyn FileSystemPort,
    path: &str,
    content: &str,
) -> Result<(), DomainError> {
    if let Some(parent) = Path::new(path).parent() {
        let parent_str = parent.to_string_lossy();
        if !parent_str.is_empty() && parent_str != "." {
            filesystem.create_dir_all(&parent_str)?;
        }
    }
    filesystem.write_string(path, content)
}

const CLIENT_PACKAGE_JSON: &str = "\
{
  \"name\": \".awesome-schema\",
  \"type\": \"module\",
  \"exports\": {
    \"./client\": {
      \"types\": \"./client/index.ts\",
      \"require\": \"./client/index.cjs\",
      \"default\": \"./client/index.ts\"
    }
  }
}
";

/// `node_modules/.awesome-schema/package.json` when `client_index` is the default client file.
fn client_package_json_path(client_index: &str) -> Option<String> {
    let path = Path::new(client_index);
    let client_dir = path.parent()?;
    if client_dir.file_name()?.to_str()? != "client" {
        return None;
    }
    let root = client_dir.parent()?;
    if root.file_name()?.to_str()? != ".awesome-schema" {
        return None;
    }
    Some(root.join("package.json").to_string_lossy().into_owned())
}

fn nearest_node_modules(
    schema_path: &str,
    filesystem: &dyn FileSystemPort,
) -> Result<PathBuf, DomainError> {
    let mut dir = if filesystem.is_directory(schema_path) {
        PathBuf::from(schema_path)
    } else {
        Path::new(schema_path).parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf)
    };
    loop {
        let candidate = dir.join("node_modules");
        if filesystem.is_directory(&candidate.to_string_lossy()) {
            return Ok(candidate);
        }
        if !dir.pop() {
            break;
        }
    }
    Err(DomainError::CodegenError(
        "typescript generator has no output and no node_modules directory was found; set output or install dependencies"
            .to_owned(),
    ))
}

fn normalize_relative_path(path: PathBuf) -> String {
    if path.is_absolute() {
        return path.to_string_lossy().into_owned();
    }
    let mut parts: Vec<String> = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                parts.pop();
            }
            other => parts.push(other.as_os_str().to_string_lossy().into_owned()),
        }
    }
    if parts.is_empty() { "index.ts".to_owned() } else { parts.join("/") }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::domain::{Datasource, Generator, NamingConvention};

    fn schema_with_ts_output(output: &str) -> DatabaseSchema {
        DatabaseSchema {
            datasource: Datasource {
                provider: "surrealdb".to_owned(),
                url: None,
                namespace: None,
                database: None,
                extra: BTreeMap::new(),
            },
            naming: NamingConvention::default(),
            generators: vec![Generator {
                provider: "typescript".to_owned(),
                output: output.to_owned(),
                extra: BTreeMap::new(),
            }],
            object_types: Vec::new(),
            models: Vec::new(),
            edges: Vec::new(),
        }
    }

    struct DirAwareFs {
        dirs: Vec<String>,
    }

    impl FileSystemPort for DirAwareFs {
        fn read_to_string(&self, _path: &str) -> Result<String, DomainError> {
            Err(DomainError::SchemaReadFailed("n/a".to_owned()))
        }

        fn write_string(&self, _path: &str, _content: &str) -> Result<(), DomainError> {
            Ok(())
        }

        fn create_dir_all(&self, _path: &str) -> Result<(), DomainError> {
            Ok(())
        }

        fn exists(&self, _path: &str) -> bool {
            false
        }

        fn is_directory(&self, path: &str) -> bool {
            self.dirs.iter().any(|dir| dir == path)
        }

        fn list_dir(&self, _path: &str) -> Result<Vec<String>, DomainError> {
            Ok(Vec::new())
        }

        fn remove_file(&self, _path: &str) -> Result<(), DomainError> {
            Ok(())
        }
    }

    #[test]
    fn resolves_directory_output_to_index_ts() {
        let schema = schema_with_ts_output("./generated");
        let fs = DirAwareFs { dirs: Vec::new() };
        let path =
            resolve_typescript_output_path(&schema, "/proj/awesome.schema", &fs).expect("resolve");
        assert!(path.ends_with("generated/index.ts") || path.ends_with("generated\\index.ts"));
    }

    #[test]
    fn resolves_rust_directory_and_file_output() {
        let mut schema = schema_with_ts_output("./generated");
        schema.generators[0].provider = "rust".to_owned();
        let fs = DirAwareFs { dirs: Vec::new() };
        let path = resolve_rust_output_path(&schema, "/proj/awesome.schema", &fs).expect("dir");
        assert!(path.ends_with("generated/client.rs") || path.ends_with("generated\\client.rs"));

        schema.generators[0].output = "./gen/client.rs".to_owned();
        let path = resolve_rust_output_path(&schema, "/proj/awesome.schema", &fs).expect("file");
        assert!(path.ends_with("gen/client.rs") || path.ends_with("gen\\client.rs"));

        schema.generators.clear();
        let error =
            resolve_rust_output_path(&schema, "/proj/awesome.schema", &fs).expect_err("missing");
        assert!(error.to_string().contains("rust"));
    }

    #[test]
    fn resolves_ts_file_output() {
        let schema = schema_with_ts_output("./gen/client.ts");
        let fs = DirAwareFs { dirs: Vec::new() };
        let path =
            resolve_typescript_output_path(&schema, "/proj/awesome.schema", &fs).expect("resolve");
        assert!(path.ends_with("gen/client.ts") || path.ends_with("gen\\client.ts"));
    }

    #[test]
    fn resolves_empty_output_into_node_modules() {
        let schema = schema_with_ts_output("");
        let fs = DirAwareFs { dirs: vec!["/proj/node_modules".to_owned()] };
        let path =
            resolve_typescript_output_path(&schema, "/proj/awesome.schema", &fs).expect("resolve");
        assert!(
            path.ends_with("node_modules/.awesome-schema/client/index.ts")
                || path.ends_with("node_modules\\.awesome-schema\\client\\index.ts")
        );
        let marker = client_package_json_path(&path).expect("marker");
        assert!(
            marker.ends_with("node_modules/.awesome-schema/package.json")
                || marker.ends_with("node_modules\\.awesome-schema\\package.json")
        );
    }

    #[test]
    fn rejects_empty_output_without_node_modules() {
        let schema = schema_with_ts_output("  ");
        let fs = DirAwareFs { dirs: Vec::new() };
        let err =
            resolve_typescript_output_path(&schema, "/proj/awesome.schema", &fs).expect_err("err");
        assert!(matches!(err, DomainError::CodegenError(_)));
    }

    #[test]
    fn rejects_missing_typescript_generator() {
        let mut schema = schema_with_ts_output("./generated");
        schema.generators.clear();
        let fs = DirAwareFs { dirs: Vec::new() };
        let err = resolve_typescript_output_path(&schema, "awesome.schema", &fs).expect_err("err");
        assert!(matches!(err, DomainError::CodegenError(_)));
    }
}

use std::fs;
use std::path::PathBuf;

use codegen::CodeGenerator;

/// `include!` cannot carry the emitter's crate-level `#![allow]`.
fn strip_crate_allow(code: &str) -> String {
    let Some(start) = code.find("#![allow(") else {
        return code.to_owned();
    };
    let Some(relative_end) = code[start..].find(")]") else {
        return code.to_owned();
    };
    let mut stripped = String::new();
    stripped.push_str(&code[..start]);
    stripped.push_str(&code[start + relative_end + 2..]);
    stripped
}

fn main() {
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let schema_path = manifest.join("fixtures/client.schema");
    println!("cargo:rerun-if-changed={}", schema_path.display());

    let source = fs::read_to_string(&schema_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", schema_path.display()));
    let schema = parser::parse(&source).unwrap_or_else(|error| panic!("parse fixture: {error}"));
    let code = codegen_rust::RustGenerator::new()
        .generate(&schema)
        .unwrap_or_else(|error| panic!("generate rust client: {error}"));
    let code = strip_crate_allow(&code);

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    let destination = out_dir.join("client.rs");
    fs::write(&destination, code)
        .unwrap_or_else(|error| panic!("write {}: {error}", destination.display()));
}

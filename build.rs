use std::{env, fs, path::Path};
use syn::{Attribute, Item, Lit, Meta, Type};
fn generate_config_docs() {
    // Path to the Rust source file containing the struct
    let rust_file_path = "src/config.rs";

    // Read the Rust source file
    let src = fs::read_to_string(rust_file_path).expect("Failed to read Rust source file");

    // Parse the source file using syn
    let syntax = syn::parse_file(&src).expect("Failed to parse Rust source file");

    // Name of the struct to find
    let target_struct_name = "Config";

    // Find the struct item
    let my_struct = syntax
        .items
        .iter()
        .find_map(|item| {
            if let Item::Struct(s) = item {
                if s.ident == target_struct_name {
                    Some(s)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .expect(&format!("Struct '{}' not found", target_struct_name));

    // Extract fields, their types, and doc comments
    let mut output = String::new();

    for field in &my_struct.fields {
        // Field name (unwrap because struct fields must be named)
        let field_name = field.ident.as_ref().unwrap().to_string();

        // Field type, as string
        let field_type = type_to_string(&field.ty);

        // Extract doc comments for this field
        let doc_comment = extract_doc_comments(&field.attrs);

        output.push_str(&format!(
            "- ## {} `{}`\n{}\n\n",
            field_name, field_type, doc_comment
        ));
    }

    // Write output markdown file to OUT_DIR
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env var not set");
    let dest_path = Path::new(&manifest_dir).join("CONFIGURATION_KEYS.md");
    fs::write(&dest_path, output).expect("Failed to write markdown file");
}
fn main() {
    println!("cargo:rerun-if-changed=src/config.rs");

    generate_config_docs();
}

// Helper function to convert syn::Type to string
fn type_to_string(ty: &Type) -> String {
    use quote::ToTokens;
    ty.to_token_stream().to_string().replace(" ", "")
}

// Helper function to extract doc comments (///) from attributes
fn extract_doc_comments(attrs: &[Attribute]) -> String {
    let mut doc_lines = vec![];
    for attr in attrs {
        if attr.path.is_ident("doc") {
            if let Ok(Meta::NameValue(meta_name_value)) = attr.parse_meta() {
                if let Lit::Str(lit_str) = meta_name_value.lit {
                    doc_lines.push(lit_str.value().trim().to_string());
                }
            }
        }
    }
    doc_lines.join("\n")
}

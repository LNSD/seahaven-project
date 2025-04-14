/// The path to the setup spec file
const SETUP_SPEC_FILE_PATH: &str = "schemas/setup-spec.json";

/// The path to the generated types file
const GENERATED_TYPES_FILE_PATH: &str = "codegen/src/model/setup_spec.rs";

fn main() {
    // Generate the setup-spec.json Rust types
    {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .map(std::path::PathBuf::from)
            .expect("Failed to get CARGO_MANIFEST_DIR");
        let out_dir = std::env::var("OUT_DIR")
            .map(std::path::PathBuf::from)
            .expect("Failed to get OUT_DIR");

        let setup_spec_file_path = manifest_dir.join(SETUP_SPEC_FILE_PATH);
        let generated_types_file_path = out_dir.join(GENERATED_TYPES_FILE_PATH);

        // Read the spec file from the local filesystem
        let content =
            std::fs::read_to_string(&setup_spec_file_path).expect("Failed to read setup spec file");

        // Parse the JSON schema
        let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content)
            .expect("Failed to parse setup spec as JSON schema");

        // Generate Rust types using typify
        let mut type_space =
            typify::TypeSpace::new(typify::TypeSpaceSettings::default().with_struct_builder(true));
        type_space
            .add_root_schema(schema)
            .expect("Failed to add schema to type space");

        let contents = prettyplease::unparse(
            &syn::parse2::<syn::File>(type_space.to_stream())
                .expect("Failed to parse generated code"),
        );

        // Create the output directory if it doesn't exist
        if let Some(parent) = generated_types_file_path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create output directory");
        }

        // Write the generated types to the file
        std::fs::write(&generated_types_file_path, contents)
            .expect("Failed to write generated types to file");

        // Re-run the build if the setup-spec.json file changes
        println!("cargo:rerun-if-changed={}", setup_spec_file_path.display());
    }
}

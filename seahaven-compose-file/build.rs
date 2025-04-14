/// The URL to the compose spec file (main branch)
#[cfg(feature = "update-spec-file")]
const COMPOSE_SPEC_FILE_URL: &str =
    "https://github.com/compose-spec/compose-spec/raw/refs/heads/main/schema/compose-spec.json";

/// The path to the patches directory
#[cfg(feature = "update-spec-file")]
const COMPOSE_SPEC_PATCHES_DIR: &str = "schemas/patches";

/// The path to the compose spec file
const COMPOSE_SPEC_FILE_PATH: &str = "schemas/compose-spec.json";

/// The path to the generated types file
const GENERATED_TYPES_FILE_PATH: &str = "codegen/src/compose_spec.rs";

fn main() {
    #[cfg(feature = "update-spec-file")]
    {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .map(std::path::PathBuf::from)
            .expect("Failed to get CARGO_MANIFEST_DIR");

        let compose_spec_file_path = manifest_dir.join(COMPOSE_SPEC_FILE_PATH);
        let compose_spec_patches_dir = manifest_dir.join(COMPOSE_SPEC_PATCHES_DIR);

        // Download the spec file using reqwest
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(COMPOSE_SPEC_FILE_URL)
            .send()
            .expect("Failed to download compose spec");

        if !response.status().is_success() {
            panic!(
                "Failed to download compose-spec.json file: {}",
                response.status()
            );
        }

        let mut content = response
            .text()
            .expect("Failed to read compose spec response body");

        // Apply patches
        let mut patch_files = std::fs::read_dir(&compose_spec_patches_dir)
            .expect("Failed to read patches directory")
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.extension()?.to_str()? == "patch" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<std::path::PathBuf>>();

        patch_files.sort_by_key(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("")
                .to_owned()
        });

        for patch_file in patch_files {
            let patch_content = std::fs::read_to_string(&patch_file).unwrap_or_else(|err| {
                panic!(
                    "Failed to read patch file {}: {}",
                    patch_file.display(),
                    err
                )
            });
            let patch = diffy::Patch::from_str(&patch_content).unwrap_or_else(|err| {
                panic!("Failed to parse patch {}: {}", patch_file.display(), err)
            });
            content = diffy::apply(&content, &patch).unwrap_or_else(|err| {
                panic!("Failed to apply patch {}: {}", patch_file.display(), err)
            });
        }

        // Write the final content back to the file
        std::fs::write(&compose_spec_file_path, content)
            .expect("Failed to write compose-spec.json to file");

        println!(
            "cargo:warning=Downloaded latest version of compose-spec.json from GitHub and applied patches"
        );
    }

    // Generate the compose-spec.json Rust types
    {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .map(std::path::PathBuf::from)
            .expect("Failed to get CARGO_MANIFEST_DIR");
        let out_dir = std::env::var("OUT_DIR")
            .map(std::path::PathBuf::from)
            .expect("Failed to get OUT_DIR");

        let compose_spec_file_path = manifest_dir.join(COMPOSE_SPEC_FILE_PATH);
        let generated_types_file_path = out_dir.join(GENERATED_TYPES_FILE_PATH);

        // Read the spec file from the local filesystem
        let content = std::fs::read_to_string(&compose_spec_file_path)
            .expect("Failed to read compose spec file");

        // Parse the JSON schema
        let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content)
            .expect("Failed to parse compose spec as JSON schema");

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

        // Re-run the build if the compose-spec.json file changes
        println!(
            "cargo:rerun-if-changed={}",
            compose_spec_file_path.display()
        );
    }
}

/// The URL to the compose spec file (main branch)
#[cfg(feature = "update-spec-file")]
const COMPOSE_SPEC_FILE_URL: &str =
    "https://github.com/compose-spec/compose-spec/raw/refs/heads/main/schema/compose-spec.json";

/// The path to the patches directory
#[cfg(feature = "update-spec-file")]
const COMPOSE_SPEC_PATCHES_DIR: &str = "schemas/patches";

/// The path to the compose spec file
#[cfg(feature = "update-spec-file")]
const COMPOSE_SPEC_FILE_PATH: &str = "schemas/compose-spec.json";

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
}

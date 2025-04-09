use std::{env, fs, path::PathBuf};

/// The URL to the compose spec file (main branch)
const COMPOSE_SPEC_FILE_URL: &str =
    "https://github.com/compose-spec/compose-spec/raw/refs/heads/main/schema/compose-spec.json";

/// The path to the compose spec file
const COMPOSE_SPEC_FILE_PATH: &str = "schemas/compose-spec.json";

fn main() {
    // Check if update-spec-file feature is enabled
    if cfg!(feature = "update-spec-file") {
        let spec_path = {
            let manifest_dir = env::var("CARGO_MANIFEST_DIR")
                .map(PathBuf::from)
                .expect("Failed to get CARGO_MANIFEST_DIR");
            manifest_dir.join(COMPOSE_SPEC_FILE_PATH)
        };

        // Create schemas directory if it doesn't exist
        if let Some(parent) = spec_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create schemas directory");
        }

        // Download the spec using reqwest
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(COMPOSE_SPEC_FILE_URL)
            .send()
            .expect("Failed to download compose spec");

        if !response.status().is_success() {
            panic!(
                "Failed to download compose spec: HTTP request failed with status {}",
                response.status()
            );
        }

        let content = response
            .text()
            .expect("Failed to read compose spec response body");

        fs::write(spec_path, content).expect("Failed to write compose-spec.json to file");

        println!("cargo:warning=Downloaded latest version of compose-spec.json from GitHub");
    }
}

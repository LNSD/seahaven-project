use std::path::PathBuf;

use clap::ArgMatches;
use tokio::process::Command;

use crate::result::Result;

pub const CMD: &str = "init";

/// The template for the setup.yaml file
const SETUP_YAML_TEMPLATE: &str = indoc::indoc! {r#"
    ---
    # The port where the app will be listening
    APP_PORT=8080
    ---

    name: {name}
    services:
      app: # https://github.com/nginxinc/NGINX-Demos/tree/e76e10522f6de1df2ffeddb8ea6ac279cc81463d/nginx-hello
        image: nginxdemos/hello:plain-text
        ports: ["${APP_PORT}:80"]
"#};

/// The template for the justfile
const JUSTFILE_TEMPLATE: &str = indoc::indoc! {r#"
    # This justfile provides commands for interacting with the Seahaven setup.
    # For more information on just, see https://just.systems/man/en/

    # Default target - displays available commands and their descriptions
    default:
        @just --list

    # Send an HTTP request to the app, and print the response
    send-request:
        @curl http://localhost:${APP_PORT}

"#};

/// The template for the README.md file
const README_MD_TEMPLATE: &str = indoc::indoc! {r#"
    # {name}

    This is a Seahaven project:

    - The [`setup.yaml`](./setup.yaml) file describes the services that make up the app.
    - The [`justfile`](./justfile) describes various tasks for interacting with the app.

    ## Bring up the app

    Similar to `docker compose up`, pull the images, start the services, and detach.

    ```
    truman up --detach
    ```

    ## Bring down the app

    Similar to `docker compose down`, stop the services, and teardown the setup.

    ```
    truman down
    ```

    ## Run a task

    For example, send an HTTP request to the app, and print out the response:

    ```
    truman run send-request
    ```

"#};

/// Create the init command
pub fn cmd() -> clap::Command {
    clap::Command::new(CMD)
        .about("Initialize a new Seahaven project")
        .args([
            clap::arg!(--name <NAME> "Set the resulting package name, defaults to the directory name")
                .value_parser(clap::value_parser!(String)),
            clap::arg!(--vcs <VCS> "Initialize a version control repository")
                .default_value("git")
                .value_parser(clap::value_parser!(Vcs)),
            clap::arg!(--"dry-run" "Execute command in dry run mode")
                .action(clap::ArgAction::SetTrue),
            clap::arg!([PATH] "The path to initialize the project in")
                .default_value(".")
                .value_parser(clap::value_parser!(PathBuf)),
        ])
}

/// Run the init command
pub async fn run(matches: &ArgMatches) -> Result<()> {
    let target_dir = matches
        .get_one::<PathBuf>("PATH")
        .expect("Failed to get target directory");
    let vcs = matches
        .get_one::<Vcs>("vcs")
        .expect("Failed to get VCS type");
    let dry_run = matches.get_flag("dry-run");

    // Initialize VCS if requested
    if !dry_run {
        match vcs {
            Vcs::Git => {
                // If the directory already exists, and it's a git repository, error out
                if target_dir.join(".git").is_dir() {
                    return Err(anyhow::anyhow!(
                        "git repository already initialized in: {}",
                        target_dir.join(".git").display()
                    )
                    .into());
                }

                git_init(target_dir).await?;
            }
            Vcs::Hg => {
                // If the directory already exists, and it's a mercurial repository, error out
                if target_dir.join(".hg").is_dir() {
                    return Err(anyhow::anyhow!(
                        "mercurial repository already initialized in: {}",
                        target_dir.join(".hg").display()
                    )
                    .into());
                }

                hg_init(target_dir).await?;
            }
            Vcs::None => {
                // If the directory already exists, error out
                if target_dir.is_dir() {
                    return Err(anyhow::anyhow!(
                        "directory already exists: {}",
                        target_dir.display()
                    )
                    .into());
                }

                tracing::debug!(
                    "No VCS selected, creating directory: {}",
                    target_dir.display()
                );
                std::fs::create_dir_all(target_dir)
                    .map_err(|err| anyhow::anyhow!("directory creation failed: {}", err))?;
            }
        }
    }
    println!("Created directory: {}", target_dir.display());

    // Get the project name, defaulting to the directory name if not provided
    let name = match matches.get_one::<String>("name") {
        Some(name) => name.to_string(),
        None => target_dir
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Failed to get directory name"))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to convert directory name to string"))?
            .to_string(),
    };

    // Create setup.yaml
    let setup_yaml_path = target_dir.join("setup.yaml");
    if !dry_run {
        let setup_yaml_content = SETUP_YAML_TEMPLATE.replace("{name}", &name);

        tracing::debug!("Creating setup.yaml: {}", setup_yaml_path.display());
        std::fs::write(&setup_yaml_path, setup_yaml_content)
            .map_err(|err| anyhow::anyhow!("Failed to write setup.yaml: {}", err))?;
    }
    println!("Added setup.yaml: {}", setup_yaml_path.display());

    // Create justfile
    let justfile_path = target_dir.join("justfile");
    if !dry_run {
        tracing::debug!("Creating justfile: {}", justfile_path.display());
        std::fs::write(&justfile_path, JUSTFILE_TEMPLATE)
            .map_err(|err| anyhow::anyhow!("Failed to write justfile: {}", err))?;
    }
    println!("Added justfile: {}", justfile_path.display());

    // Create README.md
    let readme_path = target_dir.join("README.md");
    if !dry_run {
        let readme_content = README_MD_TEMPLATE.replace("{name}", &name);

        tracing::debug!("Creating README.md: {}", readme_path.display());
        std::fs::write(&readme_path, readme_content)
            .map_err(|err| anyhow::anyhow!("Failed to write README.md: {}", err))?;
    }
    println!("Added README.md: {}", readme_path.display());

    indoc::printdoc!(
        r#"
        ==================================================
        New Seahaven setup initialized: {}
        ===================================================
        "#,
        setup_yaml_path.display(),
    );

    Ok(())
}
/// Version control system to initialize
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum Vcs {
    /// Git version control
    #[default]
    Git,
    /// Mercurial version control
    Hg,
    /// No version control
    None,
}

async fn git_init(target_dir: &PathBuf) -> Result<()> {
    tracing::debug!("git init: {}", target_dir.display());

    let status = Command::new("git")
        .arg("init")
        .arg("--quiet")
        .arg(target_dir)
        .spawn()
        .map_err(|err| anyhow::anyhow!("git init failed: {}", err))?
        .wait()
        .await
        .map_err(|err| anyhow::anyhow!("git init failed: {}", err))?;
    if !status.success() {
        return Err(anyhow::anyhow!("git init failed: {}", status).into());
    }

    Ok(())
}
async fn hg_init(target_dir: &PathBuf) -> Result<()> {
    tracing::debug!("hg init: {}", target_dir.display());

    let status = Command::new("hg")
        .arg("init")
        .arg("--quiet")
        .arg(target_dir)
        .spawn()
        .map_err(|err| anyhow::anyhow!("hg init failed: {}", err))?
        .wait()
        .await
        .map_err(|err| anyhow::anyhow!("hg init failed: {}", err))?;
    if !status.success() {
        return Err(anyhow::anyhow!("hg init failed: {}", status).into());
    }

    Ok(())
}

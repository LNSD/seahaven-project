# Seahaven CLI

This meta crate contains the different CLI commands for the Seahaven project, providing a unified interface for managing the development environment and project lifecycle.

## Structure

The different CLIs can be found in the `src/bin` directory:

- `truman`: Local development environment management.

### Truman

The cornerstone CLI for the Seahaven project. Manages the local development environment setup.

Truman provides a set of commands to handle the complete development lifecycle:

- Project initialization and configuration
- Service orchestration and management
- Development environment setup and maintenance
- Build and deployment workflows

Built with Rust, Truman integrates with Docker for container management and Just for task automation, ensuring a consistent and reproducible development experience.

#### `cargo install`

The easiest way to install the Truman CLI is to use `cargo` to install the `sehaven-cli` package and then use the `truman` binary.

```bash
cargo install sehaven-cli --bin truman --git https://github.com/LNSD/seahaven-project.git --locked
```

#### Usage

```bash
truman --help
```

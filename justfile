# Seahaven Project Justfile
#
# This justfile provides commands for managing the Seahaven project, a workspace management
# tool for blockchain development environments. Seahaven helps streamline the setup and
# management of blockchain development workspaces with Docker-based components.

# Default target - displays available commands and their descriptions
default:
    @just --list

# Run all project tests
test:
    cargo nextest run

# Format all Rust code (requires nightly toolchain)
fmt:
    cargo +nightly fmt --all

# Check code for common issues using clippy
clippy:
    cargo clippy --tests -- -D warnings --force-warn dead_code --force-warn deprecated

# Clean build artifacts and temporary files
clean:
    cargo clean

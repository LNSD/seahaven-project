# Seahaven Project Justfile
#
# This justfile provides commands for managing the Seahaven project, a workspace management
# tool for blockchain development environments. Seahaven helps streamline the setup and
# management of blockchain development workspaces with Docker-based components.

# Default target - displays available commands and their descriptions
default:
    @just --list

# Run project tests
test *suites='all':
    #!/usr/bin/env sh
    if [ "{{suites}}" = "all" ]; then
        cargo nextest run --all-features
        cargo test --all-features --doc
    else
        for suite in {{suites}}; do
            case $suite in
                unit) cargo nextest run --all-features 'tests::' -- --skip 'tests::it_' ;;
                it-in-tree) cargo nextest run --all-features 'tests::it_' ;;
                it-public) cargo nextest run --all-features --test '*' ;;
                doc) cargo test --all-features --doc ;;
                *)
                    echo "Unknown test suite: $suite"
                    exit 1
                    ;;
            esac
        done
    fi

# Format all Rust code (requires nightly toolchain)
fmt:
    cargo +nightly fmt --all

# Check code for common issues using clippy
clippy:
    cargo clippy --tests -- -D warnings --force-warn dead_code --force-warn deprecated

# Clean build artifacts and temporary files
clean:
    cargo clean

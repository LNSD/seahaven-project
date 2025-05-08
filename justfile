# Seahaven Project Justfile
#
# This justfile provides commands for managing the Seahaven project, a workspace management
# tool for blockchain development environments. Seahaven helps streamline the setup and
# management of blockchain development workspaces with Docker-based components.


# Default target - displays available commands and their descriptions
default:
    @just --list

# Format all Rust code (requires nightly toolchain)
fmt:
    cargo +nightly fmt --all

# Check code for common issues using clippy
clippy:
    cargo clippy --tests -- -D warnings --force-warn dead_code --force-warn deprecated

# Run project tests
test *suites='all':
    #!/usr/bin/env sh
    set -e # Exit on error

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

# Clean build artifacts and temporary files
clean:
    cargo clean

# Install git hooks
install-git-hooks:
    #!/usr/bin/env bash
    set -e # Exit on error

    # Check if pre-commit is installed
    if ! command -v "pre-commit" &> /dev/null; then
        >&2 echo "=============================================================="
        >&2 echo "Required command 'pre-commit' not available ❌"
        >&2 echo ""
        >&2 echo "Please install pre-commit using your preferred package manager"
        >&2 echo "  pip install pre-commit"
        >&2 echo "  pacman -S pre-commit"
        >&2 echo "  apt-get install pre-commit"
        >&2 echo "  brew install pre-commit"
        >&2 echo "=============================================================="
        exit 1
    fi

    # Install the pre-commit hooks
    pre-commit install --config .github/pre-commit-config.yaml

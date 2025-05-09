---
title: Seahaven CLI Wrapper
name: CLI Wrapper
slug: 001-cli-wrapper
status: draft
tags: [cli, wrapper, tooling, xdg]
editor: "Lorenzo Delgado <lnsdev@proton.me>"
contributors: [ "Lorenzo Delgado <lnsdev@proton.me>" ]
---

## Abstract

This RFC proposes the implementation of a CLI wrapper system for the `truman` command-line tool used in the Seahaven project.

Inspired by Gradle's and Maven's wrappers, 
this wrapper enables reproducible tool execution by pinning a specific version of `truman` per project.

The design ensures automatic download and caching of the required binary 
and enforces XDG Base Directory Specification for user-level configurations and binary storage.

## Background/Rationale/Motivation

Seahaven projects rely on the `truman` CLI tool for various command-line operations.

However, version drift across contributors may lead to inconsistencies, 
unexpected behavior, and onboarding difficulties.

While other ecosystems (e.g., Java with Gradle, Rust with rustup) provide consistent tooling through version-pinning and wrapper scripts, 
Seahaven lacks such a mechanism.

The goal of this RFC is to introduce a standard mechanism for:

* Ensuring that all developers run the same `truman` version.
* Automating binary download and caching.
* Integrating with XDG-compatible paths to support user-specific cache and config separation.

## Design

The Seahaven wrapper system (`trumanw`) is a lightweight, portable tool that ensures consistent version management of the `truman` CLI across development environments. Drawing inspiration from Gradle's wrapper pattern, it addresses the common challenge of tool version drift among project contributors.

At its core, the wrapper is a POSIX-compliant shell script that serves as a proxy between the user and the actual `truman` binary. When invoked, it performs a series of deterministic operations to ensure the correct version of `truman` is available and executed.

The system follows several key design principles:

1. **Project-Version Coupling**: Each project defines its exact `truman` version, ensuring consistency across all developers and CI environments.

2. **XDG Compliance**: All configuration, cache, and data storage follows the XDG Base Directory Specification, respecting user-specific organization preferences.

3. **Configuration Hierarchy**: Settings are resolved from multiple sources with a clear precedence order:
   - Environment variables (highest priority)
   - Project-specific configuration
   - Global user configuration (lowest priority)

4. **On-Demand Provisioning**: The wrapper automatically downloads and caches binaries only when needed, minimizing network usage.

5. **Transparent Operation**: The wrapper acts as a seamless proxy, forwarding all arguments to the underlying binary without modification.

6. **Cross-Platform Support**: While the initial implementation focuses on Unix-compatible systems, the design accommodates future Windows support.

The system MUST operate without external dependencies beyond standard Unix tools (`curl`, `sha256sum`, and shell utilities), making it self-contained and reliable across diverse environments. This ensures that developers can clone a project and immediately use the tool chain without additional setup steps.

The wrapper can be generated and maintained using the `truman wrapper` command. This command provides a streamlined interface for creating and updating wrapper scripts with specific versions, ensuring proper configuration and permissions. Project maintainers SHOULD use this command rather than manually creating wrapper files to ensure consistency and proper implementation.

## Proposed Implementation

The `truman` wrapper system includes the following components:

### 1. Wrapper Script

The `trumanw` script is a version-pinned proxy that resolves, fetches, and executes the appropriate `truman` binary based on project configuration:

* **File**: `trumanw` - Script that MUST use POSIX-compliant shell for Unix systems to ensure cross-platform compatibility
* **Runtime Flow**:
  1. Reads global configuration from `$XDG_CONFIG_HOME/seahaven/truman.toml`.
  2. Reads project-specific configuration from `.seahaven/wrapper.toml`.
  3. Merges configurations, with project settings taking precedence over global settings.
  4. Checks for environment variables which MUST take precedence over all configuration files.
  5. Generates a SHA256 hash of the distribution URL to create a unique identifier.
  6. Validates the existence of a cached binary matching this hash.
  7. Automatically downloads and caches the binary if not found.
  8. Executes the appropriate binary with all passed arguments.

* **Implementation Details**:
  * MUST be implemented in POSIX-compliant shell, i.e. bash.
  * MUST perform binary downloads using `curl` with proper checksum validation
  * MUST calculate hashes using `sha256sum` or platform-equivalent tool
  * MUST detect platform specifics using `uname -s` and `uname -m` for appropriate binary selection
  * SHOULD implement robust error handling for network issues and permission problems
  * SHOULD provide clear feedback during download and execution

This design ensures reproducible execution while minimizing network requests through efficient caching.

### 2. Project Configuration File

The project-specific configuration defines version and distribution details for the `truman` binary:

* **File**: `.seahaven/wrapper.toml` - TOML-formatted configuration that MUST exist in each project

#### Parameters

| Parameter  | Description | Required? |
|:-----------|:------------|:---------:|
| `distUrl`  | Specifies the exact binary version URL | Yes |
| `distBase` | Defines the base storage location (typically `XDG_CACHE_HOME`) | Yes |
| `distPath` | Defines the relative path within the base location | Yes |

The wrapper script MUST support environment variable interpolation in configuration values.
The `XDG_CACHE_HOME`, `XDG_CONFIG_HOME` and `XDG_DATA_HOME` values are special cases, and they will be replaced with the value of the `XDG_CACHE_HOME`, `XDG_CONFIG_HOME` and `XDG_DATA_HOME` environment variables, if they are set; otherwise, they will be replaced with the fallback value specified in the XDG Base Directory Specification.
This enables flexible configuration while maintaining compatibility with XDG standards.

#### Example Configuration

```toml
[wrapper]
distUrl = "https://example.com/truman/releases/0.4.1/truman"
distBase = "XDG_CACHE_HOME"
distPath = "seahaven/wrapper/dists"
```

### 3. Per-user Global Configuration

The global user-level configuration provides system-wide defaults and authentication options:

* **File**: `$XDG_CONFIG_HOME/seahaven/truman.toml` - User preferences that MAY override certain wrapper script behaviors

#### Parameters

The `[defaults]` section defines system-wide storage paths and default behaviors.

| Parameter  | Description | Required? |
|:-----------|:------------|:---------:|
| `distBase` | Defines the base storage location (typically `XDG_CACHE_HOME`) | Yes |
| `distPath` | Defines the relative path within the base location | Yes |

The `[auth]` section defines authentication tokens for accessing private repositories.

| Parameter  | Description | Required? |
|:-----------|:------------|:---------:|
| `token` | Defines the authentication token | No |

The `XDG_CACHE_HOME`, `XDG_CONFIG_HOME` and `XDG_DATA_HOME` values are special cases, and they will be replaced with the value of the `XDG_CACHE_HOME`, `XDG_CONFIG_HOME` and `XDG_DATA_HOME` environment variables, if they are set; otherwise, they will be replaced with the fallback value specified in the XDG Base Directory Specification.

#### Example Configuration

```toml
[defaults]
distBase = "XDG_CACHE_HOME"
distPath = "seahaven/wrapper/dists"

[auth]
token = "optional-secret-token"
```

### 4. Cache Directory Layout

The binary caching architecture organizes versioned distributions with a Gradle-inspired approach:

* **Root Directory**: `$XDG_CACHE_HOME/seahaven/wrapper/dists/` - Follows XDG specification standards
* **Organization Strategy**:
  * Top-level directories follow the naming template: `seahaven-<version|git-sha256>-<profile>` where:
    * `<version>` is the semantic version (e.g., `0.2.0`)
    * `<git-sha256>` is the git SHA256 hash of the commit (e.g., `a5deb9f2ac3126a32477c7d6a5a64983`)
    * `<profile>` can be `complete` (complete distribution), `default` (standard tools), or `minimal` (core functionality only). Similar to rustup's profiles.
  * Within each version directory, a unique subfolder is created using the SHA256 hash of the distribution URL
  * The binary is stored with a consistent name within this hash directory
  * Complete path format: `~/.cache/seahaven/wrapper/dists/seahaven-0.2.0-all/<hash>/truman`
  * This structure enables parallel installation of multiple versions without conflicts

The directory structure looks like:

```
$XDG_CACHE_HOME/seahaven/wrapper/dists/
├── seahaven-0.1.0-minimal
│   └── a5deb9f2ac3126a32477c7d6a5a64983
│       └── truman
├── seahaven-0.2.0-complete
│   └── 8f9ed9c78acda4b516d12f62fesd0ae1
│       └── truman
└── seahaven-a5deb9f2ac3126a32477c7d6a5a64983-default
    └── 7bc8a57d346abcf8901be23ff24dcad3
        └── truman
```

This versioning approach ensures reliable version identification, isolation between releases, and compliance with XDG standards while maintaining efficient disk space usage.

### 5. CLI Sub-command: `truman wrapper`

The `truman wrapper` command is the primary interface for managing wrapper scripts in a project.
It follows a similar pattern to Gradle's wrapper command, 
providing a consistent way to generate and update wrapper scripts.

The command MUST have the wrapper script template embedded directly within the binary itself, without requiring any external template files. This ensures that the wrapper generation process is self-contained and can be executed without network access or additional dependencies.

The command generates two essential files for wrapper functionality:

  * `trumanw` wrapper script
  * `.seahaven/wrapper.toml` configuration file

The command supports a rich set of options to customize the wrapper generation process:

| Option                      | Type        | Description                                 |
|:----------------------------|:-----------:|:--------------------------------------------|
| `--version`                 | `<semver>`  | Specify the version of `truman` to use.     |
| `--distribution-url`        | `<url>`     | Override the default distribution URL.      |
| `--distribution-sha256-sum` | `<hash>`    | Specify the expected SHA256 checksum.       |
| `--output-script`           | `<path>`    | Custom path for the wrapper script          |
| `--config-dir`              | `<path>`    | Custom location for `.seahaven/` directory. |
| `--force`                   |             | Force overwrite of existing files           |
| `--network-timeout`         | `<seconds>` | Set network timeout for downloads.          |
| `--no-validate`             |             | Skip validation of the distribution URL.    |
| `--no-download`             |             | Skip downloading the distribution.          |

The wrapper always ensures XDG-compliant configuration for consistent behavior across systems.

The command implements several key behaviors to ensure reliable operation:

  * If no version is specified, fetches the latest available version.
  * Validates the distribution URL and checksum if provided.
  * Creates necessary directories if they don't exist.
  * Preserves existing configuration if `--force` is not specified.
  * Follows XDG Base Directory Specification for all paths.
  * Supports both HTTP and HTTPS distribution URLs.
  * Handles network errors gracefully with retries.
  * Provides progress feedback during downloads.
  * Downloads the binary to the cache directory.
  * Verifies downloaded binaries against checksums.
  * Updates wrapper script permissions to be executable.

The command MUST follow the same configuration resolution order as the wrapper script:
  1. Read global configuration from `$XDG_CONFIG_HOME/seahaven/truman.toml`.
  2. Read project-specific configuration from `.seahaven/wrapper.toml`. 
  3. Check environment variables for overrides.
  4. Apply command-line options, which MUST take the highest precedence.

This ensures consistency between the wrapper's behavior and the command-line tool, with the additional benefit that command-line options provide the most direct control during wrapper generation.

Here are some common usage patterns to help you get started:

  ```bash
  # Generate wrapper with latest version
  truman wrapper

  # Generate wrapper with specific version
  truman wrapper --version 0.4.1

  # Generate wrapper with custom distribution URL
  truman wrapper --distribution-url https://example.com/truman/0.4.1/truman

  # Generate wrapper with custom config directory
  truman wrapper --config-dir ./.custom-seahaven

  # Generate wrapper with verbose output
  truman wrapper --verbose

  # Update existing wrapper to new version
  truman wrapper --version 0.4.2 --force
  ```

### 6. Environment Variable Overrides

The following environment variables may be used to override configuration at runtime:

| Variable                  | Purpose                                                      | Example Value                        |
| ------------------------- | ------------------------------------------------------------ | ------------------------------------ |
| `TRUMAN_DISTRIBUTION_URL` | Override the URL to download the `truman` binary             | `https://example.com/truman/0.5.0`   |
| `TRUMAN_VERSION`          | Shorthand to construct `distributionUrl` dynamically         | `0.5.0`                              |
| `TRUMAN_SHA256`           | Expected SHA256 checksum of the binary                       | `abcd123...`                         |
| `TRUMAN_CONFIG_DIR`       | Override the location of `.seahaven/` (project-local config) | `./.custom-seahaven`                 |
| `TRUMAN_USER_CONFIG`      | Override path to the user config file                        | `$HOME/.config/seahaven/truman.toml` |
| `TRUMAN_CACHE_HOME`       | Override XDG cache location for storing downloaded binaries  | `/tmp/.truman/cache`                 |
| `TRUMAN_EXTRA_ARGS`       | Inject extra flags when invoking the resolved binary         | `--verbose --no-cache`               |
| `TRUMAN_DISABLE_DOWNLOAD` | Skip automatic downloading (fail if binary not cached)       | `1`                                  |
| `TRUMAN_WRAPPER_DEBUG`    | Enable verbose logging of wrapper execution and decisions    | `true`                               |

Environment variables always take precedence over values defined in either local or global configuration files.

#### Notes and Behaviors

The following notes clarify the normative requirements for environment variable handling and configuration precedence:

* If both a config file and an environment variable define a field, the **environment variable MUST take precedence**.
* `TRUMAN_VERSION` MAY be used to derive the URL dynamically if `TRUMAN_DISTRIBUTION_URL` is not specified.
* `TRUMAN_EXTRA_ARGS` MUST be parsed as a shell-quoted string and appended to the execution command.
* `TRUMAN_CACHE_HOME` and `TRUMAN_USER_CONFIG` SHOULD allow overriding standard XDG paths (useful for CI and sandboxing).
* `TRUMAN_DISABLE_DOWNLOAD` SHOULD be used to enforce offline or reproducible-only execution in strict environments.

### XDG Environment Variable Resolution

The wrapper implementation MUST follow XDG Base Directory Specification standards to ensure proper system integration:

| Variable | Resolution Strategy | Fallback |
|:---------|:-------------------|:---------|
| `XDG_CONFIG_HOME` | Check environment variable first | `~/.config/` |
| `XDG_CACHE_HOME` | Check environment variable first | `~/.cache/` |
| `XDG_DATA_HOME` | Check environment variable first | `~/.local/share/` |

**Implementation Requirements**:

* Implementations MUST check for environment variables before using fallback paths.
* Applications SHOULD NOT write directly to the user's home directory (`~`).
* Directory paths MUST be created if they don't exist before writing files.
* Error handling SHOULD gracefully manage permission issues in XDG locations.

This ensures compatibility with standard Linux distributions while respecting user customizations of XDG paths.

The implementation components described above work together to provide a seamless versioning experience for project contributors while maintaining compatibility with system standards.

## Alternatives Considered

* Relying on system-wide `truman` installations: rejected due to variability.
* Manual version management per developer: error-prone and inconsistent.

While these alternatives might seem simpler initially, they would lead to versioning problems over time and increase onboarding friction for new developers.

## Future Considerations

* Add Windows wrapper script (`trumanw.bat` or PowerShell).
* Plugin registry stored in `$XDG_DATA_HOME/seahaven/plugins/`.
* GPG or checksum verification enforcement.

These enhancements will be addressed in future updates once the core wrapper functionality is established and validated.

## Security/Privacy Considerations

* No sensitive data is collected or stored.
* Optional use of authentication tokens for private downloads.
* No new attack surface is introduced by the wrapper itself.

## Copyright

Copyright and related rights waived [via CC0](https://creativecommons.org/publicdomain/zero/1.0/).

## References

### Normative References

* \[RFC2119] Bradner, S., 
  "Key words for use in RFCs to Indicate Requirement Levels", 
  BCP 14, RFC 2119, 
  DOI 10.17487/RFC2119, 
  March 1997, 
  <https://www.rfc-editor.org/info/rfc2119>

### Informative References

* \[GRADLE-WRAPPER] Gradle Team, 
  "Gradle Wrapper Documentation", 
  <https://docs.gradle.org/current/userguide/gradle_wrapper.html>

* \[XDG-BASEDIR] freedesktop.org, 
  "XDG Base Directory Specification", 
  <https://specifications.freedesktop.org/basedir-spec/latest/>

#v0_2
# Notes
- Packages must be compatible with *testcontainer* modules for reusability.
- They are specified in `setup.yaml` under either `services` or `init` sections. The ~~`image`~~`package` key should point to the directory within the workspace directory or the git repo or registry URL.
-----
# `package.toml`

- Similar to `Cargo.toml`
- Define package type: `init-container` or `service` (?)
- Meta: `name`, `description`, `version`, `license` (?), `readme`
- Sources: `git submodule`, `git repo`, `tar.gz`
- Image (like in `docker-compose.yaml`)
- Configuration: template, script, etc.
- Multiple `Dockerfile` build targets : main, wrapper, wrapper-dev, etc.
# Packages <-> setup.yaml
Should packages align with the `setup.yaml` services/init entries?

The CLI is an opinionated wrapper around `docker compose` + `just` to facilitate the operation of the "local development" environments.
# TODO

#v0_2 
- [ ] Intercept any signal and forward it to the docker command
- [ ] Add a `--output` (a.k.a. `--progress`) global option with `plain`, `json` and `tty` (for TUI). This will require better docker command *stdio* handling 
- [ ] Add support for listing the Justfile tasks like: `truman run --list` and `truman run --summary`
- [ ] Group sub-commands by functionality in the help (e.g., compose commands must under the same section, init+setup+eject under another section, etc.)
- [ ] Refactor docker/just wrapper crates and remove resolve_exe fns. Move the logic to the truman CLI and resolve the ENV vars in the sub-commands.
- [ ] Turn the system `--check-deps` option into a sub-command `truman system check`.
- [ ] Refactor the command preparation to avoid code duplication.

#v0_3 
- [ ] TBD
# Notes
- The CLI environment variables must be passed down to the docker-compose commands.
- Version compatibility with docker-compose must be ensured at startup. The minimum version must be compatible with docker-buildkit.
- A new session ID must be generated for each CLI execution. The session ID must be a UUID?.

---
## CLI commands
#v0_1 
- `truman up`
- `truman down`
- `truman build`
- `truman pull`
#v0_2 
- `truman logs`
- `truman ps`
- `truman start`
- `truman stop`
- `truman restart`
- `truman new`

## The package management commands
#v0_2 
- `truman new` to bootstrap the creation of a package
- `truman package <sub-command>` to operate with packages
## The system sub-command
- #v0_1 Check for dependencies option
- #v0_2 Support `docker system prune`
- Add system info command showing information about the different components of the system (e.g., dependencies).
## Docker compose command wrapper
- #v0_1 Pass the files to the docker-compose command `--file` and `--env-file` options, the CLI must create temporary files that should be automatically deleted after the command execution. 
- Use `<(...)` and `=(...)` shell syntax to provide the docker-compose and the .env files to docker compose (see: https://superuser.com/questions/1059781/what-exactly-is-in-bash-and-in-zsh).
- The temporary files created by truman must be created under `/var/seahaven/<session-id>/` directory.
## Compatibility with docker-compose.yaml
#v0_1 
- Call `truman eject` the `setup.yaml` file to get the `docker-compose.yaml` and `.env` files.
- Call `truman setup env` to print the `.env` file contents to the console with all the interpolated values.
- Call `truman setup compose` to print the `docker-compose.yaml` file contents to the console with all the interpolated values.

## Project bootstrap
- #v0_1 `truman init`: bootstrap the project, `git init`, 
- #v0_3 Truman init should initialize the wrapper. Can be disabled with `--no-wrapper` option.
- Support overriding the docker-compose (and env file?) templates
## Wrapper
See [[Wrapper]]
#v0_3 
- Wrapper script management, `truman wrapper`, similar to `gradle wrapper` command
- Truman CLI wrapper command
## Task runner
See [[Tasks]]
#v0_1 
- `truman run <task-name>` for root file tasks
- ~~`truman run <service> <task-name>`  like `docked compose run` (https://docs.docker.com/reference/cli/docker/compose/run/)~~

#next 
- Add justfile tasks shell auto-completion
## Plugin system
#next 
- Add a cargo-like plugin system to support installing commands with the name `truman-<subcommand>`.
- "Official" plugins should live under the `seahaven-cli/src/bin` directory.
## Setup.yaml edit
#next
- `truman new` ([[Packages]])
- `truman add` ([[Packages]], from git URL, `rev`, `branch`; from `path`)
- `truman remove` ([[Packages]])
- See [[Packages]]
## Test
#next
- See [[Tests]]
- As a `truman`, Cargo-like, CLI plugin. Run a binary in the path named `truman-test` that provides the functionality.


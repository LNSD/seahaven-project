The CLI is an opinionated wrapper around `docker compose` + `just` to facilitate the operation of the "local development" environments.
# TODO

- [ ] Move the `--file` option out of the main command, e.g., `truman --file <file> up`
- [ ] Intercept any signal and forward it to the docker command
- [ ] Add a `--output` (a.k.a. `--progress`) global option with `plain`, `json` and `tty` (for TUI). This will require better docker command *stdio* handling 
- [ ] Refactor the command preparation to avoid code duplication.

# Notes
- The CLI environment variables must be passed down to the docker-compose commands.
- Version compatibility with docker-compose must be ensured at startup. The minimum version must be compatible with docker-buildkit.
- Call `truman setup eject` the `setup.yaml` file to get the `docker-compose.yaml` and `.env` files.
- Call `truman setup env` to print the `.env` file contents to the console with all the interpolated values.
- Call `truman setup compose` to print the `docker-compose.yaml` file contents to the console with all the interpolated values.
- Use `<(...)` and `=(...)` shell syntax to provide the docker-compose and the .env files to docker compose (see: https://superuser.com/questions/1059781/what-exactly-is-in-bash-and-in-zsh).
- Support overriding the docker-compose (and env file?) templates
- A new session ID must be generated for each CLI execution. The session ID must be a UUID?.
- To pass the files to the docker-compose command `--file` and `--env-file` options, the CLI must create temporary files that should be automatically deleted after the command execution. The temporary files must be created under `/var/seahaven/<session-id>/` directory.
- Docker compose wrapper commands:
	- `truman up`
	- `truman down`
	- `truman logs`
	- `truman ps`
	- `truman start`
	- `truman stop`
	- `truman restart`
	- `truman build`
- Justfile runner wrapper, `truman run`. [[#Justfile]]
- Wrapper script management, `truman wrapper`, similar to `gradle wrapper` command. [[#Wrapper]]
- `truman init`: bootstrap the project, `git init`, wrapper init

---
# Setup.yaml edit
#next
- `truman new` ([[Packages]])
- `truman add` ([[Packages]], from git URL, `rev`, `branch`; from `path`)
- `truman remove` ([[Packages]])
- See [[Packages]]
# Justfile
-  See [[Tasks]]
- `truman run <task-name>` for root file tasks
- `truman run <service> <task-name>`  like `docked compose run` (https://docs.docker.com/reference/cli/docker/compose/run/)
# Test
#next
- See [[Tests]]
- As a `truman`, Cargo-like, CLI plugin. Run a binary in the path named `truman-test` that provides the functionality.
# Wrapper
- See [[Wrapper]]
- Truman CLI wrapper command


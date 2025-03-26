,The CLI is an opinionated wrapper around `docker compose` + `just` to facilitate the operation of the "local development" environments.

- The CLI environment variables must be passed down to the docker-compose commands.
- Version compatibility with docker-compose must be ensured at startup. The minimum version must be compatible with docker-buildkit.
- Call `truman setup eject` the `setup.yaml` file to get the `docker-compose.yaml` and `.env` files.
- Call `truman setup env` to print the `.env` file contents to the console with all the interpolated values.
- Call `truman setup compose` to print the `docker-compose.yaml` file contents to the console with all the interpolated values.
- Use `<(...)` and `=(...)` shell syntax to provide the docker-compose and the .env files to docker compose (see: https://superuser.com/questions/1059781/what-exactly-is-in-bash-and-in-zsh).
- Support overriding the docker-compose (and env file?) templates
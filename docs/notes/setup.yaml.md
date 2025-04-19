# TODO

- [x] #v0_2 Support `.env` files in the project root. See [[#Front-matter env + `.env` file support]]
# Notes
- Equivalent to `docker-compose.yaml` file with the `.env` section (constants section).
- There are two type of containers: 
	- The [[docs-drafts/Init container|init-containers]]: ephemeral containers that run once and exit, and the final state in docker compose is `service_completed_successfully`. They are perfect for setup tasks, data loading, or any prerequisites that need to be satisfied before the main application starts.
	- The [[Services|services]]: containers that run indefinitely, and the final state in docker compose is `service_started` or `service_healthy`.
- The `setup.yaml` file is "transpiled" by the [[truman (CLI)]] into a `docker-compose.yaml` file and an `.env` file, which is then used by docker-compose to run the services.
- The resulting `.env` file must be made available to the services as a mounted file. {?}
-  [[Services]] can be started in two modes:
	- **wrapped:** The service is wrapped in a shell script that sets up the configuration file and then starts the service. The container image is pulled with a tag.
	- **wrapped-dev:** The service is built from source and started in development mode. The container image is built from the source code. It is still wrapped in a shell script that sets up the configuration file and then starts the service.
- [[Services]] and [[docs-drafts/Init container]]s must be specified to be considered by the [[truman (CLI)]].
- The `setup.yaml` file must be in the root of the project, or specified with the `--file` flag.
- Any path will be treated relative to `$PWD`, like docker compose.
- The `services` top-level map is required.
- The `init` top-level map is optional.
- #next Multiple setup files can be merged into a single setup file by specifying them in the CLI the same way multiple docker-compose files are merged in the docker compose CLI.
- #next Support `sops` encrypted `.env` files.
---
## Environment variables
- ~~The `[constants]` section, as [in meson](https://mesonbuild.com/Machine-files.html#constants), can be referenced in the `[services]` section, and they will be interpolated.~~
- *envfile* format:  https://www.dotenv.org/docs/security/env (spec?)
- https://github.com/lucagoslar/serde-envfile
- https://github.com/Roger/serde-with-expand-env ~~(crate: `shellexpand`)~~
## dependency graph
- A mermaid graph can be displayed for understanding the different services `depends_on` links
## tools
- Tool specification
- See [[Tools]]

----
# Challenges
## Front-matter env + `.env` file support
- The front-matter environment approach makes sense for a few variables. But, it doesn't scale well, so when there are many lines of variables, it is recommended to use an `.env` file.
- If one wants to encrypt the environment data (e.g., using `sops`) an `.env` file MUST be used.
----
> _Which environment info should prevail in case both are present? Should we merge both?_
#### a) Merge both
We could treat the front-matter as another file that can be passed to the `docker compose` CLI.

> Merge order in case of conflicts? Front-matter is the first, or the last in the file merge chain?

The front-matter can be used as a fast option to override `.env` values for debugging purposes. So front-matter file values should prevail over the `.env` file values.

> Does Just support multiple `.env` files?`

No, so we should merge all environment variables into a single `.env` file before passing them to Just.
#### ~~b) Only one format is accepted, the other ignored~~
It should print a warning.
#### ~~c) Error out~~
If both are present, an error should be raised.


## Format preserving setup.yaml modification
Serde's YAML parsing library does not allow implementing `cargo edit`-like commands. There is no well stablished format-preserving YAML parser in Rust. 

There is https://crates.io/crates/nondestructive but it is not as well battle-tested as toml-edit.
## ~~services/init key collision~~
- If we merge the `services` and the `init-containers` maps and a service and an init-comtainer share the same name, they will collide:
### ~~Option A~~
> ~~The init container will be renamed (re-keyed) as `init-<key>`. Then when added to the `depends_on` table must be specified with the `init-` prefix. Renaming services is not an option as they are used as DNS names.~~

Nothing prevents us from having a `service-a`, a `service-a` init container (re-keyed as `init-service-a`) and a `init-service-a` init container. Fixing this woud require:
- a) Rejecting the init containers with a key prefixed with `init-`.
- b) Rejecting if any pair of init containers, `init-<key>` and `<key>` when there is a service with `<key>` name.
Nothing prevents us from having a `service-a` and `init-service-a` services, and a `service-a` init container.
- a) Reject any init container named like a service.
- ... ☠️
### Option B
> Reject the setup description file with an error ❌
### ~~Option C~~
> ~~Prepend all the `init-containers` keys with a "reference" to the section, e.g., `#/init-containers/<key>`. This prefixed keys must be used in the `depends_on` section.~~

Namespace all `init-container` so they do not collide with the `services`. What are the allowed characters in the `services` keys? ☠️

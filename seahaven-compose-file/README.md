seahaven-compose-file
=====================

A Rust library for working with Docker Compose files,
providing functionality for parsing, validating, and manipulating Compose files
according to the [Compose specification](https://github.com/compose-spec/compose-spec/blob/main/spec.md).

## Update the Compose specification schema file

The library includes a build feature `update-spec-file` that allows you to update the [Compose specification schema file](https://github.com/compose-spec/compose-spec/blob/main/schema/compose-spec.json).
When enabled, the build script will download the latest version of the schema
from the official Compose specification repository.

To update the schema file, run:

```bash
cargo build -p seahaven-compose-file --features=update-spec-file
```

This will download the latest Compose specification schema
and save it to `schemas/compose-spec.json`.

## References

- [Compose Specification](https://github.com/compose-spec/compose-spec/blob/main/spec.md)
- [Compose Specification JSON Schema](https://github.com/compose-spec/compose-spec/blob/main/schema/compose-spec.json)
- [Docker Compose File Reference](https://docs.docker.com/reference/compose-file/)

#v0_1
# Notes
- Integrate `just` as the task runner: https://github.com/casey/just
- Use a `justfile` to define tasks, and inject in the environment the [[setup.yaml]] file.
- To run a task call `truman run <task-name>`.
- The `justfile` must be in the root of the project, or specified with the `--justfile` flag.
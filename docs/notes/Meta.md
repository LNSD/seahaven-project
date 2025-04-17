Project development specific challenges
# TODO

- [x] Use `cargo-nextest` (locally and in the CI). 
- [ ] Export `cargo-nextest` JUnit compatible XML to *Codecov*.
- [ ] Add cursor rules for:
	- Writing Rust tests (given/when/then and test case names)
- [ ] Project specific cursor rules
- [ ] Extract a common crate for the command wrappers (i.e., `seahaven-docker` and `seahaven-just`)
- [ ] Move all the command wrapper crates under some directory, e.g., cmd.
# Notes
- Any document or note marked with `#next` tag should be treated as an future idea. Something to turn into a *post-v1.0.0* RFC.
- Any line marked with `#v0_X` or `#vX_Y` indicates the target release where that functionality should be available.

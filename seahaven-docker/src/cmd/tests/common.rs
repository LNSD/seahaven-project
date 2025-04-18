//! Common utilities for tests

use crate::exe::{Executable, resolve};

/// The path to the `mocker.sh` fixture file
const MOCKER_SH_PATH: &str = {
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/cmd/tests/fixtures/mocker.sh"
    )
};

/// Get the test fixture executable
///
/// The executable is the `mocker.sh` fixture script, which is used to mock the Docker CLI.
pub fn fixture_exe() -> Executable {
    resolve(MOCKER_SH_PATH).expect("Failed to resolve the test fixture executable")
}

/// Parses the output of the fixture executable into a [`Vec`] of string slices
///
/// The `mocker.sh` fixture script outputs each argument on a new line in the order they were received.
pub fn parse_fixture_exe_output(output: &std::process::Output) -> Vec<&str> {
    let stdout =
        std::str::from_utf8(&output.stdout).expect("Failed to parse command output as UTF-8");
    stdout.lines().collect()
}

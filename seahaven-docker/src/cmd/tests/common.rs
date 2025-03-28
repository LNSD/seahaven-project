//! Common utilities for tests

/// The path to the `mocker.sh` fixture file
pub const MOCKER_SH_PATH: &str = {
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/cmd/tests/fixtures/mocker.sh"
    )
};

/// Parses the output of the `mocker.sh` fixture file into a Vec of string slices
///
/// The `mocker.sh` script outputs each argument on a new line in the order they were received.
pub fn parse_fixture_output(output: &std::process::Output) -> Vec<&str> {
    let stdout =
        std::str::from_utf8(&output.stdout).expect("Failed to parse command output as UTF-8");
    stdout.lines().collect()
}

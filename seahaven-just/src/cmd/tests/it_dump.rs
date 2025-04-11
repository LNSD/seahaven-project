use super::common::{fixture_exe, parse_fixture_exe_output};
use crate::cmd::{IntoCommand, JustCmd};

#[tokio::test]
async fn run_just_dump() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = JustCmd::with_executable(exe).dump().into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run just dump command");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["--dump"]);
}

#[tokio::test]
async fn run_just_dump_with_just_format() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = JustCmd::with_executable(exe)
        .dump()
        .with_dump_just_format()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run just dump command with just format option");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["--dump", "--format", "just"]);
}

#[tokio::test]
async fn run_just_dump_with_json_format() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = JustCmd::with_executable(exe)
        .dump()
        .with_dump_json_format()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run just dump command with json format option");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["--dump", "--format", "json"]);
}

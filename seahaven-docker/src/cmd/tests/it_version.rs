use super::common::{MOCKER_SH_PATH, parse_fixture_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_version() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .version()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker version");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["version"]);
}

#[tokio::test]
async fn run_docker_version_with_json_format() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .version()
        .with_json_format()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    // Parse the fixture output
    let output = res.expect("Failed to run docker version");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["version", "--format", "json"]);
}

#[tokio::test]
async fn run_docker_version_with_custom_format() {
    //* Given
    let fmt_str = "{{.Server.Version}}";

    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .version()
        .with_custom_format(fmt_str)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker version");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["version", "--format", fmt_str]);
}

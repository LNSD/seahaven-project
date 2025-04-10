use super::common::{fixture_exe, parse_fixture_exe_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_compose_down() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .down()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose down");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "down"]);
}

#[tokio::test]
async fn run_docker_compose_down_with_volumes() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .down()
        .with_volumes(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose down with volumes");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "down", "--volumes"]);
}

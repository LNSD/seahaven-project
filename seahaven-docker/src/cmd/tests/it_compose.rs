use super::common::{MOCKER_SH_PATH, parse_fixture_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_compose() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .compose()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["compose"]);
}

#[tokio::test]
async fn run_docker_compose_down() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .compose()
        .down()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose down");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["compose", "down"]);
}

#[tokio::test]
async fn run_docker_compose_down_with_volumes() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .compose()
        .down()
        .with_volumes()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose down with volumes");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["compose", "down", "--volumes"]);
}

#[tokio::test]
async fn run_docker_compose_pull() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .compose()
        .pull()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose pull");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["compose", "pull"]);
}

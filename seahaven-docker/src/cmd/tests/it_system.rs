use super::common::{MOCKER_SH_PATH, parse_fixture_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_system() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .system()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["system"]);
}

#[tokio::test]
async fn run_docker_system_info() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .system()
        .info()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system info");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["info", "system"]);
}

#[tokio::test]
async fn run_docker_system_info_with_json_format() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .system()
        .info()
        .with_json_format()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system info with json format");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["--format", "info", "json", "system"]);
}

#[tokio::test]
async fn run_docker_system_info_with_custom_format() {
    //* Given
    let fmt_str = "{{.ServerVersion}}";

    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .system()
        .info()
        .with_custom_format(fmt_str)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system info with custom format");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["--format", "info", "system", fmt_str]);
}

#[tokio::test]
async fn run_docker_system_prune() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .system()
        .prune()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system prune");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["prune", "system"]);
}

#[tokio::test]
async fn run_docker_system_prune_with_volumes() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .system()
        .prune()
        .with_volumes()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system prune with volumes");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["--volumes", "prune", "system"]);
}

#[tokio::test]
async fn run_docker_system_prune_with_all() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .system()
        .prune()
        .with_all()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system prune with all");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["--all", "prune", "system"]);
}

#[tokio::test]
async fn run_docker_system_prune_with_force() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .system()
        .prune()
        .with_force()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system prune with force");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["--force", "prune", "system"]);
}

#[tokio::test]
async fn run_docker_system_prune_with_all_options() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .system()
        .prune()
        .with_volumes()
        .with_all()
        .with_force()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system prune with all options");
    let args = parse_fixture_output(&output);

    // The mocker.sh script sorts all arguments alphabetically
    assert_eq!(args, ["--all", "--force", "--volumes", "prune", "system"]);
}

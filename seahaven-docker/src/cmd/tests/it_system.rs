use super::common::{fixture_exe, parse_fixture_exe_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_system() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe).system().into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["system"]);
}

#[tokio::test]
async fn run_docker_system_info() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .system()
        .info()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system info");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["system", "info"]);
}

#[tokio::test]
async fn run_docker_system_info_with_json_format() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .system()
        .info()
        .with_json_format()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system info with json format");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["system", "info", "--format", "json"]);
}

#[tokio::test]
async fn run_docker_system_info_with_custom_format() {
    //* Given
    let exe = fixture_exe();

    let fmt_str = "{{.ServerVersion}}";

    let mut cmd = DockerCmd::with_executable(exe)
        .system()
        .info()
        .with_custom_format(fmt_str)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system info with custom format");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["system", "info", "--format", fmt_str]);
}

#[tokio::test]
async fn run_docker_system_prune() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .system()
        .prune()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system prune");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["system", "prune"]);
}

#[tokio::test]
async fn run_docker_system_prune_with_volumes() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .system()
        .prune()
        .with_volumes()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system prune with volumes");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["system", "prune", "--volumes"]);
}

#[tokio::test]
async fn run_docker_system_prune_with_all() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .system()
        .prune()
        .with_all()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system prune with all");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["system", "prune", "--all"]);
}

#[tokio::test]
async fn run_docker_system_prune_with_force() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .system()
        .prune()
        .with_force()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker system prune with force");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["system", "prune", "--force"]);
}

#[tokio::test]
async fn run_docker_system_prune_with_all_options() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
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
    let args = parse_fixture_exe_output(&output);

    // The arguments will appear in the order they were added
    assert_eq!(args, ["system", "prune", "--volumes", "--all", "--force"]);
}

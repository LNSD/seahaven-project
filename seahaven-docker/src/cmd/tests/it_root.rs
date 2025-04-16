use super::common::{fixture_exe, parse_fixture_exe_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_root() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe).into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker root command");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["<no-args>"]);
}

#[tokio::test]
async fn run_docker_root_version() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe).get_version().into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker --version");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["--version"]);
}

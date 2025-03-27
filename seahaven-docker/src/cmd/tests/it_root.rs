use super::common::{MOCKER_SH_PATH, parse_fixture_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_root() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH).into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker root command");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["<no-args>"]);
}

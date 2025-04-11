use super::common::{fixture_exe, parse_fixture_exe_output};
use crate::cmd::{IntoCommand, JustCmd};

#[tokio::test]
async fn run_just_root() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = JustCmd::with_executable(exe).into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run just root command");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["<no-args>"]);
}

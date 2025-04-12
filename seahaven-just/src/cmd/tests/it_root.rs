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

#[tokio::test]
async fn run_just_root_with_justfile() {
    //* Given
    let exe = fixture_exe();
    let justfile = "test/justfile";

    let mut cmd = JustCmd::with_executable(exe)
        .with_justfile(justfile)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run just root command with justfile");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["--justfile", "test/justfile"]);
}

#[tokio::test]
async fn run_just_root_with_env_file() {
    //* Given
    let exe = fixture_exe();
    let env_file = "test/.env";

    let mut cmd = JustCmd::with_executable(exe)
        .with_env_file(env_file)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run just root command with env file");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["--dotenv-path", "test/.env"]);
}

#[tokio::test]
async fn run_just_root_with_dry_run() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = JustCmd::with_executable(exe)
        .with_dry_run(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run just root command with dry run");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["--dry-run"]);
}

#[tokio::test]
async fn run_just_root_with_dry_run_false() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = JustCmd::with_executable(exe)
        .with_dry_run(false)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run just root command with dry run false");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["<no-args>"]);
}

#[tokio::test]
async fn run_just_root_with_all_options() {
    //* Given
    let exe = fixture_exe();
    let justfile = "test/justfile";
    let env_file = "test/.env";

    let mut cmd = JustCmd::with_executable(exe)
        .with_justfile(justfile)
        .with_env_file(env_file)
        .with_dry_run(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run just root command with all options");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "--justfile",
            "test/justfile",
            "--dotenv-path",
            "test/.env",
            "--dry-run"
        ]
    );
}

#[tokio::test]
async fn run_just_root_with_working_directory() {
    //* Given
    let exe = fixture_exe();
    let workdir = "test/dir";

    let mut cmd = JustCmd::with_executable(exe)
        .with_justfile("test/justfile") // Working directory requires justfile to be set
        .with_working_directory(workdir)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run just root command with working directory");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "--justfile",
            "test/justfile",
            "--working-directory",
            "test/dir"
        ]
    );
}

#[tokio::test]
async fn run_just_root_with_args() {
    //* Given
    let exe = fixture_exe();
    let args = vec!["recipe1", "arg1", "arg2"];

    let mut cmd = JustCmd::with_executable(exe).with_args(args).into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run just root command with args");
    let cmd_args = parse_fixture_exe_output(&output);

    assert_eq!(cmd_args, ["recipe1", "arg1", "arg2"]);
}

#[tokio::test]
async fn run_just_root_with_default_recipe() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = JustCmd::with_executable(exe)
        .run_default_recipe()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run just root command with default recipe");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["<no-args>"]);
}

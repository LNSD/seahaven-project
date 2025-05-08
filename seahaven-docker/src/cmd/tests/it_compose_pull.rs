use super::common::{fixture_exe, parse_fixture_exe_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_compose() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe).compose().into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose"]);
}

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

#[tokio::test]
async fn run_docker_compose_pull() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .pull()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose pull");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "pull"]);
}

#[tokio::test]
async fn run_docker_compose_with_plain_progress() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_plain_progress()
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with plain progress");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "--progress", "plain", "up"]);
}

#[tokio::test]
async fn run_docker_compose_with_quiet_progress() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_quiet_progress()
        .pull()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with quiet progress");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "--progress", "quiet", "pull"]);
}

#[tokio::test]
async fn run_docker_compose_with_project_name() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_project_name("test-project")
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with project name");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "--project-name", "test-project", "up"]);
}

#[tokio::test]
async fn run_docker_compose_with_project_name_and_progress() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_project_name("test-project")
        .with_json_progress()
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with project name and progress");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "--project-name",
            "test-project",
            "--progress",
            "json",
            "up"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_with_progress_and_project_name() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_plain_progress()
        .with_project_name("test-project")
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with progress and project name");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "--project-name",
            "test-project",
            "--progress",
            "plain",
            "up"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_with_file() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_file("docker-compose.prod.yml")
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with file option");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "--file", "docker-compose.prod.yml", "up"]);
}

#[tokio::test]
async fn run_docker_compose_with_env_file() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_env_file(".env.prod")
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with env file");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "--env-file", ".env.prod", "up"]);
}

#[tokio::test]
async fn run_docker_compose_with_tty_progress() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_tty_progress()
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with tty progress");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "--progress", "tty", "up"]);
}

#[tokio::test]
async fn run_docker_compose_with_json_progress() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_json_progress()
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with json progress");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "--progress", "json", "up"]);
}

#[tokio::test]
async fn run_docker_compose_with_all_options() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_file("docker-compose.prod.yml")
        .with_env_file(".env.prod")
        .with_project_name("test-project")
        .with_quiet_progress()
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with all options");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "--project-name",
            "test-project",
            "--file",
            "docker-compose.prod.yml",
            "--env-file",
            ".env.prod",
            "--progress",
            "quiet",
            "up"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_with_file_and_env_file() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_file("docker-compose.dev.yml")
        .with_env_file(".env.dev")
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with file and env file");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "--file",
            "docker-compose.dev.yml",
            "--env-file",
            ".env.dev",
            "up"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_with_ansi_always() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_ansi_always()
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with ansi always");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "--ansi", "always", "up"]);
}

#[tokio::test]
async fn run_docker_compose_with_ansi_never() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_ansi_never()
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with ansi never");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "--ansi", "never", "up"]);
}

#[tokio::test]
async fn run_docker_compose_with_ansi_and_progress() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_ansi_always()
        .with_plain_progress()
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with ansi and progress");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        ["compose", "--progress", "plain", "--ansi", "always", "up"]
    );
}

#[tokio::test]
async fn run_docker_compose_with_ansi_and_project_name() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_ansi_never()
        .with_project_name("test-project")
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with ansi and project name");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "--project-name",
            "test-project",
            "--ansi",
            "never",
            "up"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_with_all_options_including_ansi() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .with_file("docker-compose.prod.yml")
        .with_env_file(".env.prod")
        .with_ansi_always()
        .with_project_name("test-project")
        .with_quiet_progress()
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose with all options including ansi");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "--project-name",
            "test-project",
            "--file",
            "docker-compose.prod.yml",
            "--env-file",
            ".env.prod",
            "--progress",
            "quiet",
            "--ansi",
            "always",
            "up"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_pull_with_dry_run() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .pull()
        .with_dry_run(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose pull with dry run");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "pull", "--dry-run"]);
}

#[tokio::test]
async fn run_docker_compose_pull_with_services() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .pull()
        .with_services(["service1", "service2"])
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose pull with services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "pull", "service1", "service2"]);
}

#[tokio::test]
async fn run_docker_compose_pull_with_single_service() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .pull()
        .with_service("service1")
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose pull with single service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "pull", "service1"]);
}

#[tokio::test]
async fn run_docker_compose_pull_with_empty_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["", "api-service", ""];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .pull()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose pull with empty service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "pull", "api-service"]);
}

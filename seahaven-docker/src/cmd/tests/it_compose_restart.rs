use super::common::{fixture_exe, parse_fixture_exe_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_compose_restart() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .restart()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose restart");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "restart"]);
}

#[tokio::test]
async fn run_docker_compose_restart_with_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["service1", "service2"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .restart()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose restart with services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "restart", "service1", "service2"]);
}

#[tokio::test]
async fn run_docker_compose_restart_with_single_service() {
    //* Given
    let exe = fixture_exe();

    let service = "api-service";

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .restart()
        .with_service(service)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose restart with a single service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "restart", "api-service"]);
}

#[tokio::test]
async fn run_docker_compose_restart_with_dry_run() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .restart()
        .with_dry_run(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose restart with dry run");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "restart", "--dry-run"]);
}

#[tokio::test]
async fn run_docker_compose_restart_with_no_deps() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .restart()
        .with_no_deps(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose restart with no deps");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "restart", "--no-deps"]);
}

#[tokio::test]
async fn run_docker_compose_restart_with_timeout() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .restart()
        .with_timeout(30)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose restart with timeout");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "restart", "--timeout", "30"]);
}

#[tokio::test]
async fn run_docker_compose_restart_with_all_options() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .restart()
        .with_dry_run(true)
        .with_no_deps(true)
        .with_timeout(30)
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose restart with all options");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "restart",
            "--dry-run",
            "--no-deps",
            "--timeout",
            "30",
            "api-service",
            "web-service"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_restart_with_empty_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["", "api-service", ""];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .restart()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose restart with empty service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "restart", "api-service"]);
}

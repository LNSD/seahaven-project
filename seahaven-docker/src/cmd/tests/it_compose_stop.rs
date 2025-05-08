use super::common::{fixture_exe, parse_fixture_exe_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_compose_stop() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .stop()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose stop");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "stop"]);
}

#[tokio::test]
async fn run_docker_compose_stop_with_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["service1", "service2"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .stop()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose stop with services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "stop", "service1", "service2"]);
}

#[tokio::test]
async fn run_docker_compose_stop_with_single_service() {
    //* Given
    let exe = fixture_exe();

    let service = "api-service";

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .stop()
        .with_service(service)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose stop with a single service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "stop", "api-service"]);
}

#[tokio::test]
async fn run_docker_compose_stop_with_dry_run() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .stop()
        .with_dry_run(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose stop with dry run");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "stop", "--dry-run"]);
}

#[tokio::test]
async fn run_docker_compose_stop_with_timeout() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .stop()
        .with_timeout(30)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose stop with timeout");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "stop", "--timeout", "30"]);
}

#[tokio::test]
async fn run_docker_compose_stop_with_all_options() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .stop()
        .with_dry_run(true)
        .with_timeout(30)
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose stop with all options");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "stop",
            "--dry-run",
            "--timeout",
            "30",
            "api-service",
            "web-service"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_stop_with_empty_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["", "api-service", ""];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .stop()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose stop with empty service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "stop", "api-service"]);
}

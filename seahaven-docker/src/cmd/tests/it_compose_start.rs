use super::common::{fixture_exe, parse_fixture_exe_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_compose_start() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .start()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose start");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "start"]);
}

#[tokio::test]
async fn run_docker_compose_start_with_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["service1", "service2"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .start()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose start with services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "start", "service1", "service2"]);
}

#[tokio::test]
async fn run_docker_compose_start_with_single_service() {
    //* Given
    let exe = fixture_exe();

    let service = "api-service";

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .start()
        .with_service(service)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose start with a single service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "start", "api-service"]);
}

#[tokio::test]
async fn run_docker_compose_start_with_dry_run() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .start()
        .with_dry_run(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose start with dry run");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "start", "--dry-run"]);
}

#[tokio::test]
async fn run_docker_compose_start_with_dry_run_and_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .start()
        .with_dry_run(true)
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose start with dry run and services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "start",
            "--dry-run",
            "api-service",
            "web-service"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_start_with_empty_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["", "api-service", ""];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .start()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose start with empty service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "start", "api-service"]);
}

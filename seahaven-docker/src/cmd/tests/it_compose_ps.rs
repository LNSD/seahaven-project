use super::common::{fixture_exe, parse_fixture_exe_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_compose_ps() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .ps()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose ps");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "ps"]);
}

#[tokio::test]
async fn run_docker_compose_ps_with_all() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .ps()
        .with_all(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose ps with all flag");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "ps", "--all"]);
}

#[tokio::test]
async fn run_docker_compose_ps_with_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["service1", "service2"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .ps()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose ps with services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "ps", "service1", "service2"]);
}

#[tokio::test]
async fn run_docker_compose_ps_with_single_service() {
    //* Given
    let exe = fixture_exe();

    let service = "api-service";

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .ps()
        .with_service(service)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose ps with a single service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "ps", "api-service"]);
}

#[tokio::test]
async fn run_docker_compose_ps_with_all_and_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .ps()
        .with_all(true)
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose ps with all flag and services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        ["compose", "ps", "--all", "api-service", "web-service"]
    );
}

#[tokio::test]
async fn run_docker_compose_ps_with_dry_run() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .ps()
        .with_dry_run(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose ps with dry-run flag");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "ps", "--dry-run"]);
}

#[tokio::test]
async fn run_docker_compose_ps_with_dry_run_and_all_options() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .ps()
        .with_dry_run(true)
        .with_all(true)
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose ps with dry-run and all other options");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "ps",
            "--dry-run",
            "--all",
            "api-service",
            "web-service"
        ]
    );
}

use super::common::{fixture_exe, parse_fixture_exe_output};
use crate::cmd::{DockerCmd, IntoCommand};

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
async fn run_docker_compose_down_with_dry_run() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .down()
        .with_dry_run(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose down with dry-run");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "down", "--dry-run"]);
}

#[tokio::test]
async fn run_docker_compose_down_with_single_service() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .down()
        .with_services(["api-service"])
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose down with single service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "down", "api-service"]);
}

#[tokio::test]
async fn run_docker_compose_down_with_multiple_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service", "db-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .down()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose down with multiple services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "down",
            "api-service",
            "web-service",
            "db-service"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_down_with_services_and_volumes() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .down()
        .with_services(services)
        .with_volumes(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose down with services and volumes");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        ["compose", "down", "--volumes", "api-service", "web-service"]
    );
}

#[tokio::test]
async fn run_docker_compose_down_with_services_and_dry_run() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .down()
        .with_services(services)
        .with_dry_run(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose down with services and dry run");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        ["compose", "down", "--dry-run", "api-service", "web-service"]
    );
}

#[tokio::test]
async fn run_docker_compose_down_with_all_options() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .down()
        .with_services(services)
        .with_volumes(true)
        .with_dry_run(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose down with all options");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "down",
            "--volumes",
            "--dry-run",
            "api-service",
            "web-service"
        ]
    );
}

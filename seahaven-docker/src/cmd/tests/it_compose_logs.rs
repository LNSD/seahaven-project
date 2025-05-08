use super::common::{fixture_exe, parse_fixture_exe_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_compose_logs() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .logs()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose logs");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "logs"]);
}

#[tokio::test]
async fn run_docker_compose_logs_with_follow() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .logs()
        .with_follow(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose logs with follow flag");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "logs", "--follow"]);
}

#[tokio::test]
async fn run_docker_compose_logs_with_timestamps() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .logs()
        .with_timestamps(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose logs with timestamps flag");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "logs", "--timestamps"]);
}

#[tokio::test]
async fn run_docker_compose_logs_with_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["service1", "service2"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .logs()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose logs with services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "logs", "service1", "service2"]);
}

#[tokio::test]
async fn run_docker_compose_logs_with_single_service() {
    //* Given
    let exe = fixture_exe();

    let service = "api-service";

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .logs()
        .with_service(service)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose logs with a single service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "logs", "api-service"]);
}

#[tokio::test]
async fn run_docker_compose_logs_with_follow_and_timestamps() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .logs()
        .with_follow(true)
        .with_timestamps(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose logs with follow and timestamps flags");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "logs", "--follow", "--timestamps"]);
}

#[tokio::test]
async fn run_docker_compose_logs_with_follow_and_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .logs()
        .with_follow(true)
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose logs with follow flag and services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        ["compose", "logs", "--follow", "api-service", "web-service"]
    );
}

#[tokio::test]
async fn run_docker_compose_logs_with_timestamps_and_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .logs()
        .with_timestamps(true)
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose logs with timestamps flag and services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "logs",
            "--timestamps",
            "api-service",
            "web-service"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_logs_with_all_options() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service", "db-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .logs()
        .with_follow(true)
        .with_timestamps(true)
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose logs with all options");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "logs",
            "--follow",
            "--timestamps",
            "api-service",
            "web-service",
            "db-service"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_logs_with_dry_run() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .logs()
        .with_dry_run(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose logs with dry-run flag");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "logs", "--dry-run"]);
}

#[tokio::test]
async fn run_docker_compose_logs_with_dry_run_and_all_options() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .logs()
        .with_dry_run(true)
        .with_follow(true)
        .with_timestamps(true)
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose logs with dry-run and all other options");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "logs",
            "--dry-run",
            "--follow",
            "--timestamps",
            "api-service",
            "web-service"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_logs_with_empty_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["", "api-service", ""];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .logs()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose logs with empty service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "logs", "api-service"]);
}

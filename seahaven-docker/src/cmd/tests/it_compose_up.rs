use super::common::{fixture_exe, parse_fixture_exe_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_compose_up() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .up()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose up");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "up"]);
}

#[tokio::test]
async fn run_docker_compose_up_detached() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .up()
        .with_detach(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose up with detached flag");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "up", "--detach"]);
}

#[tokio::test]
async fn run_docker_compose_up_with_build() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .up()
        .with_build(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose up with build flag");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "up", "--build"]);
}

#[tokio::test]
async fn run_docker_compose_up_with_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["service1", "service2"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .up()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose up with services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "up", "service1", "service2"]);
}

#[tokio::test]
async fn run_docker_compose_up_with_single_service() {
    //* Given
    let exe = fixture_exe();

    let service = "api-service";

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .up()
        .with_service(service)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose up with a single service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "up", "api-service"]);
}

#[tokio::test]
async fn run_docker_compose_up_detached_with_build() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .up()
        .with_build(true)
        .with_detach(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose up with build and detached flags");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "up", "--build", "--detach"]);
}

#[tokio::test]
async fn run_docker_compose_up_detached_with_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .up()
        .with_detach(true)
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose up with detached flag and services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        ["compose", "up", "--detach", "api-service", "web-service"]
    );
}

#[tokio::test]
async fn run_docker_compose_up_with_build_and_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .up()
        .with_build(true)
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose up with build flag and services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        ["compose", "up", "--build", "api-service", "web-service"]
    );
}

#[tokio::test]
async fn run_docker_compose_up_with_all_options() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service", "db-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .up()
        .with_build(true)
        .with_detach(true)
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose up with all options");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "up",
            "--build",
            "--detach",
            "api-service",
            "web-service",
            "db-service"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_up_with_build_and_single_service() {
    //* Given
    let exe = fixture_exe();

    let service = "api-service";

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .up()
        .with_build(true)
        .with_service(service)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose up with build flag and single service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "up", "--build", "api-service"]);
}

#[tokio::test]
async fn run_docker_compose_up_detached_with_single_service() {
    //* Given
    let exe = fixture_exe();

    let service = "api-service";

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .up()
        .with_detach(true)
        .with_service(service)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output =
        res.expect("Failed to run docker compose up with detached flag and single service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "up", "--detach", "api-service"]);
}

#[tokio::test]
async fn run_docker_compose_up_with_detached_build_and_single_service() {
    //* Given
    let exe = fixture_exe();

    let service = "api-service";

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .up()
        .with_build(true)
        .with_detach(true)
        .with_service(service)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose up with all options and single service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        ["compose", "up", "--build", "--detach", "api-service"]
    );
}

#[tokio::test]
async fn run_docker_compose_up_with_dry_run() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .up()
        .with_dry_run(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose up with dry-run flag");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "up", "--dry-run"]);
}

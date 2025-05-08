use super::common::{fixture_exe, parse_fixture_exe_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_compose_build() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .build()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "build"]);
}

#[tokio::test]
async fn run_docker_compose_build_with_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["service1", "service2"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .build()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build with services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "build", "service1", "service2"]);
}

#[tokio::test]
async fn run_docker_compose_build_with_build_arg() {
    //* Given
    let exe = fixture_exe();

    let build_arg_key = "FOO";
    let build_arg_value = "bar";

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .build()
        .with_build_arg(build_arg_key, build_arg_value)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build with build arg");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "build", "--build-arg", "FOO=bar"]);
}

#[tokio::test]
async fn run_docker_compose_build_with_multiple_build_args() {
    //* Given
    let exe = fixture_exe();

    let build_args = [("FOO", "bar"), ("BAZ", "qux")];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .build()
        .with_build_args(build_args)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build with multiple build args");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "build",
            "--build-arg",
            "FOO=bar",
            "--build-arg",
            "BAZ=qux"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_build_with_single_service_and_build_arg() {
    //* Given
    let exe = fixture_exe();

    let service = "api-service";
    let build_arg_key = "NODE_ENV";
    let build_arg_value = "production";

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .build()
        .with_build_arg(build_arg_key, build_arg_value)
        .with_service(service)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build with service and build arg");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "build",
            "--build-arg",
            "NODE_ENV=production",
            "api-service"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_build_with_multiple_services_and_build_args() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];
    let build_args = [("NODE_ENV", "production"), ("DEBUG", "false")];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .build()
        .with_build_args(build_args)
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output =
        res.expect("Failed to run docker compose build with multiple services and build args");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "build",
            "--build-arg",
            "NODE_ENV=production",
            "--build-arg",
            "DEBUG=false",
            "api-service",
            "web-service"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_build_with_dry_run() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .build()
        .with_dry_run(true)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build with dry run");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "build", "--dry-run"]);
}

#[tokio::test]
async fn run_docker_compose_build_with_default_ssh_auth() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .build()
        .with_default_ssh_auth()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build with default SSH auth");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "build", "--ssh", "default"]);
}

#[tokio::test]
async fn run_docker_compose_build_with_custom_ssh_auth() {
    //* Given
    let exe = fixture_exe();

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .build()
        .with_ssh_auth("id=staging")
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build with custom SSH auth");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "build", "--ssh", "id=staging"]);
}

#[tokio::test]
async fn run_docker_compose_build_with_ssh_auth_and_build_args() {
    //* Given
    let exe = fixture_exe();

    let build_args = [("NODE_ENV", "production"), ("DEBUG", "false")];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .build()
        .with_build_args(build_args)
        .with_ssh_auth("default")
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build with SSH auth and build args");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "build",
            "--ssh",
            "default",
            "--build-arg",
            "NODE_ENV=production",
            "--build-arg",
            "DEBUG=false",
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_build_with_ssh_auth_and_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["api-service", "web-service"];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .build()
        .with_ssh_auth("default")
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build with SSH auth and services");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(
        args,
        [
            "compose",
            "build",
            "--ssh",
            "default",
            "api-service",
            "web-service"
        ]
    );
}

#[tokio::test]
async fn run_docker_compose_build_with_empty_services() {
    //* Given
    let exe = fixture_exe();

    let services = ["", "api-service", ""];

    let mut cmd = DockerCmd::with_executable(exe)
        .compose()
        .build()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build with empty service");
    let args = parse_fixture_exe_output(&output);

    assert_eq!(args, ["compose", "build", "api-service"]);
}

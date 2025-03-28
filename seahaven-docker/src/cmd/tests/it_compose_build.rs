use super::common::{MOCKER_SH_PATH, parse_fixture_output};
use crate::cmd::{DockerCmd, IntoCommand};

#[tokio::test]
async fn run_docker_compose_build() {
    //* Given
    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .compose()
        .build()
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["compose", "build"]);
}

#[tokio::test]
async fn run_docker_compose_build_with_services() {
    //* Given
    let services = ["service1", "service2"];

    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .compose()
        .build()
        .with_services(services)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build with services");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["compose", "build", "service1", "service2"]);
}

#[tokio::test]
async fn run_docker_compose_build_with_build_arg() {
    //* Given
    let arg = "FOO=bar";

    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .compose()
        .build()
        .with_build_arg(arg)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build with build arg");
    let args = parse_fixture_output(&output);

    assert_eq!(args, ["compose", "build", "--build-arg", "FOO=bar"]);
}

#[tokio::test]
async fn run_docker_compose_build_with_multiple_build_args() {
    //* Given
    let build_args = ["FOO=bar", "BAZ=qux"];

    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .compose()
        .build()
        .with_build_args(build_args)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build with multiple build args");
    let args = parse_fixture_output(&output);

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
    let service = "api-service";
    let arg = "NODE_ENV=production";

    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
        .compose()
        .build()
        .with_build_arg(arg)
        .with_service(service)
        .into_command();

    //* When
    let res = cmd.kill_on_drop(true).output().await;

    //* Then
    let output = res.expect("Failed to run docker compose build with service and build arg");
    let args = parse_fixture_output(&output);

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
    let services = ["api-service", "web-service"];
    let build_args = ["NODE_ENV=production", "DEBUG=false"];

    let mut cmd = DockerCmd::with_test_executable(MOCKER_SH_PATH)
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
    let args = parse_fixture_output(&output);

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

use seahaven_docker::{
    cmd::{DockerCmd, IntoCommand},
    exe::{Executable, resolve_cli_executable},
};

#[test_with::no_env(CI)]
#[test]
fn resolve_invalid_executable() {
    //* Given
    let invalid_exe_name = "non_existent_docker_binary";

    //* When
    let invalid_result = Executable::resolve(invalid_exe_name);

    //* Then
    assert!(
        invalid_result.is_err(),
        "Should fail with non-existent binary"
    );
}

#[test_with::no_env(CI)]
#[test]
fn executable_display_and_debug() {
    //* Given
    let exe = resolve_cli_executable().expect("docker binary not found");

    //* When
    let display_str = format!("{}", exe);
    let debug_str = format!("{:?}", exe);

    //* Then
    assert!(display_str.contains("docker"));
    assert!(debug_str.contains("docker"));
}

#[test_with::no_env(CI)]
#[test]
fn docker_cmd_default() {
    //* Given
    // Resolve the docker executable
    let expected_exe = resolve_cli_executable().expect("docker CLI executable not found");

    //* When
    let command = DockerCmd::default().into_command();

    // Get the program path
    let prog_path = command.as_std().get_program();

    //* Then
    // Assert that the program path is a valid utf-8 string
    let prog_path_str = prog_path
        .to_str()
        .expect("Program path should be a valid utf-8 string");
    assert!(
        prog_path_str.contains("docker"),
        "Program path should contain 'docker'"
    );

    // Assert that the program path is the same as the resolved executable
    assert_eq!(prog_path, expected_exe.as_ref());
}

use seahaven_package_testdata::{KITCHEN_SINK, NAME_EMPTY, NAME_INVALID, NO_TARGETS};

#[test]
fn parse_kitchen_sink_toml() {
    //* Given
    let toml_str = KITCHEN_SINK;

    //* When
    let manifest =
        seahaven_package::manifest::from_str(toml_str).expect("Failed to deserialize manifest");

    //* Then
    assert_eq!(manifest.package.name, "kitchen-sink");
}

#[test]
fn parse_no_targets_toml() {
    //* Given
    // Invalid manifest, no services or init containers defined
    let toml_str = NO_TARGETS;

    //* When
    let err =
        seahaven_package::manifest::from_str(toml_str).expect_err("Failed to deserialize manifest");

    //* Then
    assert!(
        err.to_string()
            .contains("no service or init containers defined")
    );
}

#[test]
fn parse_name_empty_toml() {
    //* Given
    let toml_str = NAME_EMPTY;

    //* When
    let err =
        seahaven_package::manifest::from_str(toml_str).expect_err("Failed to deserialize manifest");

    //* Then
    assert!(err.to_string().contains("invalid name"));
}

#[test]
fn parse_name_invalid_toml() {
    //* Given
    let toml_str = NAME_INVALID;

    //* When
    let err =
        seahaven_package::manifest::from_str(toml_str).expect_err("Failed to deserialize manifest");

    //* Then
    assert!(err.to_string().contains("invalid name"));
}

#[test]
fn display_kitchen_sink_toml() {
    //* Given
    let toml_str = KITCHEN_SINK;
    let manifest =
        seahaven_package::manifest::from_str(toml_str).expect("Failed to deserialize manifest");

    //* When
    let serialized =
        seahaven_package::manifest::to_string(&manifest).expect("Failed to serialize manifest");

    //* Then
    assert!(!serialized.is_empty());
}

#[test]
fn display_pretty_kitchen_sink_toml() {
    //* Given
    let toml_str = KITCHEN_SINK;
    let manifest =
        seahaven_package::manifest::from_str(toml_str).expect("Failed to deserialize manifest");

    //* When
    let serialized = seahaven_package::manifest::to_pretty_string(&manifest)
        .expect("Failed to serialize manifest");

    //* Then
    assert!(!serialized.is_empty());
}

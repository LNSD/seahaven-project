/// Test vector: `setup-yaml/single_service_basic`
///
/// ```yaml
/// services:
///   app:
///     image: nginx:latest
///     ports:
///       - "8080:80"
/// ```
///
/// See file: `setup-yaml/single_service_basic.yaml`
pub const SINGLE_SERVICE_BASIC: &str = indoc::indoc! { r###"
  services:
    app:
      image: nginx:latest
      ports:
        - "8080:80""### };

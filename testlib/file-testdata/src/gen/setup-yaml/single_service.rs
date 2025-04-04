/// Test vector: `setup-yaml/single_service`
///
/// ```yaml
/// services:
///   app:
///     image: nginx:latest
///     ports:
///       - "8080:80"
///     healthcheck:
///       test: ["CMD", "curl", "-f", "http://localhost:80"]
///       interval: 30s
///       timeout: 10s
///       retries: 3
/// ```
///
/// See file: `setup-yaml/single_service.yaml`
pub const SINGLE_SERVICE: &str = indoc::indoc! { r###"
  services:
    app:
      image: nginx:latest
      ports:
        - "8080:80"
      healthcheck:
        test: ["CMD", "curl", "-f", "http://localhost:80"]
        interval: 30s
        timeout: 10s
        retries: 3"### };

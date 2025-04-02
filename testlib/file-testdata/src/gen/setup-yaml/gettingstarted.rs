/// Test vector: `setup-yaml/gettingstarted`
///
/// ```yaml
/// services:
///   web:
///     build: .
///     ports:
///     - 8000:5000
///   redis:
///     image: redis:alpine
/// ```
///
/// See file: `setup-yaml/gettingstarted.yaml`
pub const GETTINGSTARTED: &str = indoc::indoc! { r###"
  services:
    web:
      build: .
      ports:
      - 8000:5000
    redis:
      image: redis:alpine"### };

/// Test vector: `setup-yaml/no_services`
///
/// ```yaml
/// ---
/// # Basic environment variables
/// CHAIN_ID=1337
/// CHAIN_NAME="testnet"
/// APP_PORT=8080
/// ---
/// # No services
/// ```
///
/// See file: `setup-yaml/no_services.yaml`
pub const NO_SERVICES: &str = indoc::indoc! { r###"
  ---
  # Basic environment variables
  CHAIN_ID=1337
  CHAIN_NAME="testnet"
  APP_PORT=8080
  ---
  # No services"### };

#[doc = "Test vector: `seahaven-file/testdata/data/no_services`"]
#[doc = ""]
#[doc = "```yaml"]
#[doc = r##"---"##]
#[doc = r##"# Basic environment variables"##]
#[doc = r##"CHAIN_ID=1337"##]
#[doc = r##"CHAIN_NAME="testnet""##]
#[doc = r##"APP_PORT=8080"##]
#[doc = r##"---"##]
#[doc = r##"# No services"##]
#[doc = "```"]
#[doc = ""]
#[doc = "See file: `seahaven-file/testdata/data/no_services.yaml`"]
pub const NO_SERVICES: &str = indoc::indoc! { r###"
  ---
  # Basic environment variables
  CHAIN_ID=1337
  CHAIN_NAME="testnet"
  APP_PORT=8080
  ---
  # No services"### };

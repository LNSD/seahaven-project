#[doc = "Test vector: `seahaven-package/testdata/data/name_invalid`"]
#[doc = ""]
#[doc = "```toml"]
#[doc = r##"[package]"##]
#[doc = r##"name = "invalid/name""##]
#[doc = r##"description = "A package with an invalid name""##]
#[doc = r##""##]
#[doc = r##"# Service target settings"##]
#[doc = r##"[[service]]"##]
#[doc = r##"name = "chain""##]
#[doc = r##"image = "ghcr.io/foundry-rs/foundry:latest""##]
#[doc = r##""##]
#[doc = r##"[[service.defaults]]"##]
#[doc = r##"command = "anvil --host=0.0.0.0 --chain-id=${CHAIN_ID} --base-fee=0""##]
#[doc = r##"ports = [ "${CHAIN_RPC}:8545" ]"##]
#[doc = "```"]
#[doc = ""]
#[doc = "See file: `seahaven-package/testdata/data/name_invalid.toml`"]
pub const NAME_INVALID: &str = indoc::indoc! { r###"
  [package]
  name = "invalid/name"
  description = "A package with an invalid name"

  # Service target settings
  [[service]]
  name = "chain"
  image = "ghcr.io/foundry-rs/foundry:latest"

  [[service.defaults]]
  command = "anvil --host=0.0.0.0 --chain-id=${CHAIN_ID} --base-fee=0"
  ports = [ "${CHAIN_RPC}:8545" ]"### };

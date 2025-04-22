#[doc = "Test vector: `seahaven-package/testdata/data/name_empty`"]
#[doc = ""]
#[doc = "```toml"]
#[doc = r##"[package]"##]
#[doc = r##"name = """##]
#[doc = r##"description = "A package with an empty name""##]
#[doc = r##""##]
#[doc = r##"# Service target settings"##]
#[doc = r##"[[service]]"##]
#[doc = r##"name = "chain""##]
#[doc = r##"image = "ghcr.io/foundry-rs/foundry:latest""##]
#[doc = r##""##]
#[doc = r##"[[service.defaults]]"##]
#[doc = r##"command = "anvil --host=0.0.0.0 --chain-id=${CHAIN_ID} --base-fee=0""##]
#[doc = r##"ports = [ "${CHAIN_RPC}:8545" ]"##]
#[doc = r##"healthcheck = { interval = "1s", retries = 10, test = "cast block" }"##]
#[doc = "```"]
#[doc = ""]
#[doc = "See file: `seahaven-package/testdata/data/name_empty.toml`"]
pub const NAME_EMPTY: &str = indoc::indoc! { r###"
  [package]
  name = ""
  description = "A package with an empty name"

  # Service target settings
  [[service]]
  name = "chain"
  image = "ghcr.io/foundry-rs/foundry:latest"

  [[service.defaults]]
  command = "anvil --host=0.0.0.0 --chain-id=${CHAIN_ID} --base-fee=0"
  ports = [ "${CHAIN_RPC}:8545" ]
  healthcheck = { interval = "1s", retries = 10, test = "cast block" }"### };

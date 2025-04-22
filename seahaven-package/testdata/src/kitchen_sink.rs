#[doc = "Test vector: `seahaven-package/testdata/data/kitchen_sink`"]
#[doc = ""]
#[doc = "```toml"]
#[doc = r##"[package]"##]
#[doc = r##"# The name of the package (Required)"##]
#[doc = r##"name = "kitchen-sink""##]
#[doc = r##"# The version of the package (Optional)"##]
#[doc = r##"version = "0.1.0""##]
#[doc = r##"# The description of the package (Optional)"##]
#[doc = r##"description = "A package that does way too much""##]
#[doc = r##"# The readme file for the package (Optional)"##]
#[doc = r##"readme = "README.md""##]
#[doc = r##""##]
#[doc = r##""##]
#[doc = r##"# Service target settings. Zero or more are allowed per package."##]
#[doc = r##"[[service]]"##]
#[doc = r##"name = "chain""##]
#[doc = r##"image = "ghcr.io/foundry-rs/foundry:latest""##]
#[doc = r##""##]
#[doc = r##"[[service.defaults]]"##]
#[doc = r##"command = "anvil --host=0.0.0.0 --chain-id=${CHAIN_ID} --base-fee=0""##]
#[doc = r##"ports = [ "${CHAIN_RPC}:8545" ]"##]
#[doc = r##"healthcheck = { interval = "1s", retries = 10, test = "cast block" }"##]
#[doc = r##""##]
#[doc = r##""##]
#[doc = r##"# Init target settings. Zero or more are allowed per package."##]
#[doc = r##"[[init]]"##]
#[doc = r##"# The name of the init target (Required)"##]
#[doc = r##"name = "deploy-core-contracts""##]
#[doc = r##"# Clone the repo and checkout the branch"##]
#[doc = r##"source = { git = "https://github.com/LNSD/seahaven-project.git", rev = "d3af235" }"##]
#[doc = r##"# The build context for the Dockerfile"##]
#[doc = r##"context = "testlib/testdata/app-contracts""##]
#[doc = r##"# The name of the Dockerfile to use within the context"##]
#[doc = r##"dockerfile = "Dockerfile""##]
#[doc = r##""##]
#[doc = r##"# The service defaults for the 'deploy-core-contracts' init target"##]
#[doc = r##"[[init.defaults]]"##]
#[doc = r##"command = "cast deploy""##]
#[doc = r##""##]
#[doc = r##"[[init]]"##]
#[doc = r##"name = "deploy-app-contracts""##]
#[doc = r##"# The source for the init target, relative to the package root"##]
#[doc = r##"source = "./app-contracts""##]
#[doc = r##"# The name of the Dockerfile to use within the source"##]
#[doc = r##"dockerfile = "Dockerfile""##]
#[doc = r##"# The build args for the Dockerfile"##]
#[doc = r##"build_args = { CHAIN_ID = 1337 }"##]
#[doc = r##"# The build target for the Dockerfile"##]
#[doc = r##"target = "debug""##]
#[doc = r##""##]
#[doc = r##"# The service defaults for the 'deploy-app-contracts' init target"##]
#[doc = r##"[[init.defaults]]"##]
#[doc = r##"command = "cast deploy""##]
#[doc = "```"]
#[doc = ""]
#[doc = "See file: `seahaven-package/testdata/data/kitchen_sink.toml`"]
pub const KITCHEN_SINK: &str = indoc::indoc! { r###"
  [package]
  # The name of the package (Required)
  name = "kitchen-sink"
  # The version of the package (Optional)
  version = "0.1.0"
  # The description of the package (Optional)
  description = "A package that does way too much"
  # The readme file for the package (Optional)
  readme = "README.md"


  # Service target settings. Zero or more are allowed per package.
  [[service]]
  name = "chain"
  image = "ghcr.io/foundry-rs/foundry:latest"

  [[service.defaults]]
  command = "anvil --host=0.0.0.0 --chain-id=${CHAIN_ID} --base-fee=0"
  ports = [ "${CHAIN_RPC}:8545" ]
  healthcheck = { interval = "1s", retries = 10, test = "cast block" }


  # Init target settings. Zero or more are allowed per package.
  [[init]]
  # The name of the init target (Required)
  name = "deploy-core-contracts"
  # Clone the repo and checkout the branch
  source = { git = "https://github.com/LNSD/seahaven-project.git", rev = "d3af235" }
  # The build context for the Dockerfile
  context = "testlib/testdata/app-contracts"
  # The name of the Dockerfile to use within the context
  dockerfile = "Dockerfile"

  # The service defaults for the 'deploy-core-contracts' init target
  [[init.defaults]]
  command = "cast deploy"

  [[init]]
  name = "deploy-app-contracts"
  # The source for the init target, relative to the package root
  source = "./app-contracts"
  # The name of the Dockerfile to use within the source
  dockerfile = "Dockerfile"
  # The build args for the Dockerfile
  build_args = { CHAIN_ID = 1337 }
  # The build target for the Dockerfile
  target = "debug"

  # The service defaults for the 'deploy-app-contracts' init target
  [[init.defaults]]
  command = "cast deploy""### };

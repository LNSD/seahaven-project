#[doc = "Test vector: `seahaven-package/testdata/data/no_targets`"]
#[doc = ""]
#[doc = "```toml"]
#[doc = r##"[package]"##]
#[doc = r##"name = "no-targets""##]
#[doc = r##"description = "A package that does nothing""##]
#[doc = r##""##]
#[doc = r##"# No targets are defined"##]
#[doc = "```"]
#[doc = ""]
#[doc = "See file: `seahaven-package/testdata/data/no_targets.toml`"]
pub const NO_TARGETS: &str = indoc::indoc! { r###"
  [package]
  name = "no-targets"
  description = "A package that does nothing"

  # No targets are defined"### };

#[doc = "Test vector: `seahaven-file/testdata/data/single_service_basic`"]
#[doc = ""]
#[doc = "```yaml"]
#[doc = r##"services:"##]
#[doc = r##"  app:"##]
#[doc = r##"    image: nginx:latest"##]
#[doc = r##"    ports:"##]
#[doc = r##"      - "8080:80""##]
#[doc = "```"]
#[doc = ""]
#[doc = "See file: `seahaven-file/testdata/data/single_service_basic.yaml`"]
pub const SINGLE_SERVICE_BASIC: &str = indoc::indoc! { r###"
  services:
    app:
      image: nginx:latest
      ports:
        - "8080:80""### };

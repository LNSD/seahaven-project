#[doc = "Test vector: `seahaven-file/testdata/data/single_service`"]
#[doc = ""]
#[doc = "```yaml"]
#[doc = r##"services:"##]
#[doc = r##"  app:"##]
#[doc = r##"    image: nginx:latest"##]
#[doc = r##"    ports:"##]
#[doc = r##"    - 8080:80"##]
#[doc = r##"    healthcheck:"##]
#[doc = r##"      test:"##]
#[doc = r##"      - curl"##]
#[doc = r##"      - -f"##]
#[doc = r##"      - http://localhost:80"##]
#[doc = r##"      interval: 30s"##]
#[doc = r##"      timeout: 10s"##]
#[doc = r##"      retries: 3"##]
#[doc = "```"]
#[doc = ""]
#[doc = "See file: `seahaven-file/testdata/data/single_service.yaml`"]
pub const SINGLE_SERVICE: &str = indoc::indoc! { r###"
  services:
    app:
      image: nginx:latest
      ports:
      - 8080:80
      healthcheck:
        test:
        - curl
        - -f
        - http://localhost:80
        interval: 30s
        timeout: 10s
        retries: 3"### };

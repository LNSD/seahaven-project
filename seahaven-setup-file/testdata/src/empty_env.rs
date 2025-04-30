#[doc = "Test vector: `seahaven-file/testdata/data/empty_env`"]
#[doc = ""]
#[doc = "```yaml"]
#[doc = r##"---"##]
#[doc = r##"# Empty front matter"##]
#[doc = r##"---"##]
#[doc = r##"services:"##]
#[doc = r##"  app:"##]
#[doc = r##"    image: nginx:latest"##]
#[doc = r##"    environment:"##]
#[doc = r##"      - COMPLEX_VAR="value with spaces""##]
#[doc = r##"      - EMPTY_VAR="##]
#[doc = r##"      - QUOTED_VAR="quoted value""##]
#[doc = r##"    ports:"##]
#[doc = r##"      - "${APP_PORT}:80""##]
#[doc = r##"    volumes:"##]
#[doc = r##"      - ./data:/data:ro"##]
#[doc = r##"    depends_on:"##]
#[doc = r##"      db:"##]
#[doc = r##"        condition: service_healthy"##]
#[doc = r##"      cache:"##]
#[doc = r##"        condition: service_started"##]
#[doc = r##""##]
#[doc = r##"  db:"##]
#[doc = r##"    image: postgres:14"##]
#[doc = r##"    environment:"##]
#[doc = r##"      POSTGRES_PASSWORD: "${DB_PASSWORD}""##]
#[doc = r##"    healthcheck:"##]
#[doc = r##"      test: ["CMD-SHELL", "pg_isready -U postgres"]"##]
#[doc = r##"      interval: 10s"##]
#[doc = r##"      timeout: 5s"##]
#[doc = r##"      retries: 5"##]
#[doc = r##""##]
#[doc = r##"  cache:"##]
#[doc = r##"    image: redis:7"##]
#[doc = r##"    command: redis-server --appendonly yes"##]
#[doc = r##"    volumes:"##]
#[doc = r##"      - redis-data:/data"##]
#[doc = r##"    healthcheck:"##]
#[doc = r##"      test: ["CMD", "redis-cli", "ping"]"##]
#[doc = r##"      interval: 5s"##]
#[doc = r##"      timeout: 3s"##]
#[doc = r##"      retries: 3"##]
#[doc = r##""##]
#[doc = r##"volumes:"##]
#[doc = r##"  redis-data:"##]
#[doc = "```"]
#[doc = ""]
#[doc = "See file: `seahaven-file/testdata/data/empty_env.yaml`"]
pub const EMPTY_ENV: &str = indoc::indoc! { r###"
  ---
  # Empty front matter
  ---
  services:
    app:
      image: nginx:latest
      environment:
        - COMPLEX_VAR="value with spaces"
        - EMPTY_VAR=
        - QUOTED_VAR="quoted value"
      ports:
        - "${APP_PORT}:80"
      volumes:
        - ./data:/data:ro
      depends_on:
        db:
          condition: service_healthy
        cache:
          condition: service_started

    db:
      image: postgres:14
      environment:
        POSTGRES_PASSWORD: "${DB_PASSWORD}"
      healthcheck:
        test: ["CMD-SHELL", "pg_isready -U postgres"]
        interval: 10s
        timeout: 5s
        retries: 5

    cache:
      image: redis:7
      command: redis-server --appendonly yes
      volumes:
        - redis-data:/data
      healthcheck:
        test: ["CMD", "redis-cli", "ping"]
        interval: 5s
        timeout: 3s
        retries: 3

  volumes:
    redis-data:"### };

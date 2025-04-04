/// Test vector: `setup-yaml/empty_env`
///
/// ```yaml
/// ---
/// # Empty front matter
/// ---
/// services:
///   app:
///     image: nginx:latest
///     environment:
///       - COMPLEX_VAR="value with spaces"
///       - EMPTY_VAR=
///       - QUOTED_VAR="quoted value"
///     ports:
///       - "${APP_PORT}:80"
///     volumes:
///       - ./data:/data:ro
///     depends_on:
///       db:
///         condition: service_healthy
///       cache:
///         condition: service_started
///
///   db:
///     image: postgres:14
///     environment:
///       POSTGRES_PASSWORD: "${DB_PASSWORD}"
///     healthcheck:
///       test: ["CMD-SHELL", "pg_isready -U postgres"]
///       interval: 10s
///       timeout: 5s
///       retries: 5
///
///   cache:
///     image: redis:7
///     command: redis-server --appendonly yes
///     volumes:
///       - redis-data:/data
///     healthcheck:
///       test: ["CMD", "redis-cli", "ping"]
///       interval: 5s
///       timeout: 3s
///       retries: 3
///
/// volumes:
///   redis-data:
/// ```
///
/// See file: `setup-yaml/empty_env.yaml`
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

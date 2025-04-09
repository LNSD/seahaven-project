Init containers are specialized containers that run and complete before the main application containers in a service. They are perfect for setup tasks, data loading, or any prerequisites that need to be satisfied before the main application starts.

# Configuration Structure

In Seahaven, init containers must be defined under a dedicated `init` key in the configuration. This separation is important to avoid key collisions when translating `setup.yaml` into `docker-compose.yaml`. The structure ensures that service names and init container names cannot overlap.

Example structure:

```yaml
services:
	chain:
		image: ghcr.io/foundry-rs/foundry:latest
		ports: ["${CHAIN_RPC}:8545"]
		command: ["anvil --host=0.0.0.0 --chain-id=1337 --base-fee=0"]
		healthcheck: { interval: 1s, retries: 10, test: "cast block" }

init:
  deploy-smart-contracts:
    build:
	    context: "./smart-contracts"
	    dockerfile: "Dockerfile.deploy"
    depends_on:
    	chain: { condition: service_healthy }
	volumes:
	- ./.env:/opt/.env:ro
	- ./contracts.json:/opt/contracts.json:ro
```

# Core Concepts

## How It Works

Init containers follow a specific execution pattern that ensures proper application startup:

- Init containers run to completion before the main container starts.
- If an init container fails, the main container won't start.
- This ensures your application only runs when all prerequisites are met.

## Key Features

Init containers provide several essential capabilities for application setup:

- Run setup scripts before the main application starts.
- Load initial data or configurations.
- Perform health checks or dependency validation.
- Execute one-time initialization tasks.

# Use cases and Implementation Patterns

These patterns demonstrate common use cases and best practices for implementing init containers in Seahaven. Each pattern includes a complete example that you can adapt to your needs.

## Configuration Management

This pattern demonstrates how to generate and manage configuration files using templates and environment variables. It's useful for creating application-specific configuration that depends on runtime environment.

```yaml
init:
  init-config:
    image: alpine:latest
    volumes:
    - ./templates:/templates
    - ./config:/config
    command: >
      sh -c "
        envsubst < /templates/config.template > /config/config.json &&
        echo 'Configuration generated successfully'
      "
```

## Multiple Init Containers and Dependency Validation

When your application requires multiple initialization steps, you can chain them together using dependencies. This pattern ensures that init containers run in the correct order and that the main application only starts when all prerequisites are met.

```yaml
services:
  app:
    image: your-app-image
    depends_on:
      init-db: { condition: service_completed_successfully }
      init-config: { condition: service_completed_successfully }

init:
  init-db:
    image: postgres:latest
    command: ["sh", "-c", "psql -U postgres -f /docker-entrypoint-initdb.d/init.sql"]
    volumes:
    - ./init.sql:/docker-entrypoint-initdb.d/init.sql

  init-config:
    image: busybox
    command: ["sh", "-c", "echo 'Config initialized' > /config/status.txt"]
    volumes:
    - ./config:/config
```

## Database Operations

Database initialization is a common use case for init containers. This pattern shows how to set up a database with proper health checks, environment configuration, and initial data loading.

```yaml
init:
  init-db:
    image: postgres:latest
    environment:
    - POSTGRES_PASSWORD=${DB_PASSWORD}
    volumes:
    - ./migrations:/docker-entrypoint-initdb.d
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5
```

## Smart Contracts Deployment

This pattern demonstrates how to deploy smart contracts to a blockchain network using init containers. It's particularly useful for setting up development environments or ensuring contracts are deployed before the main application starts.

```yaml
services:
    chain:
        image: ghcr.io/foundry-rs/foundry:latest
        ports: ["${CHAIN_RPC}:8545"]
        command: ["anvil", "--host=0.0.0.0", "--chain-id=1337", "--base-fee=0"]
        healthcheck: { interval: 1s, retries: 10, test: "cast block" }

    app:
        image: your-app-image:latest
        ports: ["${APP_PORT}:80"]
        depends_on:
            chain: { condition: service_healthy }
            deploy-smart-contracts: { condition: service_completed_successfully }
        secrets:
        - app-private-key
        environment:
        - RPC_URL=http://chain:8545
        

init:
  deploy-smart-contracts:
    build:
      context: "./smart-contracts"
      dockerfile: "Dockerfile.deploy"
    depends_on:
      chain: { condition: service_healthy }
    volumes:
    - ./.env:/opt/.env:ro
    - ./contracts.json:/opt/contracts.json:ro
```

## Environment and Secrets Management

This pattern demonstrates how to securely handle environment variables and secrets in init containers. It's particularly useful when you need to fetch or generate sensitive configuration before starting your main application.

```yaml
init:
  init-secrets:
    image: vault:latest
    environment:
    - VAULT_ADDR=${VAULT_ADDR}
    - VAULT_TOKEN=${VAULT_TOKEN}
    secrets:
    - db_password
    command: ["sh", "-c", "vault kv get secret/db"]
```


## Data Preparation and Processing

This pattern shows how to use init containers for data preparation tasks, such as downloading required data, processing configuration templates, or initializing cache storage.

```yaml
init:
  prepare-data:
    image: alpine:latest
    volumes:
    - ./data:/data
    - ./cache:/cache
    command: >
      sh -c "
        wget -O /data/initial-dataset.json ${DATA_URL} &&
        jq '. | to_entries | map({key: .key, value: .value})' /data/initial-dataset.json > /cache/processed-data.json
      "
```

# Best Practices

When implementing init containers, following best practices is crucial for creating reliable, secure, and efficient initialization processes. These guidelines cover container design, security considerations, and performance optimization to help you build robust init containers that effectively prepare your environment for the main application.

## Container Design

Follow these guidelines to create efficient and reliable init containers:

1. **Keep Containers Lightweight**
   - Use minimal base images.
   - Include only necessary tools.
   - Optimize for quick execution.

2. **Handle Failures Gracefully**
   - Implement proper error handling.
   - Provide clear error messages.
   - Set appropriate timeouts.

3. **Ensure Idempotency**
   - Design init scripts to be re-runnable.
   - Include checks for existing state.
   - Use conditional logic for setup steps.

## Security

Security is crucial when designing init containers. Consider these aspects:

1. **Principle of Least Privilege**
   - Run init containers with minimal permissions.
   - Use non-root users when possible.
   - Limit network access.

2. **Secret Management**
   - Use Docker secrets or environment variables.
   - Avoid hardcoding sensitive information.
   - Rotate credentials regularly.

3. **Image Security**
   - Use official base images.
   - Keep images updated.
   - Scan for vulnerabilities.

## Performance

Optimize your init containers for better performance:

1. **Startup Optimization**
   - Minimize init container size.
   - Optimize initialization scripts.
   - Use caching effectively.

2. **Resource Management**
   - Set appropriate resource limits.
   - Monitor memory and CPU usage.
   - Clean up temporary files.

3. **Parallel Execution**
   - Run independent init containers in parallel.
   - Use proper dependency management.
   - Optimize startup sequence.

# Troubleshooting

## Common Issues

When working with init containers, you might encounter these common problems:

1. **Init Container Timeouts**
   - Set appropriate `timeout` values in healthchecks.
   - Use `restart: on-failure` for retry logic.
   - Implement proper logging for debugging.

2. **Resource Constraints**
   - Monitor resource usage with `docker stats`.
   - Set appropriate resource limits.
   - Use lightweight base images.

3. **Dependency Issues**
   - Verify service names and conditions.
   - Check network connectivity.
   - Ensure proper volume mounts.

## Debugging

Debugging init containers can be challenging due to their temporary nature and specific execution patterns. These are the commands you can use to diagnose and resolve issues:

1. **View Logs**
   The logs command provides visibility into the container's execution:
   ```bash
   # List all logs specific to an init container
   docker compose logs <init-container-name>
   ```
   This command shows the complete output from the init container, including startup messages, errors, and execution results. Use `--follow` to watch logs in real-time.

2. **Inspect Container State**
   To understand the container's configuration and current state:
   ```bash
   # List all containers
   docker compose ps
   # Inspect a specific container
   docker inspect <init-container-name>
   ```
   These commands help you understand the container's current state, configuration, and resource usage. The `inspect` command provides detailed information about the container's setup.

3. **Interactive Debugging**
   For hands-on investigation of the container:
   ```bash
   # Run a shell inside the container
   docker compose exec <init-container-name> sh
   ```
   This allows you to interactively explore the container's filesystem and run commands. Useful for verifying file permissions, checking environment variables, or testing commands directly.

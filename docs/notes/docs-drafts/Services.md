Services in Seahaven represent the main application containers that run your application. They are defined under the `services` section of the configuration and work in conjunction with init containers to provide a complete application environment.

> [!NOTE]
> Seahaven services follow the [Compose file specification](https://github.com/compose-spec/compose-spec/blob/main/spec.md) for service definitions.

# Configuration Structure

Services in Seahaven are defined under the `services` section of the configuration. Each service must have a unique name and can include various configuration options to define its behavior, dependencies, and resources.

> [!WARNING]
> Container names must be unique across both the `services` and `init` sections. Using the same name in both sections will cause a conflict and prevent the application from starting properly.

Example structure:

```yaml
services:
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
```

# Core Concepts

Understanding the fundamental principles of services is crucial for building reliable and efficient applications. This section covers the essential concepts that govern how services operate in Seahaven.

## How It Works

Services follow these key principles:

- Services run continuously (unlike [[docs-drafts/Init container|init containers]] which run to completion)
- Services can depend on other services and [[docs-drafts/Init container|init containers]]
- Services can be configured with various options for networking, volumes, environment variables, and more
- Services can be health-checked to ensure they're running properly

## Key Features

Services provide several essential capabilities:

- Long-running application containers
- Inter-service communication
- Resource management
- Environment configuration
- Volume mounting
- Secret management
- Health monitoring

# Use cases and Implementation Patterns

These patterns demonstrate common use cases and best practices for implementing services in Seahaven. Each pattern includes a complete example that you can adapt to your needs.

## Basic Service

The most basic service configuration includes an image and optional configuration:

```yaml
services:
  web:
    image: nginx:latest
    ports: ["${APP_PORT}:80"]
```

## Service with Environment Variables

Services can be configured with environment variables for runtime configuration:

```yaml
services:
  api:
    image: your-api-image:latest
    environment:
      - DATABASE_URL=postgresql://user:password@db:5432/mydb
      - API_KEY=${API_KEY}
    ports: ["${API_PORT}:3000"]
```

## Service with Volumes

Services can mount volumes for persistent data and configuration:

```yaml
services:
  app:
    image: your-app-image:latest
    volumes:
      - ./config:/app/config:ro
      - app-data:/app/data
    ports: ["${APP_PORT}:8080"]
```

## Service with Health Checks

Services can include health checks to ensure they're running properly:

```yaml
services:
  database:
    image: postgres:latest
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5
```

## Service Dependencies

Services can depend on other services and init containers:

```yaml
services:
  db:
    image: postgres:latest
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5

  app:
    image: your-app-image:latest
    depends_on:
      db: { condition: service_healthy }
      init-db: { condition: service_completed_successfully }

init:
  init-db:
    image: postgres:latest
    command: ["sh", "-c", "psql -U postgres -f /docker-entrypoint-initdb.d/init.sql"]
    volumes:
    - ./init.sql:/docker-entrypoint-initdb.d/init.sql
```

## Service with Secrets

Services can securely access secrets:

```yaml
services:
  api:
    image: your-api-image:latest
    secrets:
    - api-key
    - db-password
    environment:
    - API_KEY_FILE=/run/secrets/api-key
```

# Best Practices

When implementing services, following best practices is crucial for creating reliable, secure, and efficient applications. These guidelines cover container design, security considerations, and performance optimization to help you build robust services that effectively run your application.

## Container Design

1. **Image Selection**
    - Use official images when possible.
    - Specify exact versions for reproducibility.
    - Consider using multi-stage builds for custom images.

2. **Resource Management**
    - Set appropriate resource limits.
    - Monitor resource usage.
    - Use health checks to ensure proper operation.

3. **Security**
    - Run containers as non-root users.
    - Use read-only volumes where possible.
    - Implement proper secret management.

## Configuration

1. **Environment Variables**
    - Use .env files for local development.
    - Keep sensitive data in secrets.
    - Document required environment variables.

2. **Networking**
    - Use internal networks for service communication.
    - Expose only necessary ports.
    - Implement proper security groups.

3. **Volumes**
    - Use named volumes for persistent data.
    - Mount configuration files as read-only.
    - Implement proper backup strategies.

# Troubleshooting

When working with services, you might encounter various issues that need to be diagnosed and resolved. This section covers common problems and provides guidance on how to debug and fix them.

## Common Issues

1. **Service Startup Failures**
    - Check logs for error messages.
    - Verify environment variables.
    - Ensure dependencies are available.

2. **Resource Issues**
    - Monitor resource usage.
    - Adjust resource limits.
    - Check for memory leaks.

3. **Network Problems**
    - Verify network configuration.
    - Check service dependencies.
    - Ensure ports are properly exposed.

## Debugging

Debugging services can be challenging due to their distributed nature and complex interactions. These commands help you diagnose and resolve issues:

1. **View Logs**
    The logs command provides visibility into the service's execution and helps diagnose issues:
    ```bash
    # View logs for a specific service
    docker compose logs <service-name>
    ```
    This command shows the complete output from the service, including startup messages, errors, and execution results. Use `--follow` to watch logs in real-time.

2. **Inspect Service**
    To understand the service's configuration and current state:
    ```bash
    # List all services
    docker compose ps
    # Inspect a specific service
    docker inspect <service-name>
    ```
    These commands help you understand the service's current state, configuration, and resource usage. The `inspect` command provides detailed information about the service's setup.

3. **Interactive Debugging**
    For hands-on investigation of the service:
    ```bash
    # Open a shell in a specific service
    docker compose exec <service-name> sh
    ```
    This allows you to interactively explore the service's filesystem and run commands. Useful for verifying file permissions, checking environment variables, or testing commands directly.

4. **Health Checks**
    To verify the health status of your services:
    ```bash
    # Check the health of a specific service
    docker compose ps --filter "status=healthy"
    ```
    This command helps you monitor service health and identify any services that might be experiencing issues. Use this to ensure all services are running properly.

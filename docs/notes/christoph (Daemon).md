#next 

An ephemeral server (implemented in Rust?/Go?) could be running monitoring the services a-la-test-containers. That should act as the intermediary between the test cases and the environment.
# Notes
- MCP server
- Ryuk (testcontainers reaper functionality)
- Orchestrator for concurrent test runs. See [[Tests]]

- https://github.com/testcontainers/moby-ryuk
- https://docs.gradle.org/current/userguide/gradle_daemon.html
- https://blog.gradle.org/how-gradle-works-1 (Startup)
- https://blog.gradle.org/how-gradle-works-2 (Inside the Daemon)
- https://blog.worldline.tech/2023/01/04/ryuk.html
//! Docker command builder
//!
//! Supported commands:
//! - `docker version`
//! - `docker system info`
//! - `docker system prune`
//! - `docker compose build`
//! - `docker compose up`
//! - `docker compose down`

mod common;
pub mod compose;
mod root;
pub mod system;
pub mod version;

pub use common::IntoCommand;
pub use root::DockerCmd;

#[cfg(test)]
mod tests {
    mod common;
    mod it_root;
    mod it_system;
    mod it_version;
}

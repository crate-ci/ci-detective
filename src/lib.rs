//! Detect what CI this code is running on and extract the information it provides from env vars.
//!
//! Documentation of individual build environment variables is from the appropriate
//! documentation and is copyright the provider under the original license

fn env(var: &str) -> Option<String> {
    let env_var = std::env::var(var).unwrap_or_default();
    if !env_var.is_empty() {
        Some(env_var)
    } else {
        None
    }
}

/// Jenkins CI
pub mod jenkins;
pub use jenkins::Jenkins;

/// Travis CI
pub mod travis;
pub use travis::Travis;

/// Docker
pub mod docker;
pub use docker::Docker;

/// Codeship CI
pub mod codeship;
pub use codeship::Codeship;

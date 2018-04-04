//! Detect what CI this code is running on and extract the information it provides from env vars.
//!
//! Documentation of individual build environment variables is from the appropriate
//! documentation and is copyright the provider under the original license

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, missing_docs, unsafe_code, unused)]
#![warn(unreachable_pub)]
#![cfg_attr(feature = "nightly", feature(non_exhaustive))]

/// Grab the configuration from whatever CI you're on.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "nightly", non_exhaustive)]
pub enum CI {
    /// Jenkins CI
    Jenkins(Jenkins),
    /// Travis CI
    Travis(Travis),
    /// Docker
    Docker(Docker),
    /// Codeship CI
    Codeship(Codeship),
    /// Codefresh CI
    Codefresh(Codefresh),
    /// Circle CI
    Circle(Circle),
    #[doc(hidden)]
    __NonExhaustive,
}

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

/// Codefresh CI
pub mod codefresh;
pub use codefresh::Codefresh;

// TeamCity CI doesn't provide environment variables by default
// <https://github.com/codecov/codecov-bash/blob/8b76995ad4a95a61cecd4b049a448a402d91d197/codecov#L521-L547>

/// Circle CI
pub mod circle;
pub use circle::Circle;

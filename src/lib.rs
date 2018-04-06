//! Detect what CI this code is running on and extract the information it provides from env vars.
//!
//! Documentation of individual build environment variables is from the appropriate
//! documentation and is copyright the provider under the original license

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, missing_docs, unsafe_code, unused)]
#![warn(unreachable_pub)]
#![cfg_attr(feature = "nightly", feature(non_exhaustive))]

#[macro_use]
extern crate ci_detective_internal_derive;

/// Grab the configuration from whatever CI you're on.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "nightly", non_exhaustive)]
pub enum CI {
    // /// Jenkins CI
    // Jenkins(Box<Jenkins>),
    // /// Travis CI
    // Travis(Box<Travis>),
    // /// Docker
    // Docker(Box<Docker>),
    // /// Codeship CI
    // Codeship(Box<Codeship>),
    // /// Codefresh CI
    // Codefresh(Box<Codefresh>),
    /// Circle CI
    Circle(Box<Circle>),
    // /// Appveyor CI
    // Appveyor(Box<Appveyor>),
    #[doc(hidden)]
    __NonExhaustive,
}

impl CI {
    /// Grab the CI environment information lazily
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn lazy() -> Option<Self> {
        None
        //  .or_else(|| Jenkins  ::lazy().map(|it| CI::Jenkins  (Box::new(it))))
        //  .or_else(|| Travis   ::lazy().map(|it| CI::Travis   (Box::new(it))))
        //  .or_else(|| Docker   ::lazy().map(|it| CI::Docker   (Box::new(it))))
        //  .or_else(|| Codeship ::lazy().map(|it| CI::Codeship (Box::new(it))))
        //  .or_else(|| Codefresh::lazy().map(|it| CI::Codefresh(Box::new(it))))
            .or_else(|| Circle   ::lazy().map(|it| CI::Circle   (Box::new(it))))
        //  .or_else(|| Appveyor ::lazy().map(|it| CI::Appveyor (Box::new(it))))
    }

    /// Grab the CI environment information eagerly
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn eager() -> Option<Self> {
        None
        //  .or_else(|| Jenkins  ::lazy().map(|mut it| { it.load(); CI::Jenkins  (Box::new(it)) }))
        //  .or_else(|| Travis   ::lazy().map(|mut it| { it.load(); CI::Travis   (Box::new(it)) }))
        //  .or_else(|| Docker   ::lazy().map(|mut it| { it.load(); CI::Docker   (Box::new(it)) }))
        //  .or_else(|| Codeship ::lazy().map(|mut it| { it.load(); CI::Codeship (Box::new(it)) }))
        //  .or_else(|| Codefresh::lazy().map(|mut it| { it.load(); CI::Codefresh(Box::new(it)) }))
            .or_else(|| Circle   ::lazy().map(|mut it| { it.load(); CI::Circle   (Box::new(it)) }))
        //  .or_else(|| Appveyor ::lazy().map(|mut it| { it.load(); CI::Appveyor (Box::new(it)) }))
    }
}

fn env(var: &str) -> Option<String> {
    let env_var = std::env::var(var).unwrap_or_default();
    if !env_var.is_empty() {
        Some(env_var)
    } else {
        None
    }
}

///// Jenkins CI
//pub mod jenkins;
//pub use jenkins::Jenkins;
//
///// Travis CI
//pub mod travis;
//pub use travis::Travis;
//
///// Docker
//pub mod docker;
//pub use docker::Docker;
//
///// Codeship CI
//pub mod codeship;
//pub use codeship::Codeship;
//
///// Codefresh CI
//pub mod codefresh;
//pub use codefresh::Codefresh;

// TeamCity CI doesn't provide environment variables by default
// <https://github.com/codecov/codecov-bash/blob/8b76995ad4a95a61cecd4b049a448a402d91d197/codecov#L521-L547>

/// Circle CI
pub mod circle;
pub use circle::Circle;

///// Appveyor CI
//pub mod appveyor;
//pub use appveyor::Appveyor;

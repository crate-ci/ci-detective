// Portions of the CI information extraction are based off of codecov/codecov-bash
// Apache License Version 2.0, January 2004
// https://github.com/codecov/codecov-bash/blob/master/LICENSE

// Documentation of individual build environment variables is from the appropriate documentation
// And is copyright the provider under the original license

//! The environment definitions guaranteed by various vendors.
//!
//! No interpretation is done at this level.

/// Jenkins CI
pub mod jenkins;
pub use self::jenkins::Jenkins;

/// Travis CI
pub mod travis;
pub use self::travis::Travis;

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
    // /// Jenkins CI
    // Jenkins(Jenkins),
    // /// Travis CI
    // Travis(Travis),
    // /// Docker
    // Docker(Docker),
    // /// Codeship CI
    // Codeship(Codeship),
    // /// Codefresh CI
    // Codefresh(Codefresh),
    /// Circle CI
    Circle(Circle),
    // /// Appveyor CI
    // Appveyor(Appveyor),
    #[doc(hidden)]
    __NonExhaustive,
}

impl CI {
    /// Grab the CI environment information lazily
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn lazy() -> Option<Self> {
        None
        //  .or_else(|| Jenkins  ::lazy().map(CI::Jenkins  ))
        //  .or_else(|| Travis   ::lazy().map(CI::Travis   ))
        //  .or_else(|| Docker   ::lazy().map(CI::Docker   ))
        //  .or_else(|| Codeship ::lazy().map(CI::Codeship ))
        //  .or_else(|| Codefresh::lazy().map(CI::Codefresh))
            .or_else(|| Circle   ::lazy().map(CI::Circle   ))
        //  .or_else(|| Appveyor ::lazy().map(CI::Appveyor ))
    }

    /// Grab the CI environment information eagerly
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn eager() -> Option<Self> {
        None
        //  .or_else(|| Jenkins  ::lazy().map(|mut it| { it.load(); Jenkins  (it) }))
        //  .or_else(|| Travis   ::lazy().map(|mut it| { it.load(); Travis   (it) }))
        //  .or_else(|| Docker   ::lazy().map(|mut it| { it.load(); Docker   (it) }))
        //  .or_else(|| Codeship ::lazy().map(|mut it| { it.load(); Codeship (it) }))
        //  .or_else(|| Codefresh::lazy().map(|mut it| { it.load(); Codefresh(it) }))
            .or_else(|| Circle   ::lazy().map(|mut it| { it.load(); CI::Circle   (it) }))
        //  .or_else(|| Appveyor ::lazy().map(|mut it| { it.load(); Appveyor (it) }))
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

// TODO(CAD): Hide mutability with Cell?
// TODO(FUTURE_RUST(1.26)): Use FromStr again (revert this line's blame)
#[derive(Copy, Clone, Debug)]
struct LazyEnv<T>(Option<T>, &'static str);

impl<T> LazyEnv<T> {
    fn new(var: &'static str) -> Self {
        LazyEnv(None, var)
    }
}

macro_rules! lazy_env_get {
    (::std::path::PathBuf) => {
        impl LazyEnv<::std::path::PathBuf> {
            fn get(&mut self) -> Option<&::std::path::PathBuf> {
                if self.0.is_none() {
                    self.0 = env(self.1).map(Into::into);
                }
                self.0.as_ref()
            }
        }
    };
    ($ty:ty) => {
        impl LazyEnv<$ty> {
            fn get(&mut self) -> Option<&$ty> {
                if self.0.is_none() {
                    self.0 = env(self.1).and_then(|it| it.parse().ok());
                }
                self.0.as_ref()
            }
        }
    };
    ($head:ty, $($tail:tt)*) => { lazy_env_get!($head); lazy_env_get!($($tail)*); };
    (,) => {}
}
lazy_env_get!(u32, String, ::std::path::PathBuf);

// TODO(CAD): Make this an actual proc macro for proper attribute handling?
// This will require removing the ?/! sugar.
macro_rules! ci {
    (
        //$(#[ci(require($require_environment_present:ident))])*;
        // TODO(FUTURE-RUST#48075): Kleene Operator for trailing comma
        #[ci(require($($require_environment_var:ident = $require_environment_val:expr),+$(,)*))]
        $(#[$struct_doc:meta])*
        pub struct $CI:ident {$(
            #[ci(env($member_env:ident))]
            $(#[$member_doc:meta])*
            $member:ident: $member_ty:ident $optionality:tt ,
        )*}
    ) => {
        $(#[$struct_doc])*
        #[derive(Clone, Debug)]
        pub struct $CI {$(
            $member: ::LazyEnv<$member_ty>,
        )*}

        impl $CI {$(
            ci!{__impl $(#[$member_doc])* $member $optionality : $member_ty}
        )*}

        impl $CI {
            /// Construct a lazy instance of this CI's configuration.
            ///
            /// Returns None if the environment doesn't look like the CI's.
            /// Most CI has a set of environment variables which identify it,
            /// such as `TRAVIS=true` for Travis CI. Those are checked eagerly
            /// here.
            pub fn lazy() -> Option<Self> {
                if !(
                    //$(::env($require_environment_present).is_some() &&)*
                    $(::env(stringify!($require_environment_var))? == $require_environment_val &&)*
                    true
                ) { return None; }
                Some($CI {$(
                    $member: ::LazyEnv::new(stringify!($member_env)),
                )*})
            }

            pub(crate) fn load(&mut self) {
                $(let _ = self.$member();)*
            }

            /// Construct an instance of this CI's configuration and load it eagerly.
            ///
            /// # Panics
            ///
            /// If any of the expected environment variables are not present.
            pub fn eager() -> Self {
                let mut ci = Self::lazy().expect(concat!(
                    "Environment does not look like ",
                    stringify!($CI),
                ));
                ci.load();
                ci
            }
        }
    };

    (__impl $(#[$member_doc:meta])* $member:ident? : $member_ty:ty) => {
        $(#[$member_doc])*
        pub fn $member(&mut self) -> Option<&$member_ty> {
            self.$member.get()
        }
    };

    (__impl $(#[$member_doc:meta])* $member:ident! : $member_ty:ty) => {
        $(#[$member_doc])*
        pub fn $member(&mut self) -> &$member_ty {
            self.$member.get().expect(concat!(
                "Environment variable ",
                stringify!($member),
                " expected to parse to ",
                stringify!($member_ty),
                " but failed",
            ))
        }
    };
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

use env;
use std::path::PathBuf;
use std::str::FromStr;

/// Travis CI
///
/// # References
///
/// - <https://docs.travis-ci.com/user/environment-variables/#Default-Environment-Variables>
/// - <https://github.com/codecov/codecov-bash/blob/8b76995ad4a95a61cecd4b049a448a402d91d197/codecov#L467-L489>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "nightly", non_exhaustive)]
pub struct Travis {
    /// - set to `true` if the job is allowed to fail.
    /// - set to `false` if the job is not allowed to fail.
    pub allow_failure: bool,
    /// - for push builds, or builds not triggered by a pull request,
    ///   this is the name of the branch.
    /// - for builds triggered by a pull request
    ///   this is the name of the branch targeted by the pull request.
    /// - for builds triggered by a tag,
    ///   this is the same as the name of the tag (`TRAVIS_TAG`).
    ///
    /// > _Note that for tags, git does not store the branch from which a commit was tagged._
    pub branch: String,
    /// The absolute path to the directory where the repository
    /// being built has been copied on the worker.
    pub build_dir: PathBuf,
    /// The id of the current build that Travis CI uses internally.
    pub build_id: String,
    /// The number of the current build (for example, `4`).
    pub build_number: usize,
    /// The commit that the current build is testing.
    pub commit: String,
    /// The commit subject and body, unwrapped.
    pub commit_message: String,
    /// The range of commits that were included in the push or pull request.
    /// (Note that this is empty for builds triggered by the initial commit of a new branch.)
    pub commit_range: Option<String>,
    /// Indicates how the build was triggered.
    pub event_type: EventType,
    /// The id of the current job that Travis CI uses internally.
    pub job_id: String,
    /// The number of the current job (for example, `4.1`).
    pub job_number: String,
    /// On multi-OS builds, this value indicates the platform the job is running on.
    pub os: Option<OS>,
    /// The `osx_image` value configured in `.travis.yml`.
    /// If this is not set in `.travis.yml`, it is emtpy.
    pub osx_image: Option<String>,
    /// The pull request number if the current job is a pull request.
    pub pull_request: Option<String>,
    /// If the current job is a pull request, the name of the branch from which the PR originated.
    pub pull_request_branch: Option<String>,
    /// if the current job is a pull request, the commit SHA of the HEAD commit of the PR.
    pub pull_request_sha: Option<String>,
    /// if the current job is a pull request, the slug (in the form `owner_name/repo_name`)
    /// of the repository from which the PR originated.
    pub pull_request_slug: Option<String>,
    /// The slug (in form: `owner_name/repo_name`) of the repository currently being built.
    pub repo_slug: String,
    /// - set to `true` if there are any encrypted environment variables.
    /// - set to `false` if no encrypted environment variables are available.
    pub secure_env_vars: bool,
    /// `true` or `false` based on whether `sudo` is enabled.
    pub sudo: bool,
    /// If the current build is for a git tag, this variable is set to the tag’s name.
    pub tag: Option<String>,
    /// The current version of dart being used to run the build (if any).
    pub dart_version: Option<String>,
    /// The current version of go being used to run the build (if any).
    pub go_version: Option<String>,
    /// The current version of haxe being used to run the build (if any).
    pub haxe_version: Option<String>,
    /// The current version of jdk being used to run the build (if any).
    pub jdk_version: Option<String>,
    /// The current version of julia being used to run the build (if any).
    pub julia_version: Option<String>,
    /// The current version of node being used to run the build (if any).
    pub node_version: Option<String>,
    /// The current version of otp being used to run the build (if any).
    pub otp_release: Option<String>,
    /// The current version of perl being used to run the build (if any).
    pub perl_version: Option<String>,
    /// The current version of php being used to run the build (if any).
    pub php_version: Option<String>,
    /// The current version of python being used to run the build (if any).
    pub python_version: Option<String>,
    /// The current version of r being used to run the build (if any).
    pub r_version: Option<String>,
    /// The current version of ruby being used to run the build (if any).
    pub ruby_version: Option<String>,
    /// The current version of rust being used to run the build (if any).
    pub rust_version: Option<String>,
    /// The current version of scala being used to run the build (if any).
    pub scala_version: Option<String>,
    /// The current XCode SDK being used to run the build (if any).
    pub xcode_sdk: Option<String>,
    /// The current XCode Scheme being used to run the build (if any).
    pub xcode_scheme: Option<String>,
    /// The current XCode Project being used to run the build (if any).
    pub xcode_project: Option<String>,
    /// The current XCode Workspace being used to run the build (if any).
    pub xcode_workspace: Option<String>,
    non_exhaustive: (),
}

impl Travis {
    /// Construct this provider's information from the environment.
    pub fn from_env() -> Option<Self> {
        if !(env("CI")? == "true" && env("TRAVIS")? == "true"
            && env("CONTINUOUS_INTEGRATION")? == "true"
            && env("DEBIAN_FRONTEND")? == "noninteractive"
            && env("HAS_JOSH_K_SEAL_OF_APPROVAL")? == "true")
        {
            return None;
        }

        Some(Travis {
            allow_failure: env("TRAVIS_ALLOW_FAILURE")?.parse().ok()?,
            branch: env("TRAVIS_BRANCH")?,
            build_dir: env("TRAVIS_BUILD_DIR")?.into(),
            build_id: env("TRAVIS_BUILD_ID")?,
            build_number: env("TRAVIS_BUILD_NUMBER")?.parse().ok()?,
            commit: env("TRAVIS_COMMIT")?,
            commit_message: env("TRAVIS_COMMIT_MESSAGE")?,
            commit_range: env("TRAVIS_COMMIT_RANGE"),
            event_type: env("TRAVIS_EVENT_TYPE")?.parse().ok()?,
            job_id: env("TRAVIS_JOB_ID")?,
            job_number: env("TRAVIS_JOB_NUMBER")?,
            os: env("TRAVIS_OS_NAME").and_then(|it| it.parse().ok()),
            osx_image: env("TRAVIS_OSX_IMAGE"),
            pull_request: if let Some(pr) = env("TRAVIS_PULL_REQUEST") {
                if pr != "false" {
                    Some(pr)
                } else {
                    None
                }
            } else {
                None
            },
            pull_request_branch: env("TRAVIS_PULL_REQUEST_BRANCH"),
            pull_request_sha: env("TRAVIS_PULL_REQUEST_SHA"),
            pull_request_slug: env("TRAVIS_PULL_REQUEST_SLUG"),
            repo_slug: env("TRAVIS_REPO_SLUG")?,
            secure_env_vars: env("TRAVIS_SECURE_ENV_VARS")?.parse().ok()?,
            sudo: env("TRAVIS_SUDO")?.parse().ok()?,
            tag: env("TRAVIS_TAG"),
            dart_version: env("TRAVIS_DART_VERSION"),
            go_version: env("TRAVIS_GO_VERSION"),
            haxe_version: env("TRAVIS_HAXE_VERSION"),
            jdk_version: env("TRAVIS_JDK_VESRION"),
            julia_version: env("TRAVIS_JULIA_VERSION"),
            node_version: env("TRAVIS_NODE_VERSION"),
            otp_release: env("TRAVIS_OTP_RELEASE"),
            perl_version: env("TRAVIS_PERL_VERSION"),
            php_version: env("TRAVIS_PHP_VERSION"),
            python_version: env("TRAVIS_PYTHON_VERSION"),
            r_version: env("TRAVIS_R_VERSION"),
            ruby_version: env("TRAVIS_RUBY_VERSION"),
            rust_version: env("TRAVIS_RUST_VERSION"),
            scala_version: env("TRAVIS_SCALA_VERSION"),
            xcode_sdk: env("TRAVIS_XCODE_SDK"),
            xcode_scheme: env("TRAVIS_XCODE_SCHEME"),
            xcode_project: env("TRAVIS_XCODE_PROJECT"),
            xcode_workspace: env("TRAVIS_XCODE_WORKSPACE"),
            non_exhaustive: (),
        })
    }
}

/// Indicates how the build was triggered.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "nightly", non_exhaustive)]
#[allow(missing_docs)]
pub enum EventType {
    Push,
    PullRequest,
    Api,
    Cron,
    #[doc(hidden)]
    __NonExhaustive,
}

impl FromStr for EventType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "push" => Ok(EventType::Push),
            "pull_request" => Ok(EventType::PullRequest),
            "api" => Ok(EventType::Api),
            "cron" => Ok(EventType::Cron),
            _ => Err(()),
        }
    }
}

/// On multi-OS builds, this value indicates the platform the job is running on.
/// To be extended in the future.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "nightly", non_exhaustive)]
#[allow(missing_docs)]
pub enum OS {
    Linux,
    MacOS,
    #[doc(hidden)]
    __NonExhaustive,
}

impl FromStr for OS {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "linux" => Ok(OS::Linux),
            "osx" => Ok(OS::MacOS),
            _ => Err(()),
        }
    }
}

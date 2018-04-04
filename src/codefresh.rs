use env;
use std::path::PathBuf;
use std::str::FromStr;

/// Codefresh CI
///
/// # References
///
/// - <https://github.com/codecov/codecov-bash/blob/8b76995ad4a95a61cecd4b049a448a402d91d197/codecov#L511-L520>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "nightly", non_exhaustive)]
pub struct Codefresh {
    /// Repository owner.
    pub repo_owner: String,
    /// Repository name.
    pub repo_name: String,
    /// Branch name of the Git repository of the main pipeline, at the time of execution.
    pub branch: String,
    /// Commit author.
    pub commit_author: String,
    /// Commit url.
    pub commit_url: String,
    /// Commit message of the git repository revision, at the time of execution.
    pub commit_message: String,
    /// Revision of the Git repository of the main pipeline, at the time of execution.
    pub revision: String,
    /// Abbreviated 7-character revision hash, as used in git.
    pub short_revision: String,
    /// Will refer to the volume that was generated for the specific flow.
    /// Can be used in conjunction with a composition to provide access to your cloned repository.
    pub volume_name: String,
    /// Will refer to the mounted path of the workflow volume inside a Freestyle container.
    pub volume_path: PathBuf,
    /// Will be an indication of the current build was triggered.
    pub build_trigger: BuildTrigger,
    /// The build id.
    pub build_id: String,
    /// The timestamp the build was created.
    pub build_timestamp: String,
    /// The URL to the build in Codefresh
    pub build_url: String,
    /// Path to kubeconfig if exist
    pub kubeconfig_path: Option<PathBuf>,
    non_exhaustive: (),
}

impl Codefresh {
    /// Construct this provider's information from the environment.
    pub fn from_env() -> Option<Self> {
        Some(Codefresh {
            repo_owner: env("CF_REPO_OWNER")?,
            repo_name: env("CF_REPO_NAME")?,
            branch: env("CF_BRANCH")?,
            commit_author: env("CF_COMMIT_AUTHOR")?,
            commit_url: env("CF_COMMIT_URL")?,
            commit_message: env("CF_COMMIT_MESSAGE")?,
            revision: env("CF_REVISION")?,
            short_revision: env("CF_SHORT_REVISION")?,
            volume_name: env("CF_VOLUME_NAME")?,
            volume_path: PathBuf::from(env("CF_VOLUME_PATH")?),
            build_trigger: env("CF_BUILD_TRIGGER")?.parse().ok()?,
            build_id: env("CF_BUILD_ID")?,
            build_timestamp: env("CF_BUILD_TIMESTAMP")?,
            build_url: env("CF_BUILD_URL")?,
            kubeconfig_path: env("CF_KUBECONFIG_PATH").map(PathBuf::from),
            non_exhaustive: (),
        })
    }
}

/// How the current build was triggered.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "nightly", non_exhaustive)]
pub enum BuildTrigger {
    /// The build was triggered from the build button.
    Build,
    /// The build was triggered from a control version webhook.
    Webhook,
    #[doc(hidden)]
    __NonExhaustive,
}

impl FromStr for BuildTrigger {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "build" => Ok(BuildTrigger::Build),
            "webhook" => Ok(BuildTrigger::Webhook),
            _ => Err(()),
        }
    }
}

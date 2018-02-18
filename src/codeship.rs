use env;

/// Codeship CI
///
/// # References
///
/// - <https://documentation.codeship.com/basic/builds-and-configuration/set-environment-variables/#default-environment-variables>
/// - <https://github.com/codecov/codecov-bash/blob/8b76995ad4a95a61cecd4b049a448a402d91d197/codecov#L501-L510>
#[derive(Clone, Debug)]
pub struct Codeship {
    pub branch: String,
    pub build_number: String,
    pub build_url: String,
    pub committer_email: String,
    pub committer_name: String,
    pub committer_username: String,
    pub commit_id: String,
    pub message: String,
    pub repo_name: String,
    non_exhaustive: (),
}

impl Codeship {
    /// Construct this provider's information from the environment.
    pub fn from_env() -> Option<Self> {
        if !(env("CI")? == "true" && env("CI_NAME")? == "codeship") {
            return None;
        }

        Some(Codeship {
            branch: env("CI_BRANCH")?,
            build_number: env("CI_BUILD_NUMBER")?,
            build_url: env("CI_BUILD_URL")?,
            committer_email: env("CI_COMMITTER_EMAIL")?,
            committer_name: env("CI_COMMITTER_NAME")?,
            committer_username: env("CI_COMMITTER_USERNAME")?,
            commit_id: env("CI_COMMIT_ID")?,
            message: env("CI_MESSAGE")?,
            repo_name: env("CI_REPO_NAME")?,
            non_exhaustive: (),
        })
    }
}

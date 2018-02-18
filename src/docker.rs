use env;

/// Docker
///
/// # References
///
/// - <https://docs.docker.com/docker-cloud/builds/advanced/>
/// - <https://github.com/codecov/codecov-bash/blob/8b76995ad4a95a61cecd4b049a448a402d91d197/codecov#L490-L500>
#[derive(Clone, Debug)]
pub struct Docker {
    /// The name of the branch or the tag that is currently being tested.
    pub source_branch: String,
    /// The SHA1 hash of the commit being tested.
    pub source_commit: String,
    /// The message from the commit being tested and built.
    pub commit_msg: String,
    /// The name of the Docker repository being built.
    pub repo: String,
    /// The Docker repository tag being built.
    pub cache_tag: String,
    /// The name and tag of the Docker repository being built.
    /// (This variable is a combination of `DOCKER_REPO`:`CACHE_TAG`.)
    pub image_name: String,
    non_exhaustive: (),
}

impl Docker {
    /// Construct this provider's information from the environment.
    pub fn from_env() -> Option<Self> {
        Some(Docker {
            source_branch: env("SOURCE_BRANCH")?,
            source_commit: env("SOURCE_COMMIT")?,
            commit_msg: env("COMMIT_MSG")?,
            repo: env("DOCKER_REPO")?,
            cache_tag: env("CACHE_TAG")?,
            image_name: env("IMAGE_NAME")?,
            non_exhaustive: (),
        })
    }
}

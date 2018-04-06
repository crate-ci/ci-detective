use std::path::PathBuf;

/// Circle CI
///
/// # References
///
/// - <https://circleci.com/docs/2.0/env-vars/#circleci-built-in-environment-variables>
/// - <https://github.com/codecov/codecov-bash/blob/8b76995ad4a95a61cecd4b049a448a402d91d197/codecov#L548-L568>
#[derive(Clone, Debug, CI)]
#[ci(require(CI = "true", CIRCLECI = "true"))]
pub struct Circle {
    /// Your home directory.
    #[ci(env = "HOME")]
    #[ci(expected)]
    home: Option<PathBuf>,

    /// The name of the Git branch currently being built.
    #[ci(env = "CIRCLE_BRANCH")]
    #[ci(expected)]
    branch: Option<String>,

    /// An integer representing the number of total build instances.
    #[ci(env = "CIRCLE_NODE_TOTAL")]
    #[ci(expected)]
    node_total: Option<u32>,

    /// An integer between 0 and (`node_total` - 1) representing a specific build instance.
    #[ci(env = "CIRCLE_BUILD_NUM")]
    #[ci(expected)]
    node_index: Option<u32>,

    /// The CircleCI build number.
    #[ci(env = "CIRCLE_PREVIOUS_BUILD_NUM")]
    #[ci(expected)]
    build_num: Option<u32>,

    /// The number of previous builds in the branch.
    #[ci(env = "CIRCLE_PREVIOUS_BUILD_NUM")]
    #[ci(expected)]
    previous_build_num: Option<u32>,

    /// The URL for the current build.
    #[ci(env = "CIRCLE_BUILD_URL")]
    #[ci(expected)]
    build_url: Option<String>,

    /// The SHA1 hash for the current build’s last commit.
    #[ci(env = "CIRCLE_SHA1")]
    #[ci(expected)]
    sha1: Option<String>,

    /// The GitHub/Bitbucket username of the user who triggered the build.
    #[ci(env = "CIRCLE_USERNAME")]
    #[ci(expected)]
    username: Option<String>,

    /// The current job’s name.
    #[ci(env = "CIRCLE_JOB")]
    #[ci(expected)]
    job: Option<String>,

    /// The `working_directory` for the current job.
    #[ci(env = "CIRCLE_WORKING_DIRECTORY")]
    #[ci(expected)]
    working_directory: Option<PathBuf>,

    /// The GitHub/Bitbucket compare URL between commits in the build.
    #[ci(env = "CIRCLE_COMPARE_URL")]
    #[ci(expected)]
    compare_url: Option<String>,

    /// The GitHub/Bitbucket repository URL.
    #[ci(env = "CIRCLE_REPOSITORY_URL")]
    #[ci(expected)]
    repository_url: Option<String>,

    /// The GitHub/Bitbucket pull request number.
    #[ci(env = "CIRCLE_PR_NUMBER")]
    pr_number: Option<u32>,

    /// The GitHub/Bitbucket repository name in which the pull request was made.
    #[ci(env = "CIRCLE_PR_REPONAME")]
    pr_reponame: Option<String>,

    /// The GitHub/Bitbucket username of the user who created the pull request.
    #[ci(env = "CIRCLE_PR_USERNAME")]
    pr_username: Option<String>,

    /// Comma-separated list of URLs of pull requests this build is a part of.
    #[ci(env = "CIRCLE_PULL_REQUESTS")]
    pull_requests: Option<String>,

    /// If this build is part of only one pull request, its URL will be populated here.
    /// If there was more than one pull request,
    /// it will contain one of the pull request URLs (picked randomly).
    #[ci(env = "CIRCLE_PULL_REQUEST")]
    pull_request: Option<String>,

    /// The name of the git tag being tested, e.g. ‘release-v1.5.4’, if the build is running for a tag.
    #[ci(env = "CIRCLE_TAG")]
    tag: Option<String>,

    /// The username or organization name of the project being tested,
    /// i.e. “foo” in circleci.com/gh/foo/bar/123.
    #[ci(env = "CIRCLE_PROJECT_USERNAME")]
    #[ci(expected)]
    project_username: Option<String>,

    /// The repository name of the project being tested,
    /// i.e. “bar” in circleci.com/gh/foo/bar/123.
    #[ci(env = "CIRCLE_PROJECT_REPONAME")]
    #[ci(expected)]
    project_reponame: Option<String>,

    /// The directory where test timing data can be found.
    #[ci(env = "CIRCLE_INTERNAL_TASK_DATA")]
    #[ci(expected)]
    internal_task_data: Option<PathBuf>,

    /// The job being executed. The default 2.0 job is `build` without using Workflows.
    #[ci(env = "CIRCLE_STAGE")]
    #[ci(expected)]
    stage: Option<String>,
}

use env;

/// Circle CI
///
/// # References
///
/// - <https://circleci.com/docs/1.0/environment-variables/>
/// - <https://github.com/codecov/codecov-bash/blob/8b76995ad4a95a61cecd4b049a448a402d91d197/codecov#L548-L568>
#[derive(Clone, Debug)]
pub struct Circle {
    /// The username or organization name of the project being tested,
    /// i.e. `foo` in `circleci.com/gh/foo/bar/123`
    project_username: String,
    /// The repository name of the project being tested,
    /// i.e. `bar` in `circleci.com/gh/foo/bar/123`
    project_reponame: String,
    /// The name of the Git branch being tested, e.g. `master`,
    /// if the build is running for a branch.
    branch: Option<String>,
    /// The name of the git tag being tested, e.g. `release-v1.5.4`,
    /// if the build is running [for a tag](https://circleci.com/docs/1.0/configuration/#tags).
    tag: Option<String>,
    /// The SHA1 of the commit being tested.
    sha1: String,
    /// A link to the homepage for the current repository,
    /// for example, `https://github.com/circleci/frontend`.
    repository_url: String,
    /// A link to GitHub’s comparison view for this push.
    /// Not present for builds that are triggered by GitHub pushes.
    compare_url: Option<String>,
    /// A permanent link to the current build,
    /// for example, `https://circleci.com/gh/circleci/frontend/933`.
    build_url: String,
    /// The build number, same as in `circleci.com/gh/foo/bar/123`
    build_num: usize,
    /// The build number of the previous build, same as in `circleci.com/gh/foo/bar/123`
    previous_build_num: Option<usize>,
    /// Comma-separated list of pull requests this build is a part of.
    pull_requests: Option<String>,
    /// If this build is part of only one pull request, its URL will be populated here. If there was
    /// more than one pull request, it will contain one of the pull request URLs (picked randomly).
    pull_request: Option<String>,
    /// The directory whose contents are automatically saved as
    /// [build artifacts](https://circleci.com/docs/1.0/build-artifacts/).
    artifacts: String,
    /// The GitHub login of the user who either pushed the code
    /// to GitHub or triggered the build from the UI/API.
    username: String,
    /// The directory whose contents are automatically processed as
    /// [JUnit test metadata](https://circleci.com/docs/1.0/test-metadata/).
    test_reports: String,
    /// When the build is a part of a pull request from a fork,
    /// The username of the owner of the fork.
    pr_username: Option<String>,
    /// When the build is a part of a pull request from a fork,
    /// The name of the repository the pull request was submitted from.
    pr_reponame: Option<String>,
    /// When the build is a part of a pull request from a fork,
    /// The number of the pull request this build forms part of.
    pr_number: Option<usize>,
    /// The total number of nodes across which the current test is running.
    node_total: usize,
    /// The index (0-based) of the current node.
    node_index: usize,
    /// The build image this build runs on.
    build_image: String,
    non_exhaustive: (),
}

impl Circle {
    /// Construct this provider's information from the environment.
    pub fn from_env() -> Option<Self> {
        if !(env("CI")? == "true" && env("CIRCLECI")? == "true") {
            return None;
        }

        return Some(Circle {
            project_username: env("CIRCLE_PROJECT_USERNAME")?,
            project_reponame: env("CIRCLE_PROJECT_REPONAME")?,
            branch: env("CIRCLE_BRANCH"),
            tag: env("CIRCLE_TAG"),
            sha1: env("CIRCLE_SHA1")?,
            repository_url: env("CIRCLE_REPOSITORY_URL")?,
            compare_url: env("CIRCLE_COMPARE_URL"),
            build_url: env("CIRCLE_BUILD_URL")?,
            build_num: env("CIRCLE_BUILD_NUM")?.parse().ok()?,
            previous_build_num: env("CIRCLE_PREVIOUS_BUILD_NUM").and_then(|it| it.parse().ok()),
            pull_requests: env("CI_PULL_REQUESTS"),
            pull_request: env("CI_PULL_REQUEST"),
            artifacts: env("CIRCLE_ARTIFACTS")?,
            username: env("CIRCLE_USERNAME")?,
            test_reports: env("CIRCLE_TEST_REPORTS")?,
            pr_username: env("CIRCLE_PR_USERNAME"),
            pr_reponame: env("CIRCLE_PR_REPONAME"),
            pr_number: env("CIRCLE_PR_NUMBER").and_then(|it| it.parse().ok()),
            node_total: env("CIRCLE_NODE_TOTAL")?.parse().ok()?,
            node_index: env("CIRCLE_NODE_INDEX")?.parse().ok()?,
            build_image: env("CIRCLE_BUILD_IMAGE")?,
            non_exhaustive: (),
        });
    }
}

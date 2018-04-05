use std::path::PathBuf;

ci! {
    #[ci(require(CI = "true", CIRCLECI = "true"))]
    /// Circle CI
    ///
    /// # References
    ///
    /// - <https://circleci.com/docs/2.0/env-vars/#circleci-built-in-environment-variables>
    /// - <https://github.com/codecov/codecov-bash/blob/8b76995ad4a95a61cecd4b049a448a402d91d197/codecov#L548-L568>
    pub struct Circle {
        #[ci(env(HOME))]
        /// Your home directory.
        home: PathBuf!,

        #[ci(env(CIRCLE_BRANCH))]
        /// The name of the Git branch currently being built.
        branch: String!,

        #[ci(env(CIRCLE_NODE_TOTAL))]
        /// An integer representing the number of total build instances.
        node_total: u32!,

        #[ci(env(CIRCLE_BUILD_NUM))]
        /// An integer between 0 and (`node_total` - 1) representing a specific build instance.
        node_index: u32!,

        #[ci(env(CIRCLE_PREVIOUS_BUILD_NUM))]
        /// The CircleCI build number.
        build_num: u32!,

        #[ci(env(CIRCLE_PREVIOUS_BUILD_NUM))]
        /// The number of previous builds in the branch.
        previous_build_num: u32!,

        #[ci(env(CIRCLE_BUILD_URL))]
        /// The URL for the current build.
        build_url: String!,

        #[ci(env(CIRCLE_SHA1))]
        /// The SHA1 hash for the current build’s last commit.
        sha1: String!,

        #[ci(env(CIRCLE_USERNAME))]
        /// The GitHub/Bitbucket username of the user who triggered the build.
        username: String!,

        #[ci(env(CIRCLE_JOB))]
        /// The current job’s name.
        job: String!,

        #[ci(env(CIRCLE_WORKING_DIRECTORY))]
        /// The `working_directory` for the current job.
        working_directory: PathBuf!,

        #[ci(env(CIRCLE_COMPARE_URL))]
        /// The GitHub/Bitbucket compare URL between commits in the build.
        compare_url: String!,

        #[ci(env(CIRCLE_REPOSITORY_URL))]
        /// The GitHub/Bitbucket repository URL.
        repository_url: String!,

        #[ci(env(CIRCLE_PR_NUMBER))]
        /// The GitHub/Bitbucket pull request number.
        pr_number: u32!,

        #[ci(env(CIRCLE_PR_REPONAME))]
        /// The GitHub/Bitbucket repository name in which the pull request was made.
        pr_reponame: String!,

        #[ci(env(CIRCLE_PR_USERNAME))]
        /// The GitHub/Bitbucket username of the user who created the pull request.
        pr_username: String!,

        #[ci(env(CIRCLE_PULL_REQUESTS))]
        /// Comma-separated list of URLs of pull requests this build is a part of.
        pull_requests: String!,

        #[ci(env(CIRCLE_PULL_REQUEST))]
        /// If this build is part of only one pull request, its URL will be populated here.
        /// If there was more than one pull request,
        /// it will contain one of the pull request URLs (picked randomly).
        pull_request: String!,

        #[ci(env(CIRCLE_TAG))]
        /// The name of the git tag being tested, e.g. ‘release-v1.5.4’, if the build is running for a tag.
        tag: String?,

        #[ci(env(CIRCLE_PROJECT_USERNAME))]
        /// The username or organization name of the project being tested,
        /// i.e. “foo” in circleci.com/gh/foo/bar/123.
        project_username: String!,

        #[ci(env(CIRCLE_PROJECT_REPONAME))]
        /// The repository name of the project being tested,
        /// i.e. “bar” in circleci.com/gh/foo/bar/123.
        project_reponame: String!,

        #[ci(env(CIRCLE_INTERNAL_TASK_DATA))]
        /// The directory where test timing data can be found.
        internal_task_data: PathBuf!,

        #[ci(env(CIRCLE_STAGE))]
        /// The job being executed. The default 2.0 job is `build` without using Workflows.
        stage: String!,
    }
}

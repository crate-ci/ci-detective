ci! {
    #[ci(require(CI = "true", CIRCLECI = "true"))]
    /// Circle CI
    ///
    /// # References
    ///
    /// - <https://circleci.com/docs/1.0/environment-variables/>
    /// - <https://github.com/codecov/codecov-bash/blob/8b76995ad4a95a61cecd4b049a448a402d91d197/codecov#L548-L568>
    pub struct Circle {
        #[ci(env(CIRCLE_PROJECT_USERNAME))]
        /// The username or organization name of the project being tested,
        /// i.e. `foo` in `circleci.com/gh/foo/bar/123`
        project_username: String!,

        #[ci(env(CIRCLE_PROJECT_REPONAME))]
        /// The repository name of the project being tested,
        /// i.e. `bar` in `circleci.com/gh/foo/bar/123`
        project_reponame: String!,

        #[ci(env(CIRCLE_BRANCH))]
        /// The name of the Git branch being tested, e.g. `master`,
        /// if the build is running for a branch.
        branch: String?,

        #[ci(env(CIRCLE_TAG))]
        /// The name of the git tag being tested, e.g. `release-v1.5.4`,
        /// if the build is running [for a tag](https://circleci.com/docs/1.0/configuration/#tags).
        tag: String?,

        #[ci(env(CIRCLE_SHA1))]
        /// The SHA1 of the commit being tested.
        sha1: String!,

        #[ci(env(CIRCLE_REPOSITORY_URL))]
        /// A link to the homepage for the current repository,
        /// for example, `https://github.com/circleci/frontend`.
        repository_url: String!,

        #[ci(env(CIRCLE_COMPARE_URL))]
        /// A link to GitHub’s comparison view for this push.
        /// Not present for builds that are triggered by GitHub pushes.
        compare_url: String?,

        #[ci(env(CIRCLE_BUILD_URL))]
        /// A permanent link to the current build,
        /// for example, `https://circleci.com/gh/circleci/frontend/933`.
        build_url: String!,

        #[ci(env(CIRCLE_BUILD_NUM))]
        /// The build number, same as in `circleci.com/gh/foo/bar/123`
        build_num: usize!,

        #[ci(env(CIRCLE_PREVIOUS_BUILD_NUM))]
        /// The build number of the previous build, same as in `circleci.com/gh/foo/bar/123`
        previous_build_num: usize?,

        #[ci(env(CIRCLE_PULL_REQUESTS))]
        /// Comma-separated list of pull requests this build is a part of.
        pull_requests: String?,

        #[ci(env(CIRCLE_PULL_REQUEST))]
        /// If this build is part of only one pull request, its URL will be populated here. If there was
        /// more than one pull request, it will contain one of the pull request URLs (picked randomly).
        pull_request: String?,

        #[ci(env(CIRCLE_ARTIFACTS))]
        /// The directory whose contents are automatically saved as
        /// [build artifacts](https://circleci.com/docs/1.0/build-artifacts/).
        artifacts: String!,

        #[ci(env(CIRCLE_USERNAME))]
        /// The GitHub login of the user who either pushed the code
        /// to GitHub or triggered the build from the UI/API.
        username: String!,

        #[ci(env(CIRCLE_TEST_REPORTS))]
        /// The directory whose contents are automatically processed as
        /// [JUnit test metadata](https://circleci.com/docs/1.0/test-metadata/).
        test_reports: String!,

        #[ci(env(CIRCLE_PR_USERNAME))]
        /// When the build is a part of a pull request from a fork,
        /// The username of the owner of the fork.
        pr_username: String?,

        #[ci(env(CIRCLE_PR_REPONAME))]
        /// When the build is a part of a pull request from a fork,
        /// The name of the repository the pull request was submitted from.
        pr_reponame: String?,

        #[ci(env(CIRCLE_PR_NUMBER))]
        /// When the build is a part of a pull request from a fork,
        /// The number of the pull request this build forms part of.
        pr_number: usize?,

        #[ci(env(CIRCLE_NODE_TOTAL))]
        /// The total number of nodes across which the current test is running.
        node_total: usize!,

        #[ci(env(CIRCLE_NODE_INDEX))]
        /// The index (0-based) of the current node.
        node_index: usize!,

        #[ci(env(CIRCLE_BUILD_IMAGE))]
        /// The build image this build runs on.
        build_image: String!,
    }
}

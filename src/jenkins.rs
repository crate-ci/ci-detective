use env;
use std::path::PathBuf;

/// Jenkins CI
///
/// # References
///
/// - <https://wiki.jenkins.io/display/JENKINS/Building+a+software+project#Buildingasoftwareproject-belowJenkinsSetEnvironmentVariables>
/// - <https://github.com/codecov/codecov-bash/blob/8b76995ad4a95a61cecd4b049a448a402d91d197/codecov#L430-L466>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "nightly", non_exhaustive)]
pub struct Jenkins {
    /// The current build number, such as `153`
    pub build_number: usize,
    /// The current build id, such as `2005-08-22_23-59-59`
    /// (`YYYY-MM-DD_hh-mm-ss`, defunct since version 1.597)
    pub build_id: String,
    /// The URL where the results of this build can be found
    /// (e.g. `http://buildserver/jenkins/job/MyJobName/666/`)
    pub build_url: String,
    /// The name of the node the current build is running on. Equals `master` for master node.
    pub node_name: String,
    /// Name of the project of this build.
    /// This is the name you gave your job when you first set it up.
    /// It's the third column of the Jenkins Dashboard main page.
    pub job_name: String,
    /// String of `jenkins-${JOB_NAME}-${BUILD_NUMBER}`.
    /// Convenient to put into a resource file, a jar file, etc for easier identification.
    pub build_tag: String,
    /// Set to the URL of the Jenkins master that's running the build. This value is used
    /// by [Jenkins CLI](https://wiki.jenkins.io/display/JENKINS/Jenkins+CLI) for example
    pub jenkins_url: String,
    /// The unique number that identifies the current executor (among executors of the same machine)
    /// that's carrying out this build. This is the number you see in the "build executor status",
    /// except that the number starts from 0, not 1.
    pub executor_number: String,
    /// The absolute path of the workspace.
    pub workspace: PathBuf,
    /// For Subversion-based projects, this variable contains the revision number of the module.
    /// If you have more than one module specified, this won't be set.
    pub svn_revision: Option<String>,
    /// For CVS-based projects, this variable contains the branch of the module.
    /// If CVS is configured to check out the trunk, this environment variable will not be set.
    pub cvs_branch: Option<String>,
    /// For Git-based projects, this variable contains the Git hash of the commit
    /// checked out for the build (like `ce9a3c1404e8c91be604088670e93434c4253f03`)
    /// (all the `git_*` variables require git plugin)
    pub git_commit: Option<String>,
    /// For Git-based projects, this variable contains the Git url
    /// (like `git@github.com:user/repo.git` or `[https://github.com/user/repo.git]`)
    pub git_url: Option<String>,
    /// For Git-based projects, this variable contains the Git branch
    /// that was checked out for the build (normally `origin/master`)
    pub git_branch: Option<String>,
    /// Jenkins GitHub pull request builder plugin settings
    pub ghprb: Option<GHPRB>,
    non_exhaustive: (),
}

impl Jenkins {
    /// Construct this provider's information from the environment.
    pub fn from_env() -> Option<Self> {
        Some(Jenkins {
            build_number: env("BUILD_NUMBER")?.parse().ok()?,
            build_id: env("BUILD_ID")?,
            build_url: env("BUILD_URL")?,
            node_name: env("NODE_NAME")?,
            job_name: env("JOB_NAME")?,
            build_tag: env("BUILD_TAG")?,
            jenkins_url: env("JENKINS_URL")?,
            executor_number: env("EXECUTOR_NUMBER")?,
            workspace: env("WORKSPACE")?.into(),
            svn_revision: env("SVN_REVISION"),
            cvs_branch: env("CVS_BRANCH"),
            git_commit: env("GIT_COMMIT"),
            git_url: env("GIT_URL"),
            git_branch: env("GIT_BRANCH"),
            ghprb: GHPRB::from_env(),
            non_exhaustive: (),
        })
    }
}

/// Jenkins GitHub pull request builder plugin
///
/// # References
///
/// - <https://wiki.jenkins-ci.org/display/JENKINS/GitHub+pull+request+builder+plugin#GitHubpullrequestbuilderplugin-EnvironmentVariables>
/// - <https://github.com/codecov/codecov-bash/blob/8b76995ad4a95a61cecd4b049a448a402d91d197/codecov#L430-L466>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "nightly", non_exhaustive)]
#[allow(missing_docs)]
pub struct GHPRB {
    pub actual_commit: String,
    pub actual_commit_author: String,
    pub actual_commit_author_email: String,
    pub pull_description: String,
    pub pull_id: String,
    pub pull_link: String,
    pub pull_title: String,
    pub source_branch: String,
    pub target_branch: String,
    pub comment_body: String,
    pub sha1: String,
    non_exhaustive: (),
}

impl GHPRB {
    /// Construct this provider's information from the environment.
    pub fn from_env() -> Option<Self> {
        Some(GHPRB {
            actual_commit: env("ghprbActualCommit")?,
            actual_commit_author: env("ghprbActualCommitAuthor")?,
            actual_commit_author_email: env("ghprbActualCommitAuthorEmail")?,
            pull_description: env("ghprbPullDescription")?,
            pull_id: env("ghprbPullId")?,
            pull_link: env("ghprbPullLink")?,
            pull_title: env("ghprbPullTitle")?,
            source_branch: env("ghprbSourceBranch")?,
            target_branch: env("ghprbTargetBranch")?,
            comment_body: env("ghprbCommentBody")?,
            sha1: env("sha1")?,
            non_exhaustive: (),
        })
    }
}

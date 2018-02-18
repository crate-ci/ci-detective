// Portions of the CI information extraction are based off of codecov/codecov-bash
// Apache License Version 2.0, January 2004
// https://github.com/codecov/codecov-bash/blob/master/LICENSE

// Documentation of individual build environment variables is from the appropriate documentation
// And is copyright the provider under the original license

//! The environment definitions guaranteed by various vendors.
//!
//! No interpretation is done at this level.

use env;
use std::path::PathBuf;
use std::str::FromStr;

/// Jenkins CI
///
/// # References
///
/// - <https://wiki.jenkins.io/display/JENKINS/Building+a+software+project#Buildingasoftwareproject-belowJenkinsSetEnvironmentVariables>
/// - <https://github.com/codecov/codecov-bash/blob/8b76995ad4a95a61cecd4b049a448a402d91d197/codecov#L430-L466>
#[derive(Clone, Debug)]
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
    pub ghprb: Option<JenkinsGHPRB>,
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
            ghprb: JenkinsGHPRB::from_env(),
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
pub struct JenkinsGHPRB {
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

impl JenkinsGHPRB {
    /// Construct this provider's information from the environment.
    pub fn from_env() -> Option<Self> {
        Some(JenkinsGHPRB {
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

/// Travis CI
///
/// # References
///
/// - <https://docs.travis-ci.com/user/environment-variables/#Default-Environment-Variables>
/// - <https://github.com/codecov/codecov-bash/blob/8b76995ad4a95a61cecd4b049a448a402d91d197/codecov#L467-L489>
#[derive(Clone, Debug)]
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
    pub event_type: TravisEventType,
    /// The id of the current job that Travis CI uses internally.
    pub job_id: String,
    /// The number of the current job (for example, `4.1`).
    pub job_number: String,
    /// On multi-OS builds, this value indicates the platform the job is running on.
    pub os: Option<TravisOS>,
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
    pub dart_version: Option<String>,
    pub go_version: Option<String>,
    pub haxe_version: Option<String>,
    pub jdk_version: Option<String>,
    pub julia_version: Option<String>,
    pub node_version: Option<String>,
    pub otp_release: Option<String>,
    pub perl_version: Option<String>,
    pub php_version: Option<String>,
    pub python_version: Option<String>,
    pub r_version: Option<String>,
    pub ruby_version: Option<String>,
    pub rust_version: Option<String>,
    pub scala_version: Option<String>,
    pub xcode_sdk: Option<String>,
    pub xcode_scheme: Option<String>,
    pub xcode_project: Option<String>,
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
            pull_request: env("TRAVIS_PULL_REQUEST").filter(|it| it != "false"),
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
pub enum TravisEventType {
    Push,
    PullRequest,
    Api,
    Cron,
    #[doc(hidden)] __NonExhaustive,
}

impl FromStr for TravisEventType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "push" => Ok(TravisEventType::Push),
            "pull_request" => Ok(TravisEventType::PullRequest),
            "api" => Ok(TravisEventType::Api),
            "cron" => Ok(TravisEventType::Cron),
            _ => Err(())
        }
    }
}

/// On multi-OS builds, this value indicates the platform the job is running on.
/// To be extended in the future.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TravisOS {
    Linux,
    MacOS,
    #[doc(hidden)] __NonExhaustive,
}

impl FromStr for TravisOS {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "linux" => Ok(TravisOS::Linux),
            "osx" => Ok(TravisOS::MacOS),
            _ => Err(())
        }
    }
}

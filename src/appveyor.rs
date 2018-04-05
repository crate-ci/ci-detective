#![allow(missing_docs)]

use env;
use std::path::PathBuf;
use std::str::FromStr;

/// Appveyor CI
///
/// # References
///
/// - <https://docs.travis-ci.com/user/environment-variables/#Default-Environment-Variables>
/// - <https://github.com/codecov/codecov-bash/blob/0c5cebde9e0c640c8f737a843309df15a9509e26/codecov#L658-L668>
#[derive(Clone, Debug)]
pub struct Appveyor {
    pub api_url: String,
    pub account_name: String,
    pub project_id: String,
    pub project_name: String,
    pub project_slug: String,
    pub build_folder: PathBuf,
    pub build_id: String,
    pub build_number: u32,
    pub build_version: String,
    pub build_worker_image: String,
    pub pull_request_number: Option<u32>,
    pub pull_request_title: Option<String>,
    pub pull_request_head_repo_name: Option<String>,
    pub pull_request_head_repo_branch: Option<String>,
    pub pull_request_head_commit: Option<String>,
    pub job_id: String,
    pub job_name: String,
    pub job_number: u32,
    pub repo_provider: RepoProvider,
    pub repo_scm: RepoSCM,
    pub repo_name: String,
    pub repo_branch: String,
    pub repo_tag: bool,
    pub repo_tag_name: Option<String>,
    pub repo_commit: String,
    pub repo_commit_author: String,
    pub repo_commit_author_email: String,
    pub repo_commit_timestamp: String,
    pub repo_commit_message: String,
    pub repo_commit_message_extended: String,
    pub scheduled_build: bool,
    pub forced_build: bool,
    pub re_build: bool,
    pub platform: String,
    pub configuration: String,
    non_exhaustive: (),
}

impl Appveyor {
    /// Construct this provider's information from the environment.
    pub fn from_env() -> Option<Self> {
        if !(env("APPVEYOR")? == "True" && env("CI")? == "True") {
            return None;
        }

        Some(Appveyor {
            api_url: env("APPVEYOR_API_URL")?,
            account_name: env("APPVEYOR_ACCOUNT_NAME")?,
            project_id: env("APPVEYOR_PROJECT_ID")?,
            project_name: env("APPVEYOR_PROJECT_NAME")?,
            project_slug: env("APPVEYOR_PROJECT_SLUG")?,
            build_folder: env("APPVEYOR_BUILD_FOLDER")?.into(),
            build_id: env("APPVEYOR_BUILD_ID")?,
            build_number: env("APPVEYOR_BUILD_NUMBER")?.parse().ok()?,
            build_version: env("APPVEYOR_BUILD_VERSION")?,
            build_worker_image: env("APPVEYOR_BUILD_WORKER_IMAGE")?,
            pull_request_number: env("APPVEYOR_PULL_REQUEST_NUMBER").and_then(|it| it.parse().ok()),
            pull_request_title: env("APPVEYOR_PULL_REQUEST_TITLE"),
            pull_request_head_repo_name: env("APPVEYOR_PULL_REQUEST_HEAD_REPO_NAME"),
            pull_request_head_repo_branch: env("APPVEYOR_PULL_REQUEST_HEAD_REPO_BRANCH"),
            pull_request_head_commit: env("APVEYOR_PULL_REQUEST_HEAD_COMMIT"),
            job_id: env("APPVEYOR_JOB_ID")?,
            job_name: env("APPVEYOR_JOB_NAME")?,
            job_number: env("APPVEYOR_JOB_NUMBER")?.parse().ok()?,
            repo_provider: env("APPVEYOR_REPO_PROVIDER")?.parse().ok()?,
            repo_scm: env("APPVEYOR_REPO_SCM")?.parse().ok()?,
            repo_name: env("APPVEYOR_REPO_NAME")?,
            repo_branch: env("APPVEYOR_REPO_BRANCH")?,
            repo_tag: env("APVEYOR_REPO_TAG")?.parse().ok()?,
            repo_tag_name: env("APPVEYOR_REPO_TAG_NAME"),
            repo_commit: env("APPVEYOR_REPO_COMMIT")?,
            repo_commit_author: env("APPVEYOR_REPO_COMMIT_AUTHOR")?,
            repo_commit_author_email: env("APPVEYOR_REPO_COMMIT_AUTHOR_EMAIL")?,
            repo_commit_timestamp: env("APPVEYOR_REPO_COMMIT_TIMESTAMP")?,
            repo_commit_message: env("APPVEYOR_REPO_COMMIT_MESSAGE")?,
            repo_commit_message_extended: env("APPVEYOR_REPO_COMMIT_MESSAGE_EXTENDED")?,
            scheduled_build: env("APPVEYOR_SCHEDULED_BUILD").is_some(),
            forced_build: env("APPVEYOR_FORCED_BUILD").is_some(),
            re_build: env("APPVEYOR_RE_BUILD").is_some(),
            platform: env("PLATFORM")?,
            configuration: env("CONFIGURATION")?,
            non_exhaustive: (),
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RepoProvider {
    Github,
    BitBucket,
    Kiln,
    Vso,
    Gitlab,
    #[doc(hidden)] __NonExhaustive,
}

impl FromStr for RepoProvider {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "github" => RepoProvider::Github,
            "bitbucket" => RepoProvider::BitBucket,
            "kiln" => RepoProvider::Kiln,
            "vso" => RepoProvider::Vso,
            "gitlab" => RepoProvider::Gitlab,
            _ => return Err(()),
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RepoSCM {
    Git,
    Mercurial,
    #[doc(hidden)] __NonExhaustive,
}

impl FromStr for RepoSCM {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "git" => RepoSCM::Git,
            "mercurial" => RepoSCM::Mercurial,
            _ => return Err(()),
        })
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreatePullRequestRequest {
    pub title: String,
    pub body: String,
    pub head: String,
    pub base: String,
}

#[derive(Debug, Deserialize)]
pub struct PullRequest {
    pub id: u64,
    pub number: u32,
    pub title: String,
    pub body: Option<String>,
    pub html_url: String,
    pub head: BranchInfo,
    pub base: BranchInfo,
}

#[derive(Debug, Deserialize)]
pub struct BranchInfo {
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub sha: String,
    pub repo: Option<Repository>,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u64,
}
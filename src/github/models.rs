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
    pub head: Branch,
    pub base: Branch,
}

#[derive(Debug, Deserialize)]
pub struct Branch {
    pub ref_field: String,
    pub sha: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u64,
}
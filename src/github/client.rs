use crate::{AppResult, AppErrorType, Config};
use crate::error::AppErrorExt;
use crate::github::models::*;
use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT}};

pub struct GitHubClient {
    client: Client,
    token: String,
    repo_owner: String,
    repo_name: String,
}

impl GitHubClient {
    pub fn new(config: &Config) -> AppResult<Self> {
        let mut headers = HeaderMap::new();
        let auth_header = format!("Bearer {}", config.github_token);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_header)
            .with_app_type(AppErrorType::GitHubAuthenticationError)?);
        headers.insert(USER_AGENT, HeaderValue::from_static("jgf-cli"));

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .with_app_type(AppErrorType::GitHubConnectionError)?;

        Ok(GitHubClient {
            client,
            token: config.github_token.clone(),
            repo_owner: config.repo_owner.clone(),
            repo_name: config.repo_name.clone(),
        })
    }

    pub async fn create_pull_request(
        &self,
        title: &str,
        body: &str,
        head_branch: &str,
        base_branch: &str,
    ) -> AppResult<PullRequest> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls",
            self.repo_owner, self.repo_name
        );

        let request = CreatePullRequestRequest {
            title: title.to_string(),
            body: body.to_string(),
            head: head_branch.to_string(),
            base: base_branch.to_string(),
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .with_app_type(AppErrorType::GitHubConnectionError)?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppErrorType::GitHubApiError(
                format!("PR 생성 실패 ({}): {}", status, error_text)
            ).into());
        }

        let pull_request: PullRequest = response.json().await
            .with_app_type(AppErrorType::GitHubApiError("PR 응답 파싱 실패".to_string()))?;

        Ok(pull_request)
    }

    pub async fn get_pull_request(&self, pr_number: u32) -> AppResult<PullRequest> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls/{}",
            self.repo_owner, self.repo_name, pr_number
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .with_app_type(AppErrorType::GitHubConnectionError)?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppErrorType::GitHubApiError(
                format!("PR 조회 실패 ({}): {}", status, error_text)
            ).into());
        }

        let pull_request: PullRequest = response.json().await
            .with_app_type(AppErrorType::GitHubApiError("PR 응답 파싱 실패".to_string()))?;

        Ok(pull_request)
    }
}
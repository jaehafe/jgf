use crate::config::Config;
use crate::error::{AppError, AppResult};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppContext {
    pub config: Arc<Config>,
    pub jira_client: Option<Arc<JiraClient>>,
    pub github_client: Option<Arc<GitHubClient>>,
}

// 임시 구조체 - 추후 실제 구현
pub struct JiraClient {
    pub base_url: String,
    pub username: String,
    pub token: String,
    pub project_key: String,
}

pub struct GitHubClient {
    pub token: String,
    pub owner: String,
    pub repo: String,
}

impl AppContext {
    pub fn new(config: Config) -> Self {
        AppContext {
            config: Arc::new(config),
            jira_client: None,
            github_client: None,
        }
    }
    
    pub fn with_jira_client(mut self, client: JiraClient) -> Self {
        self.jira_client = Some(Arc::new(client));
        self
    }
    
    pub fn with_github_client(mut self, client: GitHubClient) -> Self {
        self.github_client = Some(Arc::new(client));
        self
    }
    
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    pub fn jira_client(&self) -> AppResult<&JiraClient> {
        self.jira_client
            .as_ref()
            .map(|c| c.as_ref())
            .ok_or_else(|| AppError::config_error("Jira 클라이언트가 초기화되지 않았습니다"))
    }
    
    pub fn github_client(&self) -> AppResult<&GitHubClient> {
        self.github_client
            .as_ref()
            .map(|c| c.as_ref())
            .ok_or_else(|| AppError::config_error("GitHub 클라이언트가 초기화되지 않았습니다"))
    }
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        dotenv::dotenv().ok();
        
        Ok(Config {
            jira_url: std::env::var("JIRA_URL")
                .map_err(|_| AppError::config_error("JIRA_URL이 설정되지 않았습니다"))?,
            jira_project: std::env::var("JIRA_PROJECT")
                .map_err(|_| AppError::config_error("JIRA_PROJECT가 설정되지 않았습니다"))?,
            jira_username: std::env::var("JIRA_USERNAME")
                .map_err(|_| AppError::config_error("JIRA_USERNAME이 설정되지 않았습니다"))?,
            jira_token: std::env::var("JIRA_TOKEN")
                .map_err(|_| AppError::config_error("JIRA_TOKEN이 설정되지 않았습니다"))?,
            
            github_token: std::env::var("GITHUB_TOKEN")
                .map_err(|_| AppError::config_error("GITHUB_TOKEN이 설정되지 않았습니다"))?,
            repo_owner: std::env::var("REPO_OWNER")
                .map_err(|_| AppError::config_error("REPO_OWNER가 설정되지 않았습니다"))?,
            repo_name: std::env::var("REPO_NAME")
                .map_err(|_| AppError::config_error("REPO_NAME이 설정되지 않았습니다"))?,
            
            default_branch: std::env::var("DEFAULT_BRANCH")
                .unwrap_or_else(|_| "main".to_string()),
            
            project_name: std::env::var("PROJECT_NAME")
                .unwrap_or_else(|_| "project".to_string()),
        })
    }
    
    pub fn validate(&self) -> AppResult<()> {
        if self.jira_url.is_empty() {
            return Err(AppError::validation_error("JIRA_URL이 비어있습니다"));
        }
        
        if !self.jira_url.starts_with("https://") && !self.jira_url.starts_with("http://") {
            return Err(AppError::validation_error("JIRA_URL은 http:// 또는 https://로 시작해야 합니다"));
        }
        
        if self.github_token.is_empty() {
            return Err(AppError::validation_error("GITHUB_TOKEN이 비어있습니다"));
        }
        
        Ok(())
    }
}
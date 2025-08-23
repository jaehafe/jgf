use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::jira::JiraClient;
use crate::github::GitHubClient;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppContext {
    pub config: Arc<Config>,
    pub jira_client: Option<Arc<JiraClient>>,
    pub github_client: Option<Arc<GitHubClient>>,
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
    
    pub async fn init_clients(mut self) -> AppResult<Self> {
        let jira_client = JiraClient::new(
            self.config.get_jira_base_url(),
            self.config.jira_username.clone(),
            self.config.jira_token.clone(),
            self.config.jira_project.clone(),
        )?;
        self.jira_client = Some(Arc::new(jira_client));
        
        let github_client = GitHubClient::new(&self.config)?;
        self.github_client = Some(Arc::new(github_client));
        
        Ok(self)
    }
}
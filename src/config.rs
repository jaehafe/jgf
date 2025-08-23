use crate::error::{AppError, AppErrorType, AppResult, AppErrorExt};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use colored::Colorize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub jira_url: String,
    pub jira_project: String,
    pub jira_username: String,
    pub jira_token: String,
    
    pub github_token: String,
    pub repo_owner: String,
    pub repo_name: String,
    
    pub default_branch: String,
    
    pub project_name: String,
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        dotenv::dotenv().ok();
        
        Ok(Config {
            jira_url: std::env::var("JIRA_URL")
                .with_app_type(AppErrorType::ConfigError("JIRA_URL이 설정되지 않았습니다".into()))?,
            jira_project: std::env::var("JIRA_PROJECT")
                .with_app_type(AppErrorType::ConfigError("JIRA_PROJECT가 설정되지 않았습니다".into()))?,
            jira_username: std::env::var("JIRA_USERNAME")
                .with_app_type(AppErrorType::ConfigError("JIRA_USERNAME이 설정되지 않았습니다".into()))?,
            jira_token: std::env::var("JIRA_TOKEN")
                .with_app_type(AppErrorType::ConfigError("JIRA_TOKEN이 설정되지 않았습니다".into()))?,
            
            github_token: std::env::var("GITHUB_TOKEN")
                .with_app_type(AppErrorType::ConfigError("GITHUB_TOKEN이 설정되지 않았습니다".into()))?,
            repo_owner: std::env::var("REPO_OWNER")
                .with_app_type(AppErrorType::ConfigError("REPO_OWNER가 설정되지 않았습니다".into()))?,
            repo_name: std::env::var("REPO_NAME")
                .with_app_type(AppErrorType::ConfigError("REPO_NAME이 설정되지 않았습니다".into()))?,
            
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
        
        let jira_url = self.jira_url.trim_end_matches('/');
        if jira_url != self.jira_url {
            println!("💡 {}", "JIRA_URL 끝의 슬래시를 자동으로 제거했습니다".yellow());
        }
        
        if self.github_token.is_empty() {
            return Err(AppError::validation_error("GITHUB_TOKEN이 비어있습니다"));
        }
        
        if !self.jira_project.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) {
            return Err(AppError::validation_error("JIRA_PROJECT는 대문자와 숫자만 포함해야 합니다"));
        }
        
        Ok(())
    }
    
    pub fn check_env_file() -> bool {
        Path::new(".env").exists()
    }
    
    pub fn create_env_template() -> AppResult<()> {
        let env_path = ".env";
        
        if Path::new(env_path).exists() {
            return Err(AppError::validation_error(".env 파일이 이미 존재합니다"));
        }
        
        let template_content = include_str!("../.env.template");
        
        fs::write(env_path, template_content)
            .map_err(|e| AppError::config_error(format!(".env 파일 생성 실패: {}", e)))?;
        
        println!("✅ {}", ".env 파일이 생성되었습니다".green());
        println!("📝 {}", ".env 파일을 편집하여 설정을 완료하세요".yellow());
        
        Ok(())
    }
    
    pub fn get_jira_base_url(&self) -> String {
        self.jira_url.trim_end_matches('/').to_string()
    }
    
    pub fn get_github_repo_url(&self) -> String {
        format!("https://github.com/{}/{}", self.repo_owner, self.repo_name)
    }
    
    pub fn get_jira_ticket_url(&self, ticket_key: &str) -> String {
        format!("{}/browse/{}", self.get_jira_base_url(), ticket_key)
    }
    
    pub fn format_branch_name(&self, ticket_key: &str, _summary: Option<&str>) -> String {
        ticket_key.to_uppercase()
    }
    
    pub fn display_info(&self) {
        println!("\n{}", "📋 현재 설정".bold().cyan());
        println!("  {}: {}", "프로젝트".bold(), self.project_name);
        println!("  {}: {}", "Jira URL".bold(), self.jira_url);
        println!("  {}: {}", "Jira 프로젝트".bold(), self.jira_project);
        println!("  {}: {}/{}", "GitHub".bold(), self.repo_owner, self.repo_name);
        println!("  {}: {}", "기본 브랜치".bold(), self.default_branch);
        println!();
    }
}
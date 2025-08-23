use serde::{Deserialize, Serialize};
use std::fmt;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub struct AppError {
    pub error_type: AppErrorType,
    pub inner: anyhow::Error,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "error", content = "message", rename_all = "snake_case")]
pub enum AppErrorType {
    // System errors
    NotFound(String),
    IoError(String),
    ConfigError(String),
    ValidationError(String),
    
    // Git errors
    GitError(String),
    GitBranchExists,
    GitNoCurrentBranch,
    GitUncommittedChanges,
    
    // Jira errors
    JiraConnectionError,
    JiraAuthenticationError,
    JiraTicketNotFound(String),
    JiraTransitionNotAllowed,
    JiraApiError(String),
    
    // GitHub errors
    GitHubConnectionError,
    GitHubAuthenticationError,
    GitHubRepoNotFound,
    GitHubPrCreateFailed,
    GitHubApiError(String),
    
    // Generic
    Unknown(String),
}

impl<T> From<T> for AppError
where
    T: Into<anyhow::Error>,
{
    fn from(t: T) -> Self {
        let inner = t.into();
        let error_type = AppErrorType::Unknown(format!("{}", inner));
        
        AppError { error_type, inner }
    }
}

impl From<AppErrorType> for AppError {
    fn from(error_type: AppErrorType) -> Self {
        let inner = anyhow::anyhow!("{:?}", error_type);
        AppError { error_type, inner }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error_type {
            AppErrorType::NotFound(msg) => write!(f, "❌ 찾을 수 없음: {}", msg),
            AppErrorType::IoError(msg) => write!(f, "❌ 파일 시스템 오류: {}", msg),
            AppErrorType::ConfigError(msg) => write!(f, "❌ 설정 오류: {}", msg),
            AppErrorType::ValidationError(msg) => write!(f, "❌ 유효성 검사 실패: {}", msg),
            
            AppErrorType::GitError(msg) => write!(f, "❌ Git 오류: {}", msg),
            AppErrorType::GitBranchExists => write!(f, "❌ 브랜치가 이미 존재합니다"),
            AppErrorType::GitNoCurrentBranch => write!(f, "❌ 현재 브랜치를 찾을 수 없습니다"),
            AppErrorType::GitUncommittedChanges => write!(f, "❌ 커밋되지 않은 변경사항이 있습니다"),
            
            AppErrorType::JiraConnectionError => write!(f, "❌ Jira 연결 실패"),
            AppErrorType::JiraAuthenticationError => write!(f, "❌ Jira 인증 실패"),
            AppErrorType::JiraTicketNotFound(ticket) => write!(f, "❌ Jira 티켓을 찾을 수 없음: {}", ticket),
            AppErrorType::JiraTransitionNotAllowed => write!(f, "❌ Jira 상태 변경이 허용되지 않습니다"),
            AppErrorType::JiraApiError(msg) => write!(f, "❌ Jira API 오류: {}", msg),
            
            AppErrorType::GitHubConnectionError => write!(f, "❌ GitHub 연결 실패"),
            AppErrorType::GitHubAuthenticationError => write!(f, "❌ GitHub 인증 실패"),
            AppErrorType::GitHubRepoNotFound => write!(f, "❌ GitHub 저장소를 찾을 수 없습니다"),
            AppErrorType::GitHubPrCreateFailed => write!(f, "❌ PR 생성 실패"),
            AppErrorType::GitHubApiError(msg) => write!(f, "❌ GitHub API 오류: {}", msg),
            
            AppErrorType::Unknown(msg) => write!(f, "❌ 오류: {}", msg),
        }
    }
}

impl AppError {
    pub fn new(error_type: AppErrorType) -> Self {
        error_type.into()
    }
    
    pub fn not_found(msg: impl Into<String>) -> Self {
        AppErrorType::NotFound(msg.into()).into()
    }
    
    pub fn config_error(msg: impl Into<String>) -> Self {
        AppErrorType::ConfigError(msg.into()).into()
    }
    
    pub fn validation_error(msg: impl Into<String>) -> Self {
        AppErrorType::ValidationError(msg.into()).into()
    }
    
    pub fn git_error(msg: impl Into<String>) -> Self {
        AppErrorType::GitError(msg.into()).into()
    }
    
    pub fn jira_api_error(msg: impl Into<String>) -> Self {
        AppErrorType::JiraApiError(msg.into()).into()
    }
    
    pub fn github_api_error(msg: impl Into<String>) -> Self {
        AppErrorType::GitHubApiError(msg.into()).into()
    }
}

// Extension trait for convenient error conversion
pub trait AppErrorExt<T, E> {
    fn with_app_type(self, error_type: AppErrorType) -> AppResult<T>;
}

impl<T, E: Into<anyhow::Error>> AppErrorExt<T, E> for Result<T, E> {
    fn with_app_type(self, error_type: AppErrorType) -> AppResult<T> {
        self.map_err(|error| AppError {
            error_type,
            inner: error.into(),
        })
    }
}
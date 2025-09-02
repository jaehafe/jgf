use crate::error::{AppError, AppErrorType, AppResult, AppErrorExt};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use colored::Colorize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project: String,
    pub jira: JiraConfig,
    pub github: GithubConfig,
    #[serde(rename = "defaultBranch")]
    pub default_branch: String,
    #[serde(rename = "prTemplate", skip_serializing_if = "Option::is_none")]
    pub pr_template: Option<PrTemplate>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JiraConfig {
    pub url: String,
    pub project: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GithubConfig {
    pub owner: String,
    pub repo: String,
}

#[derive(Clone, Debug)]
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
    
    pub project_root: Option<PathBuf>,
    pub pr_template_content: Option<String>,
}

impl Config {
    pub fn load() -> AppResult<Self> {
        let current_dir = env::current_dir()
            .map_err(|e| AppError::config_error(format!("현재 디렉토리를 가져올 수 없습니다: {}", e)))?;
        
        let (config_path, project_root) = Self::find_config_file(&current_dir)?;
        
        if let Some(config_path) = config_path {
            Self::from_project_config(&config_path, project_root)
        } else {
            Self::from_env()
        }
    }
    
    fn find_config_file(start_dir: &Path) -> AppResult<(Option<PathBuf>, Option<PathBuf>)> {
        let mut current = start_dir.to_path_buf();
        
        loop {
            let config_file = current.join("jgf.json");
            
            if config_file.exists() {
                return Ok((Some(config_file), Some(current)));
            }
            
            if !current.pop() {
                break;
            }
        }
        
        Ok((None, None))
    }
    
    fn from_project_config(config_path: &Path, project_root: Option<PathBuf>) -> AppResult<Self> {
        let content = fs::read_to_string(config_path)
            .map_err(|e| AppError::config_error(format!("설정 파일을 읽을 수 없습니다: {}", e)))?;
        
        let mut project_config: ProjectConfig = serde_json::from_str(&content)
            .map_err(|e| AppError::config_error(format!("설정 파일 파싱 실패: {}", e)))?;
        
        if let Some(ref root) = project_root {
            let env_file = root.join(".env");
            if env_file.exists() {
                dotenv::from_path(&env_file).ok();
            }
        }
        
        let jira_username = project_config.jira.username.unwrap_or_else(|| {
            env::var("JIRA_USERNAME").unwrap_or_default()
        });
        
        let jira_token = env::var("JIRA_TOKEN")
            .with_app_type(AppErrorType::ConfigError("JIRA_TOKEN이 .env 파일에 설정되지 않았습니다".into()))?;
        
        let github_token = env::var("GITHUB_TOKEN")
            .with_app_type(AppErrorType::ConfigError("GITHUB_TOKEN이 .env 파일에 설정되지 않았습니다".into()))?;
        
        let mut config = Config {
            jira_url: project_config.jira.url,
            jira_project: project_config.jira.project,
            jira_username,
            jira_token,
            
            github_token,
            repo_owner: project_config.github.owner,
            repo_name: project_config.github.repo,
            
            default_branch: project_config.default_branch,
            
            project_name: project_config.project,
            
            project_root,
            pr_template_content: None,
        };
        
        if let Some(template) = project_config.pr_template {
            if let Some(path) = template.path {
                if let Some(root) = &config.project_root {
                    let template_path = root.join(&path);
                    if template_path.exists() {
                        if let Ok(content) = fs::read_to_string(&template_path) {
                            config.pr_template_content = Some(content);
                        }
                    }
                }
            } else if let Some(content) = template.content {
                config.pr_template_content = Some(content);
            }
        }
        
        Ok(config)
    }
    
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
            
            project_root: None,
            pr_template_content: None,
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
    
    pub fn create_project_template() -> AppResult<()> {
        let config_path = "jgf.json";
        let env_path = ".env";
        
        if Path::new(config_path).exists() {
            return Err(AppError::validation_error("jgf.json 파일이 이미 존재합니다"));
        }
        
        let project_config = ProjectConfig {
            project: "my-project".to_string(),
            jira: JiraConfig {
                url: "https://your-company.atlassian.net".to_string(),
                project: "PROJ".to_string(),
                username: None,
            },
            github: GithubConfig {
                owner: "your-org".to_string(),
                repo: "your-repo".to_string(),
            },
            default_branch: "main".to_string(),
            pr_template: None,
        };
        
        let config_content = serde_json::to_string_pretty(&project_config)
            .map_err(|e| AppError::config_error(format!("설정 파일 생성 실패: {}", e)))?;
        
        fs::write(config_path, config_content)
            .map_err(|e| AppError::config_error(format!("jgf.json 파일 생성 실패: {}", e)))?;
        
        if !Path::new(env_path).exists() {
            let env_content = "# 토큰 정보는 .env 파일에 저장합니다\n\
                               # 이 파일은 .gitignore에 추가하세요\n\n\
                               JIRA_TOKEN=your-jira-api-token\n\
                               GITHUB_TOKEN=your-github-token\n\
                               \n\
                               # Optional: JIRA_USERNAME이 jgf.json에 없을 경우 사용\n\
                               # JIRA_USERNAME=your-email@example.com\n";
            
            fs::write(env_path, env_content)
                .map_err(|e| AppError::config_error(format!(".env 파일 생성 실패: {}", e)))?;
        }
        
        println!("✅ {}", "jgf.json 파일이 생성되었습니다".green());
        println!("✅ {}", ".env 파일이 생성되었습니다".green());
        println!("📝 {}", "두 파일을 편집하여 설정을 완료하세요".yellow());
        println!("⚠️  {}", ".env 파일을 .gitignore에 추가하는 것을 잊지 마세요!".red());
        
        Ok(())
    }
    
    pub fn create_env_template() -> AppResult<()> {
        Self::create_project_template()
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
        if let Some(ref root) = self.project_root {
            println!("  {}: {}", "프로젝트 경로".bold(), root.display());
        }
        println!("  {}: {}", "Jira URL".bold(), self.jira_url);
        println!("  {}: {}", "Jira 프로젝트".bold(), self.jira_project);
        println!("  {}: {}/{}", "GitHub".bold(), self.repo_owner, self.repo_name);
        println!("  {}: {}", "기본 브랜치".bold(), self.default_branch);
        println!();
    }
    
    pub fn get_pr_template(&self) -> Option<String> {
        if let Some(root) = &self.project_root {
            let possible_paths = vec![
                root.join(".github").join("pull_request_template.md"),
                root.join(".github").join("PULL_REQUEST_TEMPLATE.md"),
                root.join("pull_request_template.md"),
                root.join("PULL_REQUEST_TEMPLATE.md"),
                root.join("docs").join("pull_request_template.md"),
                root.join("docs").join("PULL_REQUEST_TEMPLATE.md"),
                root.join(".gitlab").join("merge_request_templates").join("default.md"),
            ];
            
            for path in possible_paths {
                if path.exists() {
                    if let Ok(content) = fs::read_to_string(&path) {
                        return Some(content);
                    }
                }
            }
        }
        
        None
    }
}
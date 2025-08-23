use crate::error::{AppError, AppErrorType, AppResult, AppErrorExt};
use super::models::*;
use base64::Engine;
use reqwest::{header, Client, Response};
use serde_json::json;

pub struct JiraClient {
    pub base_url: String,
    pub username: String,
    pub token: String,
    pub project_key: String,
    client: Client,
}

impl JiraClient {
    pub fn new(base_url: String, username: String, token: String, project_key: String) -> AppResult<Self> {
        let auth_header = format!("{}:{}", username, token);
        let encoded_auth = base64::engine::general_purpose::STANDARD.encode(auth_header);
        
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Basic {}", encoded_auth))
                .map_err(|e| AppError::jira_api_error(format!("인증 헤더 생성 실패: {}", e)))?,
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .with_app_type(AppErrorType::JiraConnectionError)?;
        
        Ok(JiraClient {
            base_url,
            username,
            token,
            project_key,
            client,
        })
    }
    
    async fn handle_response<T: for<'de> serde::Deserialize<'de>>(
        &self, 
        response: Response,
        context: &str
    ) -> AppResult<T> {
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "응답 읽기 실패".to_string());
            
            return match status {
                reqwest::StatusCode::UNAUTHORIZED => {
                    Err(AppError::new(AppErrorType::JiraAuthenticationError))
                }
                reqwest::StatusCode::NOT_FOUND => {
                    Err(AppError::new(AppErrorType::JiraTicketNotFound(format!("{}: 리소스를 찾을 수 없음", context))))
                }
                _ => {
                    Err(AppError::jira_api_error(format!("{}: {} - {}", context, status, error_text)))
                }
            };
        }
        
        response
            .json()
            .await
            .with_app_type(AppErrorType::JiraApiError(format!("{}: JSON 파싱 실패", context)))
    }
    
    pub async fn test_connection(&self) -> AppResult<()> {
        let url = format!("{}/rest/api/3/myself", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .with_app_type(AppErrorType::JiraConnectionError)?;
        
        if response.status().is_success() {
            Ok(())
        } else if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            Err(AppError::new(AppErrorType::JiraAuthenticationError))
        } else {
            Err(AppError::new(AppErrorType::JiraConnectionError))
        }
    }
    
    pub async fn get_issue(&self, issue_key: &str) -> AppResult<Issue> {
        let url = format!("{}/rest/api/3/issue/{}", self.base_url, issue_key);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .with_app_type(AppErrorType::JiraConnectionError)?;
        
        self.handle_response(response, &format!("이슈 조회: {}", issue_key)).await
    }
    
    pub async fn search_assigned_issues(&self, assignee_email: &str, max_results: Option<i32>) -> AppResult<SearchResults> {
        let jql = format!("project = {} AND assignee = \"{}\" ORDER BY priority DESC, updated DESC", 
                         self.project_key, assignee_email);
        
        let url = format!("{}/rest/api/3/search", self.base_url);
        let max_results = max_results.unwrap_or(50);
        
        let body = json!({
            "jql": jql,
            "maxResults": max_results,
            "startAt": 0,
            "fields": [
                "summary", "description", "status", "priority", 
                "assignee", "reporter", "created", "updated",
                "issuetype", "project"
            ]
        });
        
        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .with_app_type(AppErrorType::JiraConnectionError)?;
        
        self.handle_response(response, "할당된 이슈 검색").await
    }
    
    pub async fn search_issues_by_status(&self, status: &str, max_results: Option<i32>) -> AppResult<SearchResults> {
        let jql = format!("project = {} AND status = \"{}\" ORDER BY updated DESC", 
                         self.project_key, status);
        
        let url = format!("{}/rest/api/3/search", self.base_url);
        let max_results = max_results.unwrap_or(50);
        
        let body = json!({
            "jql": jql,
            "maxResults": max_results,
            "startAt": 0,
            "fields": [
                "summary", "description", "status", "priority", 
                "assignee", "reporter", "created", "updated",
                "issuetype", "project"
            ]
        });
        
        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .with_app_type(AppErrorType::JiraConnectionError)?;
        
        self.handle_response(response, &format!("상태별 이슈 검색: {}", status)).await
    }
    
    pub async fn get_transitions(&self, issue_key: &str) -> AppResult<Vec<Transition>> {
        let url = format!("{}/rest/api/3/issue/{}/transitions", self.base_url, issue_key);
        
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .with_app_type(AppErrorType::JiraConnectionError)?;
        
        let transitions: TransitionList = self
            .handle_response(response, &format!("전환 목록 조회: {}", issue_key))
            .await?;
        
        Ok(transitions.transitions)
    }
    
    pub async fn transition_issue(&self, issue_key: &str, transition_id: &str) -> AppResult<()> {
        let url = format!("{}/rest/api/3/issue/{}/transitions", self.base_url, issue_key);
        
        let body = TransitionRequest {
            transition: TransitionId {
                id: transition_id.to_string(),
            },
        };
        
        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .with_app_type(AppErrorType::JiraConnectionError)?;
        
        if response.status().is_success() {
            Ok(())
        } else if response.status() == reqwest::StatusCode::BAD_REQUEST {
            Err(AppError::new(AppErrorType::JiraTransitionNotAllowed))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "응답 읽기 실패".to_string());
            Err(AppError::jira_api_error(format!("상태 전환 실패 ({}): {}", issue_key, error_text)))
        }
    }
    
    pub async fn find_transition_by_name(&self, issue_key: &str, transition_name: &str) -> AppResult<Option<String>> {
        let transitions = self.get_transitions(issue_key).await?;
        
        for transition in transitions {
            if transition.name.to_lowercase() == transition_name.to_lowercase() ||
               transition.to.name.to_lowercase() == transition_name.to_lowercase() {
                return Ok(Some(transition.id));
            }
        }
        
        Ok(None)
    }
    
    pub async fn transition_to_status(&self, issue_key: &str, target_status: &str) -> AppResult<()> {
        let transitions = self.get_transitions(issue_key).await?;
        
        // 가능한 전환 상태 찾기
        let mut found_transition: Option<String> = None;
        let mut available_statuses: Vec<String> = Vec::new();
        
        for transition in &transitions {
            available_statuses.push(format!("{} ({})", transition.name, transition.to.name));
            
            if transition.name.to_lowercase() == target_status.to_lowercase() ||
               transition.to.name.to_lowercase() == target_status.to_lowercase() ||
               (target_status == "In Review" && (transition.to.name == "리뷰 중" || transition.to.name == "검토 중" || transition.to.name == "Review")) {
                found_transition = Some(transition.id.clone());
                break;
            }
        }
        
        if let Some(transition_id) = found_transition {
            self.transition_issue(issue_key, &transition_id).await
        } else {
            let available = if available_statuses.is_empty() {
                "전환 가능한 상태 없음".to_string()
            } else {
                format!("가능한 상태: {}", available_statuses.join(", "))
            };
            
            Err(AppError::jira_api_error(format!(
                "이슈 {}에서 '{}' 상태로 전환할 수 없습니다. {}", 
                issue_key, target_status, available
            )))
        }
    }
}
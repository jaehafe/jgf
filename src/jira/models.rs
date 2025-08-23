use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub key: String,
    pub fields: IssueFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueFields {
    pub summary: String,
    pub description: Option<Value>,
    pub status: Status,
    pub priority: Option<Priority>,
    pub assignee: Option<User>,
    pub reporter: Option<User>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub issuetype: IssueType,
    pub project: Project,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "statusCategory")]
    pub status_category: Option<StatusCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCategory {
    pub id: i32,
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Priority {
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub account_id: Option<String>,
    pub email_address: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueType {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub subtask: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub expand: Option<String>,
    #[serde(rename = "startAt")]
    pub start_at: i32,
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    pub total: i32,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub id: String,
    pub name: String,
    pub to: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionList {
    pub transitions: Vec<Transition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionRequest {
    pub transition: TransitionId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionId {
    pub id: String,
}

impl Issue {
    pub fn format_summary(&self) -> String {
        format!("[{}] {}", self.key, self.fields.summary)
    }
    
    pub fn format_status(&self) -> String {
        self.fields.status.name.clone()
    }
    
    pub fn format_assignee(&self) -> String {
        self.fields.assignee
            .as_ref()
            .and_then(|u| u.display_name.clone())
            .unwrap_or_else(|| "미할당".to_string())
    }
    
    pub fn format_priority(&self) -> String {
        self.fields.priority
            .as_ref()
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "없음".to_string())
    }
}
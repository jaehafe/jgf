use crate::{AppResult, AppContext, Config, utils};
use crate::jira::models::TicketAction;
use colored::Colorize;

pub async fn run(limit: Option<i32>, status_filter: Option<String>, interactive: Option<bool>) -> AppResult<()> {
    let config = Config::from_env()?;
    config.validate()?;
    
    let context = AppContext::new(config).init_clients().await?;
    
    utils::info_message("할당된 티켓을 조회하고 있습니다...");
    
    let issues = if let Some(status) = status_filter {
        context.jira_client()?.search_issues_by_status(&status, limit).await?
    } else {
        context.jira_client()?.search_assigned_issues(&context.config().jira_username, limit).await?
    };
    
    if issues.issues.is_empty() {
        utils::warning_message("조회된 티켓이 없습니다.");
        return Ok(());
    }
    
    utils::success_message(&format!("총 {} 개의 티켓을 찾았습니다.", issues.issues.len()));
    println!();
    
    for (index, issue) in issues.issues.iter().enumerate() {
        let number = format!("[{}]", index + 1);
        let key = format!("{}", issue.key).bold().cyan();
        let summary = &issue.fields.summary;
        let status = match issue.fields.status.name.as_str() {
            "To Do" => issue.fields.status.name.blue(),
            "In Progress" => issue.fields.status.name.yellow(),
            "Done" => issue.fields.status.name.green(),
            "In Review" => issue.fields.status.name.purple(),
            _ => issue.fields.status.name.normal(),
        };
        let assignee = issue.format_assignee();
        let priority = issue.format_priority();
        
        println!("{} {} {}", number.bold(), key, summary);
        println!("   상태: {} | 담당자: {} | 우선순위: {}", status, assignee, priority);
        
        let url = context.config().get_jira_ticket_url(&issue.key);
        println!("   링크: {}", url.dimmed());
        println!();
    }
    
    let is_interactive = interactive.unwrap_or(true);
    
    if !issues.issues.is_empty() && is_interactive {
        println!();
        let should_select = utils::prompt_confirmation("티켓을 선택하여 작업을 시작하시겠습니까?")?;
        
        if should_select {
            let ticket_options: Vec<String> = issues.issues
                .iter()
                .map(|issue| format!("{} - {}", issue.key, issue.fields.summary))
                .collect();
            
            let selected = utils::prompt_select("작업할 티켓을 선택하세요:", ticket_options)?;
            let selected_index = issues.issues
                .iter()
                .position(|issue| format!("{} - {}", issue.key, issue.fields.summary) == selected)
                .unwrap();
            
            let selected_issue = &issues.issues[selected_index];
            
            println!();
            utils::info_message(&format!("선택된 티켓: {}", selected_issue.key));
            
            let actions = vec![
                TicketAction::CreateBranch,
                TicketAction::OpenBrowser,
                TicketAction::Cancel,
            ];
            
            let selected_action = utils::prompt_select("수행할 작업을 선택하세요:", actions)?;
            
            match selected_action {
                TicketAction::CreateBranch => {
                    crate::cli::commands::start::run(selected_issue.key.clone()).await?;
                }
                TicketAction::OpenBrowser => {
                    let url = context.config().get_jira_ticket_url(&selected_issue.key);
                    utils::info_message(&format!("브라우저에서 열기: {}", url));
                    
                    #[cfg(target_os = "macos")]
                    std::process::Command::new("open").arg(&url).spawn().ok();
                    
                    #[cfg(target_os = "linux")]
                    std::process::Command::new("xdg-open").arg(&url).spawn().ok();
                    
                    #[cfg(target_os = "windows")]
                    std::process::Command::new("cmd").args(&["/C", "start", &url]).spawn().ok();
                }
                TicketAction::Cancel => {
                    utils::info_message("작업이 취소되었습니다.");
                }
            }
        }
    }
    
    Ok(())
}
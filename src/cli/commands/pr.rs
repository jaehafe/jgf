use crate::{AppResult, AppContext, Config, git::GitOps, utils};

pub async fn run() -> AppResult<()> {
    let config = Config::from_env()?;
    config.validate()?;
    
    let git_ops = GitOps::open()?;
    let current_branch = git_ops.get_current_branch()?;
    
    if current_branch == config.default_branch {
        utils::error_message(&format!("기본 브랜치({})에서는 PR을 생성할 수 없습니다.", config.default_branch));
        return Ok(());
    }
    
    if !git_ops.is_clean_working_directory()? {
        utils::warning_message("커밋되지 않은 변경사항이 있습니다.");
        let should_continue = utils::prompt_confirmation("계속하시겠습니까?")?;
        if !should_continue {
            return Ok(());
        }
    }
    
    utils::rocket_message(&format!("브랜치 '{}'에서 '{}'으로 PR 생성", current_branch, config.default_branch));
    println!();
    
    let context = AppContext::new(config).init_clients().await?;
    
    let ticket_key = if current_branch.len() >= 6 && current_branch.starts_with("EM-") {
        current_branch.clone()
    } else {
        utils::warning_message("브랜치명에서 티켓 번호를 추출할 수 없습니다.");
        return Ok(());
    };
    
    let spinner = utils::create_spinner(&format!("Jira 티켓 {} 정보 조회 중...", ticket_key));
    
    let (title, body) = match context.jira_client()?.get_issue(&ticket_key).await {
        Ok(issue) => {
            spinner.finish_and_clear();
            let title = format!("[{}] {}", issue.key, issue.fields.summary);
            let jira_url = context.config().get_jira_ticket_url(&issue.key);
            let body = format!(
                "## 관련 티켓\n{}\n\n## 변경사항\n- \n\n## 테스트 방법\n- ",
                jira_url
            );
            (title, body)
        }
        Err(e) => {
            spinner.finish_and_clear();
            utils::warning_message(&format!("티켓 정보 조회 실패: {}", e));
            let title = format!("[{}] 제목을 입력해주세요", ticket_key);
            let jira_url = context.config().get_jira_ticket_url(&ticket_key);
            let body = format!(
                "## 관련 티켓\n{}\n\n## 변경사항\n- \n\n## 테스트 방법\n- ",
                jira_url
            );
            (title, body)
        }
    };
    
    let spinner = utils::create_spinner("GitHub에 PR 생성 중...");
    
    match context.github_client()?.create_pull_request(
        &title,
        &body,
        &current_branch,
        &context.config().default_branch,
    ).await {
        Ok(pr) => {
            spinner.finish_and_clear();
            utils::success_message(&format!("PR이 성공적으로 생성되었습니다! #{}", pr.number));
            utils::info_message(&format!("PR 링크: {}", pr.html_url));
            
            if let Ok(issue) = context.jira_client()?.get_issue(&ticket_key).await {
                if issue.fields.status.name.to_lowercase() == "in progress" || 
                   issue.fields.status.name == "진행 중" {
                    
                    utils::info_message(&format!("현재 티켓 상태: {}", issue.fields.status.name));
                    utils::info_message("PR이 생성되었습니다. PR Merge 시 'Done'으로 변경됩니다.");
                } else if issue.fields.status.name.to_lowercase() == "done" || 
                          issue.fields.status.name == "완료" {
                    utils::info_message("티켓이 이미 'Done' 상태입니다.");
                } else {
                    utils::info_message(&format!("현재 티켓 상태: {}", issue.fields.status.name));
                }
            }
        }
        Err(e) => {
            spinner.finish_and_clear();
            let error_msg = format!("{}", e);
            if error_msg.contains("already exists") {
                utils::warning_message(&format!("브랜치 '{}'에 대한 PR이 이미 존재합니다.", current_branch));
                let pr_url = format!("https://github.com/{}/{}/pulls?q=is%3Apr+head%3A{}", 
                    context.config().repo_owner, 
                    context.config().repo_name, 
                    current_branch);
                utils::info_message(&format!("PR 확인: {}", pr_url));
                
                if let Ok(issue) = context.jira_client()?.get_issue(&ticket_key).await {
                    if issue.fields.status.name.to_lowercase() == "in progress" || 
                       issue.fields.status.name == "진행 중" {
                        
                        utils::info_message(&format!("현재 티켓 상태: {}", issue.fields.status.name));
                        utils::info_message("PR이 이미 생성되어 있습니다. PR Merge시 'Done'으로 변경됩니다.");
                    } else if issue.fields.status.name.to_lowercase() == "done" || 
                              issue.fields.status.name == "완료" {
                        utils::info_message("티켓이 이미 'Done' 상태입니다.");
                    } else {
                        utils::info_message(&format!("현재 티켓 상태: {}", issue.fields.status.name));
                    }
                }
            } else {
                utils::error_message(&format!("PR 생성 실패: {}", e));
            }
        }
    }
    
    Ok(())
}
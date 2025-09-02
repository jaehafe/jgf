use crate::{AppResult, AppContext, Config, git::GitOps, utils};

pub async fn run(ticket: String) -> AppResult<()> {
    let ticket = ticket.trim().to_uppercase();
    
    utils::rocket_message(&format!("티켓 {} 작업을 시작합니다", ticket));
    println!();
    
    let config = Config::load()?;
    config.validate()?;
    
    let git_ops = GitOps::open()?;
    
    let current_branch = git_ops.get_current_branch()?;
    utils::branch_message(&format!("현재 브랜치: {}", current_branch));
    
    if !git_ops.is_clean_working_directory()? {
        utils::error_message("커밋되지 않은 변경사항이 있습니다.");
        utils::info_message("변경사항을 커밋하거나 stash한 후 다시 시도하세요.");
        return Ok(());
    }
    
    let branch_name = config.format_branch_name(&ticket, None);
    
    if git_ops.branch_exists(&branch_name)? {
        utils::warning_message(&format!("브랜치 '{}'가 이미 존재합니다.", branch_name));
        let switch_to_existing = utils::prompt_confirmation("기존 브랜치로 전환하시겠습니까?")?;
        
        if switch_to_existing {
            utils::info_message(&format!("브랜치 '{}'로 전환합니다.", branch_name));
            git_ops.checkout_branch(&branch_name)?;
            utils::success_message(&format!("브랜치 '{}'로 전환되었습니다.", branch_name));
        }
        return Ok(());
    }
    
    let spinner = utils::create_spinner(&format!("기본 브랜치({})에서 최신 변경사항을 가져오는 중...", config.default_branch));
    git_ops.pull_latest(&config.default_branch)?;
    spinner.finish_with_message(format!("최신 변경사항을 가져왔습니다"));
    
    let spinner = utils::create_spinner(&format!("새 브랜치 '{}' 생성 중...", branch_name));
    git_ops.create_and_checkout_branch(&branch_name, &config.default_branch)?;
    spinner.finish_and_clear();
    utils::success_message(&format!("브랜치 '{}'가 생성되고 체크아웃되었습니다", branch_name));
    
    let context = AppContext::new(config).init_clients().await?;
    
    let spinner = utils::create_spinner(&format!("Jira 티켓 {} 정보 조회 중...", ticket));
    
    match context.jira_client()?.get_issue(&ticket).await {
        Ok(issue) => {
            spinner.finish_and_clear();
            utils::ticket_message(&format!("티켓: {}", issue.format_summary()));
            utils::info_message(&format!("상태: {}", issue.fields.status.name));
            
            if issue.fields.status.name.to_lowercase() != "in progress" && 
               issue.fields.status.name != "진행 중" {
                
                let should_update = utils::prompt_confirmation("티켓 상태를 'In Progress'로 변경하시겠습니까?")?;
                
                if should_update {
                    let spinner = utils::create_spinner("티켓 상태를 'In Progress'로 변경 중...");
                    
                    match context.jira_client()?.transition_to_status(&ticket, "In Progress").await {
                        Ok(()) => {
                            spinner.finish_and_clear();
                            utils::success_message("티켓 상태가 'In Progress'로 변경되었습니다");
                        }
                        Err(e) => {
                            spinner.finish_and_clear();
                            utils::warning_message(&format!("상태 변경 실패: {}", e));
                            utils::info_message("수동으로 Jira에서 상태를 변경해주세요");
                        }
                    }
                }
            } else {
                utils::success_message("티켓이 이미 'In Progress' 상태입니다");
            }
            
            let url = context.config().get_jira_ticket_url(&ticket);
            utils::info_message(&format!("티켓 링크: {}", url));
        }
        Err(e) => {
            spinner.finish_and_clear();
            utils::warning_message(&format!("티켓 정보 조회 실패: {}", e));
            utils::info_message("브랜치는 생성되었습니다. 수동으로 Jira 상태를 확인해주세요");
        }
    }
    
    println!();
    utils::sparkle_message(&format!("작업 준비 완료! 브랜치: {}", branch_name));
    
    Ok(())
}
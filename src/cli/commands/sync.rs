use crate::{AppResult, AppContext, Config, git::GitOps, utils};

pub async fn run() -> AppResult<()> {
    let config = Config::from_env()?;
    config.validate()?;
    
    let git_ops = GitOps::open()?;
    let current_branch = git_ops.get_current_branch()?;
    
    utils::rocket_message("머지된 브랜치 동기화 시작");
    println!();
    
    if current_branch != config.default_branch {
        utils::info_message(&format!("기본 브랜치({})로 전환합니다.", config.default_branch));
        git_ops.checkout_branch(&config.default_branch)?;
    }
    
    let spinner = utils::create_spinner("최신 변경사항을 가져오는 중...");
    git_ops.pull_latest(&config.default_branch)?;
    spinner.finish_and_clear();
    
    let branches = git_ops.list_branches()?;
    let ticket_branches: Vec<String> = branches
        .into_iter()
        .filter(|branch| branch.starts_with("EM-") && branch != &config.default_branch)
        .collect();
    
    if ticket_branches.is_empty() {
        utils::info_message("정리할 브랜치가 없습니다.");
        return Ok(());
    }
    
    utils::branch_message(&format!("{}개의 티켓 브랜치를 발견했습니다", ticket_branches.len()));
    
    let context = AppContext::new(config).init_clients().await?;
    
    for branch in ticket_branches {
        println!();
        utils::branch_message(&format!("브랜치 '{}' 확인 중...", branch));
        
        let is_merged = check_if_merged(&git_ops, &branch, &context.config().default_branch)?;
        
        if is_merged {
            utils::success_message(&format!("브랜치 '{}'가 머지되었습니다.", branch));
            
            let should_update_jira = utils::prompt_confirmation(&format!("티켓 {}를 'Done' 상태로 변경하시겠습니까?", branch))?;
            
            if should_update_jira {
                match context.jira_client()?.get_issue(&branch).await {
                    Ok(issue) => {
                        if issue.fields.status.name.to_lowercase() != "done" && 
                           issue.fields.status.name != "완료" {
                            
                            let spinner = utils::create_spinner(&format!("티켓 {} 상태를 'Done'으로 변경 중...", branch));
                            
                            match context.jira_client()?.transition_to_status(&branch, "Done").await {
                                Ok(()) => {
                                    spinner.finish_and_clear();
                                    utils::success_message(&format!("티켓 {} 상태가 'Done'으로 변경되었습니다", branch));
                                }
                                Err(e) => {
                                    spinner.finish_and_clear();
                                    utils::warning_message(&format!("상태 변경 실패: {}", e));
                                }
                            }
                        } else {
                            utils::info_message(&format!("티켓 {}이 이미 'Done' 상태입니다.", branch));
                        }
                    }
                    Err(e) => {
                        utils::warning_message(&format!("티켓 {} 조회 실패: {}", branch, e));
                    }
                }
            }
            
            let should_delete_branch = utils::prompt_confirmation(&format!("로컬 브랜치 '{}'를 삭제하시겠습니까?", branch))?;
            
            if should_delete_branch {
                match delete_branch(&git_ops, &branch) {
                    Ok(()) => {
                        utils::success_message(&format!("브랜치 '{}'가 삭제되었습니다", branch));
                    }
                    Err(e) => {
                        utils::warning_message(&format!("브랜치 삭제 실패: {}", e));
                    }
                }
            }
        } else {
            utils::info_message(&format!("브랜치 '{}'는 아직 머지되지 않았습니다.", branch));
        }
    }
    
    println!();
    utils::sparkle_message("브랜치 동기화 완료!");
    Ok(())
}

fn check_if_merged(git_ops: &GitOps, branch: &str, base_branch: &str) -> AppResult<bool> {
    use std::process::Command;
    use crate::error::AppErrorExt;
    use crate::AppErrorType;
    
    let output = Command::new("git")
        .args(&["merge-base", "--is-ancestor", branch, base_branch])
        .output()
        .with_app_type(AppErrorType::GitError("merge-base 명령 실행 실패".to_string()))?;
    
    Ok(output.status.success())
}

fn delete_branch(git_ops: &GitOps, branch: &str) -> AppResult<()> {
    use std::process::Command;
    use crate::error::AppErrorExt;
    use crate::AppErrorType;
    
    let output = Command::new("git")
        .args(&["branch", "-d", branch])
        .output()
        .with_app_type(AppErrorType::GitError("브랜치 삭제 명령 실행 실패".to_string()))?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(AppErrorType::GitError(format!("브랜치 삭제 실패: {}", error_msg)).into());
    }
    
    Ok(())
}
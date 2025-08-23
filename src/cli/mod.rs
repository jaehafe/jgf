pub mod commands;

use clap::{Parser, Subcommand};
use crate::AppResult;

#[derive(Parser)]
#[command(name = "jgf")]
#[command(about = "Jira Git Flow - Jira와 Git을 연동하는 CLI 도구")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "설정 파일(.env) 생성")]
    Init,
    
    #[command(about = "할당된 티켓 목록 조회")]
    Tickets {
        #[arg(short, long, help = "표시할 최대 티켓 수")]
        limit: Option<i32>,
        
        #[arg(short, long, help = "상태별 필터링 (예: In Progress, Done)")]
        status: Option<String>,
        
        #[arg(short, long, help = "인터랙티브 모드 (기본값: true)")]
        interactive: Option<bool>,
    },
    
    #[command(about = "티켓으로 브랜치 생성 및 In Progress 상태로 변경")]
    Start {
        #[arg(help = "Jira 티켓 번호 (예: EM-100)")]
        ticket: String,
    },
    
    #[command(about = "현재 브랜치로 PR 생성 및 In Review 상태로 변경")]
    Pr,
    
    #[command(about = "머지된 브랜치 확인 및 Done 상태로 변경")]
    Sync,
}

impl Cli {
    pub async fn run(self) -> AppResult<()> {
        match self.command {
            Some(Commands::Init) => commands::init::run().await,
            Some(Commands::Tickets { limit, status, interactive }) => commands::tickets::run(limit, status, interactive).await,
            Some(Commands::Start { ticket }) => commands::start::run(ticket).await,
            Some(Commands::Pr) => commands::pr::run().await,
            Some(Commands::Sync) => commands::sync::run().await,
            None => {
                utils::info_message("사용법: jgf <command>");
                utils::info_message("도움말: jgf --help");
                Ok(())
            }
        }
    }
}

use crate::utils;
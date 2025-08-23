use jgf::{AppContext, AppResult, Config, utils};
use colored::Colorize;

#[tokio::main]
async fn main() -> AppResult<()> {
    println!("{}", "🚀 JGF (Jira Git Flow) CLI".bold().cyan());
    println!("{}", "========================".cyan());
    
    match Config::from_env() {
        Ok(config) => {
            if let Err(e) = config.validate() {
                utils::error_message(&e.to_string());
                return Err(e);
            }
            
            utils::success_message("설정 로드 및 검증 성공");
            config.display_info();
            
            let spinner = utils::create_spinner("클라이언트 초기화 중...");
            let context = AppContext::new(config)
                .init_clients()
                .await?;
            spinner.finish_with_message("✅ 클라이언트 초기화 완료");
            
            utils::success_message("JGF가 준비되었습니다!");
            
            let test_ticket = utils::format_ticket_key("EM", "100");
            utils::info_message(&format!("티켓 예시: {}", test_ticket));
            
            let branch_name = context.config().format_branch_name("EM-100", Some("Add new feature"));
            utils::info_message(&format!("브랜치명 예시: {}", branch_name));
        }
        Err(e) => {
            utils::error_message(&e.to_string());
            utils::warning_message("먼저 'jgf init' 명령어를 실행하여 설정 파일을 생성하세요.");
        }
    }
    
    Ok(())
}
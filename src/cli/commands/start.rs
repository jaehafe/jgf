use crate::{AppResult, utils};

pub async fn run(ticket: String) -> AppResult<()> {
    utils::info_message(&format!("티켓 {} 작업을 시작합니다.", ticket));
    utils::warning_message("이 기능은 아직 구현되지 않았습니다.");
    
    Ok(())
}
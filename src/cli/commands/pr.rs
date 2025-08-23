use crate::{AppResult, utils};

pub async fn run(title: Option<String>, description: Option<String>) -> AppResult<()> {
    utils::info_message("PR을 생성합니다.");
    
    if let Some(title) = title {
        utils::info_message(&format!("제목: {}", title));
    }
    
    if let Some(description) = description {
        utils::info_message(&format!("설명: {}", description));
    }
    
    utils::warning_message("이 기능은 아직 구현되지 않았습니다.");
    
    Ok(())
}
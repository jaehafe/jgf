use crate::{AppResult, Config, utils};

pub async fn run() -> AppResult<()> {
    utils::info_message("JGF 초기 설정을 시작합니다.");
    
    Config::create_project_template()?;
    
    utils::success_message("초기화가 완료되었습니다!");
    utils::info_message("다음 단계:");
    utils::info_message("1. jgf.json 파일을 편집하여 프로젝트 정보를 입력하세요");
    utils::info_message("2. .env 파일을 편집하여 토큰 정보를 입력하세요");
    utils::info_message("3. jgf tickets 명령어로 티켓을 확인해보세요");
    
    Ok(())
}
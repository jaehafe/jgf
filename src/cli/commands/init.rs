use crate::{AppResult, Config, utils};

pub async fn run() -> AppResult<()> {
    utils::info_message("JGF 초기 설정을 시작합니다.");
    
    if Config::check_env_file() {
        utils::warning_message(".env 파일이 이미 존재합니다.");
        let overwrite = utils::prompt_confirmation("덮어쓰시겠습니까?")?;
        
        if !overwrite {
            utils::info_message("초기화가 취소되었습니다.");
            return Ok(());
        }
    }
    
    Config::create_env_template()?;
    
    utils::success_message("초기화가 완료되었습니다!");
    utils::info_message("다음 단계:");
    utils::info_message("1. .env 파일을 편집하여 설정값을 입력하세요");
    utils::info_message("2. jgf tickets 명령어로 티켓을 확인해보세요");
    
    Ok(())
}
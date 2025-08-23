use crate::error::{AppError, AppErrorExt, AppResult};
use colored::Colorize;
use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use crate::AppErrorType;

static ROCKET: Emoji<'_, '_> = Emoji("🚀  ", "");
static PACKAGE: Emoji<'_, '_> = Emoji("📦  ", "");
static GEAR: Emoji<'_, '_> = Emoji("⚙️   ", "");
static LINK: Emoji<'_, '_> = Emoji("🔗  ", "");
static BRANCH: Emoji<'_, '_> = Emoji("🌿  ", "");
static TICKET: Emoji<'_, '_> = Emoji("🎫  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨  ", "");
static CHECK: Emoji<'_, '_> = Emoji("✅  ", "");
static CROSS: Emoji<'_, '_> = Emoji("❌  ", "");
static WARNING: Emoji<'_, '_> = Emoji("⚠️   ", "");
static INFO: Emoji<'_, '_> = Emoji("💡  ", "");
static SYNC: Emoji<'_, '_> = Emoji("🔄  ", "");
static TRASH: Emoji<'_, '_> = Emoji("🗑️   ", "");

pub fn create_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner
}

pub fn success_message(message: &str) {
    println!("{}{}", CHECK, message.green());
}

pub fn error_message(message: &str) {
    println!("{}{}", CROSS, message.red());
}

pub fn info_message(message: &str) {
    println!("{}{}", INFO, message.cyan());
}

pub fn warning_message(message: &str) {
    println!("{}{}", WARNING, message.yellow());
}

pub fn step_message(step: &str, total: &str, message: &str) {
    println!(
        "{} {}{}",
        style(format!("[{}/{}]", step, total)).bold().dim(),
        GEAR,
        message
    );
}

pub fn rocket_message(message: &str) {
    println!("{}{}", ROCKET, message.bold());
}

pub fn ticket_message(message: &str) {
    println!("{}{}", TICKET, message);
}

pub fn branch_message(message: &str) {
    println!("{}{}", BRANCH, message);
}

pub fn sparkle_message(message: &str) {
    println!("{}{}", SPARKLE, message.green().bold());
}

pub fn prompt_confirmation(message: &str) -> AppResult<bool> {
    let answer = inquire::Confirm::new(message)
        .with_default(false)
        .prompt()
        .with_app_type(AppErrorType::IoError("입력 오류 발생".to_string()))?;
    
    Ok(answer)
}

pub fn prompt_text(message: &str, default: Option<&str>) -> AppResult<String> {
    let mut prompt = inquire::Text::new(message);
    
    if let Some(default_value) = default {
        prompt = prompt.with_default(default_value);
    }
    
    prompt
        .prompt()
        .map_err(|e| AppError::validation_error(format!("입력 오류: {}", e)))
}

pub fn prompt_select<T: std::fmt::Display>(
    message: &str,
    options: Vec<T>,
) -> AppResult<T> {
    inquire::Select::new(message, options)
        .prompt()
        .map_err(|e| AppError::validation_error(format!("선택 오류: {}", e)))
}

pub fn format_ticket_key(project: &str, number: &str) -> String {
    if number.to_uppercase().starts_with(project) {
        number.to_uppercase()
    } else {
        format!("{}-{}", project, number)
    }
}

pub fn validate_ticket_key(key: &str) -> bool {
    let parts: Vec<&str> = key.split('-').collect();
    if parts.len() != 2 {
        return false;
    }
    
    parts[0].chars().all(|c| c.is_ascii_uppercase()) &&
    parts[1].chars().all(|c| c.is_ascii_digit())
}
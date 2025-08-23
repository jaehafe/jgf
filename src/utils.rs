use crate::error::{AppError, AppResult};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn create_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"]),
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner
}

pub fn success_message(message: &str) {
    println!("✅ {}", message.green());
}

pub fn error_message(message: &str) {
    println!("❌ {}", message.red());
}

pub fn info_message(message: &str) {
    println!("ℹ️  {}", message.blue());
}

pub fn warning_message(message: &str) {
    println!("⚠️  {}", message.yellow());
}

pub fn prompt_confirmation(message: &str) -> AppResult<bool> {
    let answer = inquire::Confirm::new(message)
        .with_default(false)
        .prompt()
        .map_err(|e| AppError::validation_error(format!("입력 오류: {}", e)))?;
    
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
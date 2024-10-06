extern crate notify_rust;
extern crate reqwest;

#[macro_use]
extern crate serde_json;

mod rofi;
mod todoist;

use std::{env, fs, path::PathBuf, process::exit};

use notify_rust::Notification;

fn main() {
    let todoist_api_token = match get_todoist_api_token() {
        Ok(todoist_api_token) => todoist_api_token,
        Err(err) => {
            println!("Error: {}", err);
            Notification::new()
                .summary("rofi-todoist")
                .body(&format!("Error: {}", err))
                .show()
                .expect("Failed to show notification");
            exit(1);
        }
    };

    dbg!(&todoist_api_token);

    let option = match rofi::get_text("Add task") {
        Some(text) => text,
        None => {
            println!("No text, exiting");
            exit(1);
        }
    };

    let response_message = match todoist::create_task(option, todoist_api_token) {
        Ok(task_name) => format!("Created task '{}' successfully", task_name),
        Err(err) => format!("Error: {}", err),
    };

    Notification::new()
        .summary("rofi-todoist")
        .body(&response_message)
        .show()
        .expect("Failed to show notification");
}

fn get_todoist_api_token() -> Result<String, String> {
    if let Ok(token) = env::var("TODOIST_API_TOKEN") {
        if !token.is_empty() {
            return Ok(token);
        }
    }

    // If not found in environment variables, try to read from the file ~/.config/todoist
    let config_path = PathBuf::from(env::var("HOME").unwrap()).join(".config/todoist");
    if let Ok(content) = fs::read_to_string(config_path) {
        let token = content.trim().to_string();
        if !token.is_empty() {
            return Ok(token);
        }
    }

    Err("TODOIST_API_TOKEN is not set and ~/.config/todoist is missing or empty".into())
}

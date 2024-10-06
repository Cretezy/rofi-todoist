use std::process::{Command, Stdio};

pub fn get_text<T: AsRef<str>>(prompt: T) -> Option<String> {
    let command = Command::new("rofi")
        .arg("-dmenu")
        .args(["-lines", "0"])
        .args(["-l", "0"])
        .args(["-p", prompt.as_ref()])
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to run rofi");

    let result = String::from_utf8_lossy(&command.stdout).replace("\n", "");

    if result.is_empty() {
        return None;
    }

    Some(result)
}

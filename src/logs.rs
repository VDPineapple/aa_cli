use std::env;
use std::fs;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Attribute, SetAttribute},
};

pub fn new_line() {
    execute!(std::io::stdout(), Print("\n")).unwrap();
}

pub fn write(color: &Color, bold: bool, text: &str) {
    execute!(
        std::io::stdout(),
        if bold { SetAttribute(Attribute::Bold) } else { SetAttribute(Attribute::Reset) },
        SetForegroundColor(*color),
        Print(text),
        ResetColor
    ).unwrap();
}

pub fn write_line(color: &Color, bold: bool, text: &str) {
    write(color, bold, text);
    execute!(std::io::stdout(), Print("\n")).unwrap();
}

pub fn print_logs(logs: &Vec<String>) {
    let orange = Color::Rgb { r: 255, g: 165, b: 0 };
    for log in logs {
        write_line(&orange, false, format!("{}", log).as_str());
    }
}

pub fn read_logs() -> Vec<String> {
    let dir = env::current_dir().unwrap();
    let file = match fs::read_to_string(format!("{}/aa_cli_logs.txt", dir.to_str().unwrap())) {
        Ok(f) => f,
        Err(_) => String::new(),
    };
    let logs: Vec<String> = file.split("\n").map(|s| s.to_string()).collect();
    logs
}

pub fn write_logs(logs: &Vec<String>) {
    let dir = env::current_dir().unwrap();
    fs::write(format!("{}/aa_cli_logs.txt", dir.to_str().unwrap()), logs.join("\n")).unwrap();
}

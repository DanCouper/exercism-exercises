// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

fn prefix(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Info => "[INFO]: ",
        LogLevel::Warning => "[WARNING]: ",
        LogLevel::Error => "[ERROR]: ",
        LogLevel::Debug => "[DEBUG]: ",
    }
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let mut msg = String::new();
    msg.push_str(prefix(level));
    msg.push_str(message);
    msg
}

pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}

pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}

pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}

pub fn debug(message: &str) -> String {
    log(LogLevel::Debug, message)
}

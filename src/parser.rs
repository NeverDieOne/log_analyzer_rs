use chrono::DateTime;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat(String),
    InvalidTimestamp(String),
    InvalidLevel(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidLevel(level) => write!(f, "Invalid log level: '{}'", level),
            ParseError::InvalidTimestamp(value) => write!(f, "Invalid date format: '{}'", value),
            ParseError::InvalidFormat(value) => write!(f, "Invalid log format: '{}'", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
pub struct LogEntry {
    pub timestamp: DateTime<chrono::Utc>,
    pub level: LogLevel,
    pub service: String,
    pub message: String,
}

pub fn parse_log_line(line: &str) -> Result<LogEntry, ParseError> {
    let mut parts = line.splitn(4, ' ');

    let ts_str = parts.next().ok_or(ParseError::InvalidFormat(line.to_string()))?;
    let level_str = parts.next().ok_or(ParseError::InvalidFormat(line.to_string()))?;
    let service = parts.next().ok_or(ParseError::InvalidFormat(line.to_string()))?;
    let message = parts.next().ok_or(ParseError::InvalidFormat(line.to_string()))?;

    let level = match level_str {
        "INFO" => LogLevel::Info,
        "WARN" => LogLevel::Warning,
        "ERROR" => LogLevel::Error,
        other => return Err(ParseError::InvalidLevel(other.to_string())),
    };

    let timestamp = match DateTime::parse_from_rfc3339(ts_str) {
        Ok(dt) => dt.with_timezone(&chrono::Utc),
        Err(_) => return Err(ParseError::InvalidTimestamp(ts_str.to_string())),
    };

    Ok(LogEntry {
        timestamp: timestamp,
        level: level,
        service: service.to_string(),
        message: message.to_string(),
    })
}

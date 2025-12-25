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
    let parts = line.splitn(4, ' ').collect::<Vec<&str>>();
    if parts.len() != 4 {
        return Err(ParseError::InvalidFormat(line.to_string()));
    }

    let level = match parts[1] {
        "INFO" => LogLevel::Info,
        "WARN" => LogLevel::Warning,
        "ERROR" => LogLevel::Error,
        other => return Err(ParseError::InvalidLevel(other.to_string())),
    };

    let timestamp = match DateTime::parse_from_rfc3339(parts[0]) {
        Ok(dt) => dt.with_timezone(&chrono::Utc),
        Err(err) => return Err(ParseError::InvalidTimestamp(err.to_string())),
    };

    Ok(LogEntry {
        timestamp: timestamp,
        level: level,
        service: parts[2].to_string(),
        message: parts[3].to_string(),
    })
}

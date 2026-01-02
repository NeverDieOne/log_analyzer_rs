use chrono::DateTime;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    Format(String),
    Timestamp(String),
    Level(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Level(level) => write!(f, "Invalid log level: '{level}'"),
            ParseError::Timestamp(value) => write!(f, "Invalid date format: '{value}'"),
            ParseError::Format(value) => write!(f, "Invalid log format: '{value}'"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LogEntry {
    pub timestamp: DateTime<chrono::Utc>,
    pub level: LogLevel,
    pub service: String,
    pub message: String,
}

pub fn parse_log_line(line: &str) -> Result<LogEntry, ParseError> {
    let mut parts = line.splitn(4, ' ');

    let ts_str = parts.next().ok_or(ParseError::Format(line.to_string()))?;
    let level_str = parts.next().ok_or(ParseError::Format(line.to_string()))?;
    let service = parts.next().ok_or(ParseError::Format(line.to_string()))?;
    let message = parts.next().ok_or(ParseError::Format(line.to_string()))?;

    let level = match level_str {
        "INFO" => LogLevel::Info,
        "WARN" => LogLevel::Warning,
        "ERROR" => LogLevel::Error,
        other => return Err(ParseError::Level(other.to_string())),
    };

    let timestamp = match DateTime::parse_from_rfc3339(ts_str) {
        Ok(dt) => dt.with_timezone(&chrono::Utc),
        Err(_) => return Err(ParseError::Timestamp(ts_str.to_string())),
    };

    Ok(LogEntry {
        timestamp,
        level,
        service: service.to_string(),
        message: message.to_string(),
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_line_valid() {
        let line = "2024-01-01T12:00:00Z INFO auth User logged in";
        let entry = parse_log_line(line).unwrap();
        let expected_entry = LogEntry {
            timestamp: "2024-01-01T12:00:00Z".parse().unwrap(),
            level: LogLevel::Info,
            service: "auth".to_string(),
            message: "User logged in".to_string(),
        };
        assert_eq!(entry, expected_entry)
    }

    #[test]
    fn test_parse_log_line_invalid() {
        let line = "Invalid log line";
        let err = parse_log_line(line).unwrap_err();
        match err {
            ParseError::Format(_) => (),
            _ => panic!("Expected Format error"),
        }
    }

    #[test]
    fn test_parse_log_line_invalid_log_level() {
        let line = "2024-01-01T12:00:00Z WARNING auth User logged in";
        let err = parse_log_line(line).unwrap_err();
        match err {
            ParseError::Level(_) => (),
            _ => panic!("Expected Level error"),
        }
    }

    #[test]
    fn test_parse_log_line_invalid_timestamp() {
        let line = "2024-01-01 WARN auth User logged in";
        let err = parse_log_line(line).unwrap_err();
        match err {
            ParseError::Timestamp(_) => (),
            _ => panic!("Expected Timestamp error"),
        }
    }
}
use crate::parser::{LogEntry, LogLevel};
use crate::Args;
use chrono::DateTime;
use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    InvalidLevel(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidLevel(level) => write!(f, "Invalid log level: '{}'", level),
        }
    }
}

#[derive(Debug)]
pub struct Filters {
    level: Option<LogLevel>,
    service: Option<String>,
    contains: Option<String>,
    from: Option<DateTime<chrono::Utc>>,
    to: Option<DateTime<chrono::Utc>>,
}

impl TryFrom<Args> for Filters {
    type Error = ConfigError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let level = match args.level.as_deref() {
            Some("info") => Some(LogLevel::Info),
            Some("warn") => Some(LogLevel::Warning),
            Some("error") => Some(LogLevel::Error),
            Some(other) => return Err(ConfigError::InvalidLevel(other.to_string())),
            None => None,
        };

        Ok(Filters {
            level,
            service: args.service,
            contains: args.contains,
            from: args.from,
            to: args.to,
        })
    }
}

pub fn match_filters(log_entry: &LogEntry, filters: &Filters) -> bool {
    if let Some(level) = filters.level.as_ref() {
        if level != &log_entry.level {
            return false;
        }
    }

    if let Some(service) = filters.service.as_ref() {
        if service != &log_entry.service {
            return false;
        }
    };

    if let Some(contains) = filters.contains.as_ref() {
        if !log_entry.message.contains(contains) {
            return false;
        }
    };

    if let Some(from) = filters.from {
        if log_entry.timestamp < from {
            return false;
        }
    };

    if let Some(to) = filters.to {
        if log_entry.timestamp > to {
            return false;
        }
    };

    true
}

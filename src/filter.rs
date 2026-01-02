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
            ConfigError::InvalidLevel(level) => write!(f, "Invalid log level: '{level}'"),
        }
    }
}

#[derive(Debug, PartialEq)]
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
    if let Some(level) = &filters.level && level != &log_entry.level {
        return false;
    }

    if let Some(service) = &filters.service && service != &log_entry.service {
        return false;
    }

    if let Some(contains) = &filters.contains && !log_entry.message.contains(contains) {
        return false;
    }

    if let Some(from) = filters.from && log_entry.timestamp < from {
        return false;
    }

    if let Some(to) = filters.to && log_entry.timestamp > to {
         return false;
    }
    
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filters_from_args_valid() {
        let args = Args {
            file: "".to_string(),
            level: None,
            service: None,
            contains: None,
            from: None,
            to: None,
            json: false,
            count: false,
        };
        let filters = Filters::try_from(args).expect("Can not parse filters");
        let expected_filters = Filters {
            level: None,
            service: None,
            contains: None,
            from: None,
            to: None,
        };
        assert_eq!(filters, expected_filters);
    }
}

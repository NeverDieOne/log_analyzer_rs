use crate::cli::Args;
use crate::core::parser::{LogEntry, LogLevel};
use chrono::DateTime;
use std::fmt;

#[derive(Debug)]
pub enum FilterError {
    InvalidLevel(String),
}

impl fmt::Display for FilterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilterError::InvalidLevel(level) => write!(f, "Invalid log level: '{level}'"),
        }
    }
}

impl std::error::Error for FilterError {}

#[derive(Debug, PartialEq, Default)]
pub struct Filters {
    level: Option<LogLevel>,
    service: Option<String>,
    contains: Option<String>,
    from: Option<DateTime<chrono::Utc>>,
    to: Option<DateTime<chrono::Utc>>,
}

impl TryFrom<Args> for Filters {
    type Error = FilterError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let level = match args.level.as_deref() {
            Some("info") => Some(LogLevel::Info),
            Some("warn") => Some(LogLevel::Warning),
            Some("error") => Some(LogLevel::Error),
            Some(other) => return Err(FilterError::InvalidLevel(other.to_string())),
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
    if let Some(level) = &filters.level
        && level != &log_entry.level
    {
        return false;
    }

    if let Some(service) = &filters.service
        && service != &log_entry.service
    {
        return false;
    }

    if let Some(contains) = &filters.contains
        && !log_entry.message.contains(contains)
    {
        return false;
    }

    if let Some(from) = filters.from
        && log_entry.timestamp < from
    {
        return false;
    }

    if let Some(to) = filters.to
        && log_entry.timestamp > to
    {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filters_invalid_level() {
        let args = Args {
            level: Some("warning".to_string()),
            ..Default::default()
        };
        let err = Filters::try_from(args).unwrap_err();
        matches!(err, FilterError::InvalidLevel(_));
    }

    #[test]
    fn test_match_level() {
        let filters = Filters {
            level: Some(LogLevel::Error),
            ..Default::default()
        };

        let log_entry = LogEntry {
            level: LogLevel::Info,
            service: "auth".to_string(),
            message: "User logged in".to_string(),
            timestamp: chrono::Utc::now(),
        };
        assert!(!match_filters(&log_entry, &filters));

        let log_entry_error = LogEntry {
            level: LogLevel::Error,
            service: "auth".to_string(),
            message: "Failed login attempt".to_string(),
            timestamp: chrono::Utc::now(),
        };
        assert!(match_filters(&log_entry_error, &filters));
    }

    #[test]
    fn test_match_service() {
        let filters = Filters {
            service: Some("payment".to_string()),
            ..Default::default()
        };
        let log_entry = LogEntry {
            level: LogLevel::Info,
            service: "auth".to_string(),
            message: "User logged in".to_string(),
            timestamp: chrono::Utc::now(),
        };
        assert!(!match_filters(&log_entry, &filters));

        let log_entry_payment = LogEntry {
            level: LogLevel::Info,
            service: "payment".to_string(),
            message: "Payment processed".to_string(),
            timestamp: chrono::Utc::now(),
        };
        assert!(match_filters(&log_entry_payment, &filters));
    }

    #[test]
    fn test_match_contains() {
        let filters = Filters {
            contains: Some("error".to_string()),
            ..Default::default()
        };
        let log_entry = LogEntry {
            level: LogLevel::Info,
            service: "auth".to_string(),
            message: "User logged in".to_string(),
            timestamp: chrono::Utc::now(),
        };
        assert!(!match_filters(&log_entry, &filters));

        let log_entry_error = LogEntry {
            level: LogLevel::Error,
            service: "auth".to_string(),
            message: "An error occurred".to_string(),
            timestamp: chrono::Utc::now(),
        };
        assert!(match_filters(&log_entry_error, &filters));
    }

    #[test]
    fn test_match_time_range() {
        let from = chrono::Utc::now() - chrono::Duration::hours(1);
        let to = chrono::Utc::now() + chrono::Duration::hours(1);
        let filters = Filters {
            from: Some(from),
            to: Some(to),
            ..Default::default()
        };

        let log_entry_outside = LogEntry {
            level: LogLevel::Info,
            service: "auth".to_string(),
            message: "Old log entry".to_string(),
            timestamp: chrono::Utc::now() - chrono::Duration::hours(2),
        };
        assert!(!match_filters(&log_entry_outside, &filters));

        let log_entry_inside = LogEntry {
            level: LogLevel::Info,
            service: "auth".to_string(),
            message: "Recent log entry".to_string(),
            timestamp: chrono::Utc::now(),
        };
        assert!(match_filters(&log_entry_inside, &filters));
    }
}

use crate::core::consumer::Consumer;
use crate::core::output::Output;
use crate::core::parser::LogEntry;

pub struct TextConsumer;

impl Consumer for TextConsumer {
    fn consume(&mut self, entry: &LogEntry) -> Vec<Output> {
        vec![Output::Line(format!(
            "{} [{}] {}: {}\n",
            entry.timestamp.to_rfc3339(),
            entry.level.as_str(),
            entry.service,
            entry.message
        ))]
    }

    fn finalize(&mut self) -> Vec<Output> {
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parser::LogLevel;
    
    fn sample_entry() -> LogEntry {
        LogEntry {
            timestamp: "2024-01-01T12:00:00Z".parse().unwrap(),
            level: LogLevel::Info,
            service: "auth".to_string(),
            message: "User logged in".to_string(),
        }
    }

    #[test]
    fn text_consumer_emits_single_line() {
        let mut consumer = TextConsumer;

        let entry = LogEntry {
            timestamp: "2024-01-01T12:00:00Z".parse().unwrap(),
            level: LogLevel::Info,
            service: "auth".to_string(),
            message: "User logged in".to_string(),
        };

        let output = consumer.consume(&entry);

        assert_eq!(output.len(), 1);
    }

    #[test]
    fn text_consumer_output_contains_expected_fields() {
        let mut consumer = TextConsumer;

        let entry = sample_entry();

        let output = consumer.consume(&entry);
        let line = match &output[0] {
            Output::Line(s) => s,
            _ => panic!("Expected Output::Line"),
        };

        assert!(line.contains("INFO"));
        assert!(line.contains("auth"));
        assert!(line.contains("User logged in"));
    }

    #[test]
    fn text_consumer_ends_line_with_newline() {
        let mut consumer = TextConsumer;

        let entry = sample_entry();

        let output = consumer.consume(&entry);
        let line = match &output[0] {
            Output::Line(s) => s,
            _ => panic!("Expected Output::Line"),
        };

        assert!(line.ends_with('\n'));
    }

    #[test]
    fn text_consumer_finalize_is_empty() {
        let mut consumer = TextConsumer;

        let output = consumer.finalize();

        assert!(output.is_empty());
    }
}

use crate::core::consumer::{Consumer, ConsumerError};
use crate::core::output::Output;
use crate::core::parser::LogEntry;

pub struct TextConsumer;

impl Consumer for TextConsumer {
    fn consume(&mut self, entry: &LogEntry) -> Result<Vec<Output>, ConsumerError> {
        Ok(vec![Output::Line(format!(
            "{} [{}] {}: {}\n",
            entry.timestamp.to_rfc3339(),
            entry.level.as_str(),
            entry.service,
            entry.message
        ))])
    }

    fn finalize(&mut self) -> Result<Vec<Output>, ConsumerError> {
        Ok(vec![])
    }
}

use crate::output::Output;
use crate::parser::LogEntry;
use std::fmt;

#[derive(Debug)]
pub struct ConsumerError;

impl fmt::Display for ConsumerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Consumer error")
    }
}

impl std::error::Error for ConsumerError {}

pub trait Consumer {
    fn consume(&mut self, entry: &LogEntry) -> Result<Vec<Output>, ConsumerError>;
    fn finalize(&mut self) -> Result<Vec<Output>, ConsumerError>;
}

pub struct TextConsumer;

impl Consumer for TextConsumer {
    fn consume(&mut self, entry: &LogEntry) -> Result<Vec<Output>, ConsumerError> {
        Ok(vec![Output::Line(format!(
            "{} [{}] {}: {}",
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

pub struct JsonConsumer {
    first: bool,
}

impl JsonConsumer {
    pub fn new() -> JsonConsumer {
        JsonConsumer { first: true }
    }
}

impl Consumer for JsonConsumer {
    fn consume(&mut self, entry: &LogEntry) -> Result<Vec<Output>, ConsumerError> {
        let mut out = vec![];

        if self.first {
            out.push(Output::Begin);
            self.first = false;
        } else {
            out.push(Output::Line(",\n".to_string()));
        }

        out.push(Output::Line(
            format!(
                "    {{\"timestamp\": \"{}\", \"level\": \"{}\", \"service\": \"{}\", \"message\": \"{}\"}}",
                entry.timestamp.to_rfc3339(),
                entry.level.as_str(),
                entry.service,
                entry.message
            )
        ));

        Ok(out)
    }

    fn finalize(&mut self) -> Result<Vec<Output>, ConsumerError> {
        if self.first {
            Ok(vec![])
        } else {
            Ok(vec![Output::End])
        }
    }
}

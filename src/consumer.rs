use std::io::Write;

use crate::parser::LogEntry;

#[derive(Debug)]
pub struct ConsumerError;

pub trait Consumer {
    fn consume(&mut self, entry: &LogEntry) -> Result<(), ConsumerError>;
    fn finalize(&mut self) -> Result<(), ConsumerError>;
}

pub struct TextConsumer {
    writer: Box<dyn Write>,
}

impl TextConsumer {
    pub fn new(writer: Box<dyn Write>) -> Self {
        TextConsumer { writer }
    }
}

impl Consumer for TextConsumer {
    fn consume(&mut self, entry: &LogEntry) -> Result<(), ConsumerError> {
        self.writer
            .write_fmt(format_args!(
                "{} [{}] {}: '{}'\n",
                entry.timestamp.to_rfc3339(),
                entry.level.as_str(),
                entry.service,
                entry.message
            ))
            .map_err(|_| ConsumerError)
    }

    fn finalize(&mut self) -> Result<(), ConsumerError> {
        Ok(())
    }
}

pub struct JsonConsumer {
    writer: Box<dyn Write>,
    first: bool,
}

impl JsonConsumer {
    pub fn new(writer: Box<dyn Write>) -> Result<Self, ConsumerError> {
        let mut consumer = JsonConsumer {
            writer,
            first: true,
        };
        consumer.writer.write_all(b"[\n").map_err(|_| ConsumerError)?;
        Ok(consumer)
    }
}

impl Consumer for JsonConsumer {
    fn consume(&mut self, entry: &LogEntry) -> Result<(), ConsumerError> {
        if !self.first {
            self.writer
                .write_all(b",\n")
                .map_err(|_| ConsumerError)?
        }
        self.first = false;

        self.writer
            .write_fmt(format_args!(
                "    {{\"timestamp\": \"{}\", \"level\": \"{}\", \"service\": \"{}\", \"message\": \"{}\"}}",
                entry.timestamp.to_rfc3339(),
                entry.level.as_str(),
                entry.service,
                entry.message
            ))
            .map_err(|_| ConsumerError)
    }

    fn finalize(&mut self) -> Result<(), ConsumerError> {
        self.writer
            .write_all(b"\n]\n")
            .map_err(|_| ConsumerError)
    }
}

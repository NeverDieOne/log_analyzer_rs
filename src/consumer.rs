use crate::parser::{LogEntry};

#[derive(Debug)]
pub struct ConsumerError;

pub trait Consumer {
    fn consume(&mut self, entry: &LogEntry) -> Result<(), ConsumerError>;
    fn finalize(&mut self) -> Result<(), ConsumerError>;
}

pub struct TextConsumer;

impl Consumer for TextConsumer { 
    fn consume(&mut self, entry: &LogEntry) -> Result<(), ConsumerError> {
        println!(
            "{} [{}] {}: '{}'",
            entry.timestamp.to_rfc3339(),
            entry.level.as_str(),
            entry.service,
            entry.message
        );
        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ConsumerError> {
        Ok(())
    }
}


pub struct JsonConsumer {
    first: bool,
}

impl JsonConsumer {
    pub fn new() -> Self {
        println!("[");
        JsonConsumer { first: true }
    }
}

impl Consumer for JsonConsumer {
    fn consume(&mut self, entry: &LogEntry) -> Result<(), ConsumerError> {
        if !self.first {
            println!(",");
        }
        self.first = false;

        print!(
            "    {{\"timestamp\":\"{}\",\"level\":\"{}\",\"service\":\"{}\",\"message\":\"{}\"}}",
            entry.timestamp.to_rfc3339(),
            entry.level.as_str(),
            entry.service,
            entry.message
        );
        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ConsumerError> {
        println!("\n]");
        Ok(())
    }
}

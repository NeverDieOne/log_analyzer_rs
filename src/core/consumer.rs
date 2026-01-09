use crate::core::output::Output;
use crate::core::parser::LogEntry;
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

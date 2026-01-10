use crate::core::output::Output;
use crate::core::parser::LogEntry;

pub trait Consumer {
    fn consume(&mut self, entry: &LogEntry) -> Vec<Output>;
    fn finalize(&mut self) -> Vec<Output>;
}

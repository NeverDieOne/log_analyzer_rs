use crate::consumer::{Consumer, ConsumerError};
use crate::output::Output;
use crate::parser::LogEntry;
use std::vec;

pub struct CountAggregator {
    count: u64,
}

impl CountAggregator {
    pub fn new() -> Self {
        CountAggregator { count: 0 }
    }
}

impl Consumer for CountAggregator {
    fn consume(&mut self, _entry: &LogEntry) -> Result<Vec<Output>, ConsumerError> {
        self.count += 1;
        Ok(vec![])
    }

    fn finalize(&mut self) -> Result<Vec<Output>, ConsumerError> {
        Ok(vec![Output::Line(format!(
            "Total log entries processed: {}",
            self.count
        ))])
    }
}

use crate::core::consumer::{Consumer, ConsumerError};
use crate::core::output::Output;
use crate::core::parser::LogEntry;

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
            "---\nTotal log entries processed: {}\n",
            self.count
        ))])
    }
}

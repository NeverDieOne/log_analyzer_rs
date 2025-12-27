use crate::consumer::{Consumer, ConsumerError};
use crate::parser::LogEntry;
use std::io::Write;


pub struct CountAggregator {
    writer: Box<dyn Write>,
    count: u64,
}

impl CountAggregator {
    pub fn new(writer: Box<dyn Write>) -> Self {
        CountAggregator { writer, count: 0 }
    }
}

impl Consumer for CountAggregator {
    fn consume(&mut self, _entry: &LogEntry) -> Result<(), ConsumerError> {
        self.count += 1;
        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ConsumerError> {
        self.writer.write_fmt(format_args!("Total log entries processed: {}\n", self.count)).map_err(|_| ConsumerError)
    }
}

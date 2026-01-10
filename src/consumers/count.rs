use crate::core::consumer::Consumer;
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
    fn consume(&mut self, _entry: &LogEntry) -> Vec<Output> {
        self.count += 1;
        vec![]
    }

    fn finalize(&mut self) -> Vec<Output> {
        vec![Output::Line(format!(
            "---\nTotal log entries processed: {}\n",
            self.count
        ))]
    }
}

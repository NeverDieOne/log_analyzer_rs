use crate::core::consumer::{Consumer, ConsumerError};
use crate::core::output::Output;
use crate::core::parser::LogEntry;

pub struct LevelAggregator {
    level_counts: std::collections::HashMap<String, u64>,
}

impl LevelAggregator {
    pub fn new() -> Self {
        LevelAggregator {
            level_counts: std::collections::HashMap::new(),
        }
    }
}

impl Consumer for LevelAggregator {
    fn consume(&mut self, entry: &LogEntry) -> Result<Vec<Output>, ConsumerError> {
        let counter = self
            .level_counts
            .entry(entry.level.as_str().to_string())
            .or_insert(0);
        *counter += 1;
        Ok(vec![])
    }

    fn finalize(&mut self) -> Result<Vec<Output>, ConsumerError> {
        let mut outputs = vec![Output::Line("---\nLog Level Counts:\n".to_string())];
        for (level, count) in &self.level_counts {
            outputs.push(Output::Line(format!("{level}: {count}\n")));
        }
        Ok(outputs)
    }
}

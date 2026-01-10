use crate::core::consumer::Consumer;
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
    fn consume(&mut self, entry: &LogEntry) -> Vec<Output> {
        let counter = self
            .level_counts
            .entry(entry.level.as_str().to_string())
            .or_insert(0);
        *counter += 1;
        vec![]
    }

    fn finalize(&mut self) -> Vec<Output> {
        let mut outputs = vec![Output::Line("---\nLog Level Counts:\n".to_string())];
        for (level, count) in &self.level_counts {
            outputs.push(Output::Line(format!("{level}: {count}\n")));
        }
        outputs
    }
}

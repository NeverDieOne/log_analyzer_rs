use crate::core::consumer::Consumer;
use crate::core::output::Output;
use crate::core::parser::LogEntry;

pub struct JsonConsumer {
    first: bool,
}

impl JsonConsumer {
    pub fn new() -> JsonConsumer {
        JsonConsumer { first: true }
    }
}

impl Consumer for JsonConsumer {
    fn consume(&mut self, entry: &LogEntry) -> Vec<Output> {
        let mut out = vec![];

        if self.first {
            out.push(Output::Begin);
            self.first = false;
        } else {
            out.push(Output::Line(",\n".to_string()));
        }

        out.push(Output::Line(
            format!(
                "    {{\"timestamp\": \"{}\", \"level\": \"{}\", \"service\": \"{}\", \"message\": \"{}\"}}",
                entry.timestamp.to_rfc3339(),
                entry.level.as_str(),
                entry.service,
                entry.message
            )
        ));

        out
    }

    fn finalize(&mut self) -> Vec<Output> {
        if self.first {
            vec![]
        } else {
            vec![Output::End]
        }
    }
}

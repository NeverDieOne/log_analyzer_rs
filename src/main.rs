use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod cli;
mod consumers;
mod core;

use cli::Args;
use consumers::{CountAggregator, JsonConsumer, LevelAggregator, TextConsumer};
use core::consumer::Consumer;
use core::filter::{Filters, match_filters};
use core::output::OutputWriter;
use core::parser::parse_log_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file = File::open(&args.file).expect("Unable to open log file");
    let reader = BufReader::new(file);
    let mut output_writer = OutputWriter::new(std::io::stdout());

    let mut consumers: Vec<Box<dyn Consumer>> = Vec::new();
    if args.json {
        consumers.push(Box::new(JsonConsumer::new()));
    } else {
        consumers.push(Box::new(TextConsumer {}));
    }
    if args.count {
        consumers.push(Box::new(CountAggregator::new()));
    }
    if args.level_aggregate {
        consumers.push(Box::new(LevelAggregator::new()));
    }

    let filters = match Filters::try_from(args) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error in configuration: {e}");
            return Err(Box::new(e));
        }
    };

    for line in reader.lines() {
        let log_line = match line {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading line: {e}");
                continue;
            }
        };

        let log_entry = match parse_log_line(&log_line) {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Failed to parse log line: {log_line}; Error: {e}");
                continue;
            }
        };

        if !match_filters(&log_entry, &filters) {
            continue;
        }

        for consumer in &mut consumers {
            for out in consumer.consume(&log_entry)? {
                output_writer.write(&out)?;
            }
        }
    }

    for consumer in &mut consumers {
        for out in consumer.finalize()? {
            output_writer.write(&out)?;
        }
    }

    Ok(())
}

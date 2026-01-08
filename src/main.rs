use chrono::DateTime;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod aggregator;
mod consumer;
mod filter;
mod output;
mod parser;

use aggregator::CountAggregator;
use consumer::{Consumer, JsonConsumer, TextConsumer};
use filter::{Filters, match_filters};
use output::OutputWriter;
use parser::parse_log_line;

/// Log analyzer tool
#[derive(Parser, Debug, Default)]
struct Args {
    /// Path to the log file
    #[arg(short, long, default_value = "./app.log")]
    file: String,

    /// Minimum log level to display (info, warning, error)
    #[arg(short, long)]
    level: Option<String>,

    /// Filter by service type (e.g., auth, payment)
    #[arg(short, long)]
    service: Option<String>,

    /// Filter by message content
    #[arg(short, long)]
    contains: Option<String>,

    /// Filter from timestamp (inclusive)
    #[arg(long)]
    from: Option<DateTime<chrono::Utc>>,

    /// Filter to timestamp (inclusive)
    #[arg(long)]
    to: Option<DateTime<chrono::Utc>>,

    /// Json output format
    #[arg(long, action = clap::ArgAction::SetTrue)]
    json: bool,

    /// Aggregate log entries by count
    #[arg(long, action = clap::ArgAction::SetTrue)]
    count: bool,
}

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

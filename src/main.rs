use chrono::DateTime;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod filter;
mod parser;

use filter::{match_filters, Filters};
use parser::parse_log_line;

/// Log analyzer tool
#[derive(Parser, Debug)]
struct Args {
    /// Path to the log file
    #[arg(short, long, default_value = "./src/app.log")]
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
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.file).expect("Unable to open log file");
    let reader = BufReader::new(file);

    let filters = match Filters::try_from(args) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error in configuration: {}", e);
            return;
        }
    };

    for line in reader.lines() {
        let log_line = match line {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading line: {}", e.to_string());
                continue;
            }
        };

        let log_entry = match parse_log_line(&log_line) {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Failed to parse log line: {}; Error: {}", log_line, e);
                continue;
            }
        };

        if !match_filters(&log_entry, &filters) {
            continue;
        }

        println!("{:?}", log_entry);
    }
}

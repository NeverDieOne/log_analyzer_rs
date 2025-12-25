use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;

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
}

#[derive(Debug)]
struct LogEntry {
    timestamp: String,
    level: String,
    service: String,
    message: String,
}

fn parse_log_line(line: &str) -> Option<LogEntry> {
    let parts = line.splitn(4, ' ').collect::<Vec<&str>>();
    if parts.len() != 4 {
        return None;
    }

    return LogEntry {
        timestamp: parts[0].to_string(),
        level: parts[1].to_string(),
        service: parts[2].to_string(),
        message: parts[3].to_string(),
    }
    .into();
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.file).expect("Unable to open log file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let log_line = match line {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };

        let log_entry = match parse_log_line(&log_line) {
            Some(entry) => entry,
            None => {
                eprintln!("Failed to parse log line: {}", log_line);
                continue;
            }
        };

        if let Some(level) = args.level.as_deref() {
            if level != log_entry.level {
                continue;
            }
        };

        if let Some(service) = args.service.as_deref() {
            if service != log_entry.service {
                continue;
            }
        };

        if let Some(contains) = args.contains.as_deref() {
            if !log_entry.message.contains(contains) {
                continue;
            }
        };

        println!("{:?}", log_entry);
    }
}

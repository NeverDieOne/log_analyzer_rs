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
}

#[derive(Debug)]
enum LogLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
struct LogEntry {
    timestamp: String,
    log_type: LogLevel,
    level: String,
    message: String,
}

fn parse_log_line(line: &str) -> Option<LogEntry> {
    let parts = line.splitn(4, ' ').collect::<Vec<&str>>();
    if parts.len() != 4 {
        return None;
    }

    let log_type = match parts[1] {
        "INFO" => LogLevel::Info,
        "WARN" => LogLevel::Warning,
        "ERROR" => LogLevel::Error,
        _ => return None,
    };

    return LogEntry {
        timestamp: parts[0].to_string(),
        log_type: log_type,
        level: parts[2].to_string(),
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

        println!("{:?}", log_entry);
    }
}

use chrono::DateTime;
use clap::Parser;

/// Log analyzer tool
#[derive(Parser, Debug, Default)]
pub struct Args {
    /// Path to the log file
    #[arg(short, long, default_value = "./app.log")]
    pub file: String,

    /// Minimum log level to display (info, warning, error)
    #[arg(short, long)]
    pub level: Option<String>,

    /// Filter by service type (e.g., auth, payment)
    #[arg(short, long)]
    pub service: Option<String>,

    /// Filter by message content
    #[arg(short, long)]
    pub contains: Option<String>,

    /// Filter from timestamp (inclusive)
    #[arg(long)]
    pub from: Option<DateTime<chrono::Utc>>,

    /// Filter to timestamp (inclusive)
    #[arg(long)]
    pub to: Option<DateTime<chrono::Utc>>,

    /// Json output format
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub json: bool,

    /// Aggregate log entries by count
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub count: bool,

    /// Aggregate log entries by level
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub level_aggregate: bool,
}

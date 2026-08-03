use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum, Debug)]
pub enum OutputFormat {
    Console,
    Json,
    Csv,
    Html,
}

#[derive(Parser, Debug)]
#[command(name = "scansql")]
#[command(about = "ScanSQL: Detection-only SQL injection vulnerability scanner", long_about = None)]
pub struct Cli {
    /// Target HTTP URL to scan
    #[arg(short, long)]
    pub url: String,

    /// Comma-separated list of parameters to scan
    #[arg(short, long)]
    pub params: String,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Console)]
    pub format: OutputFormat,

    /// Output file path (required for json, csv, html)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<String>,

    /// Verbosity level (e.g. info, debug, trace)
    #[arg(short, long, default_value = "info")]
    pub verbosity: String,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

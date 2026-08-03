use reqwest::Url;
use sqrul::cli::{Cli, OutputFormat};
use sqrul::client::HttpClient;
use sqrul::config::AppConfig;
use sqrul::exporter::Exporter;
use sqrul::reporter::{print_banner, Reporter};
use sqrul::scanner::Scanner;
use sqrul::target::Target;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse_args();
    print_banner();

    tracing_subscriber::fmt()
        .with_env_filter(&args.verbosity)
        .init();

    let params: Vec<String> = args
        .params
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if params.is_empty() {
        eprintln!("Error: No parameters provided for scanning.");
        std::process::exit(1);
    }

    let url = Url::parse(&args.url)?;
    info!("Initializing scan on {} for parameters: {:?}", url, params);

    let config = AppConfig::default();
    let client = HttpClient::new(config.client)?;
    let scanner = Scanner::new(client, config.detection);

    let target = Target::new(url, params);
    let result = scanner.scan(target).await?;

    match args.format {
        OutputFormat::Console => Reporter::console(&result),
        OutputFormat::Json => {
            if let Some(out) = args.output {
                Exporter::export_json(&result, &out)?;
                info!("JSON report saved to {}", out);
            } else {
                eprintln!("Error: Output file path required for JSON format.");
            }
        }
        OutputFormat::Csv => {
            if let Some(out) = args.output {
                Exporter::export_csv(&result, &out)?;
                info!("CSV report saved to {}", out);
            } else {
                eprintln!("Error: Output file path required for CSV format.");
            }
        }
        OutputFormat::Html => {
            if let Some(out) = args.output {
                Exporter::export_html(&result, &out)?;
                info!("HTML report saved to {}", out);
            } else {
                eprintln!("Error: Output file path required for HTML format.");
            }
        }
    }

    if !result.findings.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}

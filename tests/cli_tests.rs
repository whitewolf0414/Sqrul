use clap::Parser;
use sqrul::cli::{Cli, OutputFormat};

#[test]
fn test_cli_parsing_valid_url() {
    let args = vec!["scansql", "--url", "http://example.com", "--params", "id"];
    let cli = Cli::parse_from(args);
    assert_eq!(cli.url, "http://example.com");
    assert_eq!(cli.params, "id");
    assert!(matches!(cli.format, OutputFormat::Console));
}

#[test]
fn test_cli_parsing_json_format() {
    let args = vec![
        "scansql",
        "--url",
        "http://example.com",
        "--params",
        "id",
        "--format",
        "json",
        "--output",
        "report.json",
    ];
    let cli = Cli::parse_from(args);
    assert!(matches!(cli.format, OutputFormat::Json));
    assert_eq!(cli.output.unwrap(), "report.json");
}

#[test]
fn test_cli_missing_required_params() {
    let args = vec!["scansql", "--url", "http://example.com"];
    let result = Cli::try_parse_from(args);
    assert!(result.is_err()); // --params is required
}

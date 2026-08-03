# API Reference

The `sqrul` crate can be used as a library in your own Rust projects.

## Full Usage Example

```rust
use sqrul::client::HttpClient;
use sqrul::config::AppConfig;
use sqrul::scanner::Scanner;
use sqrul::target::Target;
use reqwest::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load default configuration
    let config = AppConfig::default();

    // Build the HTTP client (redirects disabled, 5 MB body cap)
    let client = HttpClient::new(config.client)?;

    // Initialize the scanner
    let scanner = Scanner::new(client, config.detection);

    // Define the target
    let url = Url::parse("http://localhost:8080/search")?;
    let target = Target::new(url, vec!["q".to_string(), "id".to_string()]);

    // Run the scan
    let result = scanner.scan(target).await?;

    // Inspect results
    println!("Requests sent: {}", result.statistics.requests_sent);
    println!("Findings: {}", result.findings.len());

    for finding in &result.findings {
        println!(
            "[{}] {} - {} confidence",
            finding.parameter,
            finding.technique,
            finding.confidence
        );
    }

    Ok(())
}
```

---

## Key Types

### `AppConfig`
Holds both `ClientConfig` and `DetectionConfig`. Use `AppConfig::default()` for sensible defaults.

### `HttpClient`
Wraps `reqwest::Client` with hardened settings. Created via `HttpClient::new(ClientConfig)`.

### `Scanner`
Created via `Scanner::new(HttpClient, DetectionConfig)`. Call `.scan(Target).await` to run.

### `Target`
Created via `Target::new(Url, Vec<String>)`. Holds the URL and parameter list.

### `ScanResult`
Returned by `Scanner::scan()`. Contains:
- `target: Target`
- `findings: Vec<Finding>`
- `statistics: ScanStatistics`

### `Finding`
| Field | Type | Description |
|-------|------|-------------|
| `parameter` | `String` | Affected parameter name |
| `technique` | `String` | Detection technique label |
| `severity` | `Severity` | Low / Medium / High / Critical |
| `confidence` | `Confidence` | Low / Medium / High / Critical |
| `evidence` | `String` | Description of what triggered the finding |
| `affected_url` | `String` | Full URL |
| `method` | `String` | HTTP method |
| `recommendation` | `String` | Remediation guidance |

---

## Exporter

```rust
use sqrul::exporter::Exporter;
use std::path::Path;

Exporter::export_json(&result, Path::new("report.json"))?;
Exporter::export_csv(&result, Path::new("report.csv"))?;
Exporter::export_html(&result, Path::new("report.html"))?;
```

use crate::scanner::ScanResult;
use html_escape::encode_text;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct Exporter;

/// Prepends a single-quote to values starting with formula-injection characters.
fn sanitize_csv(value: &str) -> String {
    if value.starts_with(['=', '+', '-', '@']) {
        format!("'{}", value)
    } else {
        value.replace('"', "\"\"")
    }
}

impl Exporter {
    /// Serializes findings to a pretty-printed JSON file.
    /// Returns an error if serialization or file I/O fails.
    pub fn export_json<P: AsRef<Path>>(result: &ScanResult, path: P) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&result.findings).map_err(std::io::Error::other)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Writes findings to a CSV file with formula-injection sanitization.
    pub fn export_csv<P: AsRef<Path>>(result: &ScanResult, path: P) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(
            file,
            "Parameter,URL,Method,Technique,Confidence,Severity,Evidence"
        )?;

        for f in &result.findings {
            writeln!(
                file,
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"",
                sanitize_csv(&f.parameter),
                sanitize_csv(&f.affected_url),
                sanitize_csv(&f.method),
                sanitize_csv(&f.technique),
                sanitize_csv(&f.confidence.to_string()),
                sanitize_csv(&f.severity.to_string()),
                sanitize_csv(&f.evidence),
            )?;
        }
        Ok(())
    }

    /// Writes findings to an HTML report with all user-controlled values escaped.
    pub fn export_html<P: AsRef<Path>>(result: &ScanResult, path: P) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(
            file,
            "<!DOCTYPE html><html><head><title>Scan Report</title></head><body>"
        )?;
        writeln!(file, "<h1>SQL Injection Scan Report</h1>")?;
        writeln!(
            file,
            "<p>Target: {}</p>",
            encode_text(result.target.url.as_str())
        )?;
        writeln!(
            file,
            "<p>Requests sent: {}</p>",
            result.statistics.requests_sent
        )?;
        writeln!(
            file,
            "<p>Scan duration: {:.2}s</p>",
            result.statistics.duration.as_secs_f64()
        )?;

        if result.findings.is_empty() {
            writeln!(file, "<p>No vulnerabilities found.</p>")?;
        } else {
            writeln!(
                file,
                "<table border=\"1\"><tr><th>Parameter</th><th>Method</th><th>Technique</th><th>Confidence</th><th>Severity</th><th>Evidence</th></tr>"
            )?;
            for f in &result.findings {
                writeln!(
                    file,
                    "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                    encode_text(&f.parameter),
                    encode_text(&f.method),
                    encode_text(&f.technique),
                    encode_text(&f.confidence.to_string()),
                    encode_text(&f.severity.to_string()),
                    encode_text(&f.evidence),
                )?;
            }
            writeln!(file, "</table>")?;
        }
        writeln!(file, "</body></html>")?;
        Ok(())
    }
}

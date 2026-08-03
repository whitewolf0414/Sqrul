use crate::finding::{Confidence, Severity};
use crate::scanner::ScanResult;
use colored::Colorize;

// Dark orange: (210, 105, 30) | Grey: (160, 160, 160) | Light grey: (100, 100, 100)

fn orange(s: &str) -> String {
    s.truecolor(210, 105, 30).to_string()
}

fn grey(s: &str) -> String {
    s.truecolor(160, 160, 160).to_string()
}

fn dim(s: &str) -> String {
    s.truecolor(100, 100, 100).to_string()
}

fn separator() {
    println!(
        "{}",
        dim("──────────────────────────────────────────────────")
    );
}

fn thick_separator() {
    println!(
        "{}",
        orange("══════════════════════════════════════════════════")
    );
}

pub fn print_banner() {
    println!();
    thick_separator();
    println!(
        "  {}  {}",
        orange("SQRUL"),
        grey("SQL Injection Vulnerability Scanner")
    );
    println!(
        "  {}",
        dim("Detection-only · Defensive security assessments")
    );
    thick_separator();
    println!();
}

pub struct Reporter;

impl Reporter {
    pub fn console(result: &ScanResult) {
        println!();
        thick_separator();
        println!("  {}", orange("SCAN SUMMARY"));
        thick_separator();

        println!(
            "  {}  {}",
            grey("Target   :"),
            result.target.url.as_str().truecolor(210, 105, 30)
        );
        println!(
            "  {}  {}",
            grey("Params   :"),
            result.statistics.parameters_tested.to_string().white()
        );
        println!(
            "  {}  {}",
            grey("Requests :"),
            result.statistics.requests_sent.to_string().white()
        );
        println!(
            "  {}  {}",
            grey("Duration :"),
            format!("{:.2}s", result.statistics.duration.as_secs_f64()).white()
        );

        separator();

        if result.findings.is_empty() {
            println!(
                "  {} {}",
                "✔".truecolor(80, 200, 80),
                grey("No SQL injection vulnerabilities detected.")
            );
            thick_separator();
            println!();
            return;
        }

        println!(
            "  {} {}\n",
            orange("⚠"),
            format!("{} finding(s) detected", result.findings.len())
                .truecolor(210, 105, 30)
                .bold()
        );

        for (i, f) in result.findings.iter().enumerate() {
            println!(
                "  {} {}",
                orange(&format!("[{}]", i + 1)),
                f.parameter.white().bold()
            );

            println!(
                "      {}  {} {}",
                grey("URL       :"),
                f.affected_url.truecolor(160, 160, 160),
                format!("[{}]", f.method).truecolor(100, 100, 100)
            );

            println!("      {}  {}", grey("Technique :"), f.technique.white());

            let confidence_str = match f.confidence {
                Confidence::High | Confidence::Critical => f
                    .confidence
                    .to_string()
                    .truecolor(210, 105, 30)
                    .bold()
                    .to_string(),
                Confidence::Medium => f.confidence.to_string().yellow().to_string(),
                _ => f
                    .confidence
                    .to_string()
                    .truecolor(160, 160, 160)
                    .to_string(),
            };

            let severity_str = match f.severity {
                Severity::High | Severity::Critical => f
                    .severity
                    .to_string()
                    .truecolor(210, 105, 30)
                    .bold()
                    .to_string(),
                Severity::Medium => f.severity.to_string().yellow().to_string(),
                _ => f.severity.to_string().truecolor(160, 160, 160).to_string(),
            };

            println!("      {}  {}", grey("Confidence:"), confidence_str);
            println!("      {}  {}", grey("Severity  :"), severity_str);
            println!(
                "      {}  {}",
                grey("Evidence  :"),
                f.evidence.truecolor(160, 160, 160)
            );
            println!(
                "      {}  {}",
                grey("Fix       :"),
                f.recommendation.truecolor(100, 100, 100)
            );

            if i < result.findings.len() - 1 {
                separator();
            }

            println!();
        }

        thick_separator();
        println!();
    }
}

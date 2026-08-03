# SQRUL

> A modular, asynchronous SQL Injection Vulnerability Scanner for defensive security assessments.

![License](https://img.shields.io/badge/license-Apache%202.0-blue)
![Rust](https://img.shields.io/badge/rust-1.83%2B-orange)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)

---

## Features

- **Detection-Only:** No exploitation. Safe for authorized assessments.
- **Async Engine:** High-performance concurrent scanning via Tokio and Reqwest.
- **Three Detection Techniques:** Error-based, Boolean-based, and Time-based SQLi detection.
- **Weighted Confidence Scoring:** Boolean findings are scored using a multi-factor algorithm.
- **Colored Terminal UI:** Minimalist dark orange and grey output for quick triage.
- **Structured Reports:** Export findings as JSON, CSV, or HTML.
- **Security-Hardened Exports:** HTML output is XSS-escaped. CSV is formula-injection sanitized.
- **Response Size Limiting:** Enforces a 5 MB cap per response to prevent memory exhaustion.
- **Redirect Disabled:** Automatic redirects are off to eliminate SSRF attack surface.

---

## Architecture Diagram

```text
[ CLI Input ] -> [ Target Builder ] -> [ Scanner Engine ]
                                              |
                                              v
[ Config ] -----------------------> [ HTTP Client (no redirects, 5MB cap) ]
                                              |
                                              v
                                    [ Detection Engine ]
                                     /       |        \
                               Error    Boolean     Time-based
                                 \         |         /
                                  [ Findings Vec ]
                                         |
                                         v
                          [ Reporter / Exporter ]
                       (Console | JSON | CSV | HTML)
```

---

## Detection Techniques

| Technique | How It Works | Confidence |
|-----------|-------------|------------|
| **Error-Based** | Injects syntax-breaking chars, matches DB error signatures | High |
| **Boolean-Based** | Compares TRUE/FALSE payload responses against baseline (weighted score) | Low / Medium / High |
| **Time-Based** | Measures sleep-induced latency vs baseline | Medium / High |

---

## Installation

Requires **Rust 1.83.0+**.

```bash
git clone https://github.com/whitewolf0414/sqrul.git
cd sqrul
cargo build --release
```

The compiled binary is at `target/release/sqrul`.

---

## Running

```bash
# Via cargo
cargo run --release -- --url "http://example.com" --params "id,user"

# Via compiled binary
./target/release/sqrul --url "http://example.com" --params "id"
```

---

## Examples

**Basic scan:**
```bash
./target/release/sqrul --url "http://example.com/page" --params "id"
```

**Multiple parameters with JSON output:**
```bash
./target/release/sqrul --url "http://example.com/page" \
  --params "id,search,category" \
  --format json \
  --output report.json
```

**Debug verbosity:**
```bash
./target/release/sqrul --url "http://example.com" --params "id" --verbosity debug
```

---

## CLI Usage

```
Usage: sqrul [OPTIONS] --url <URL> --params <PARAMS>

Options:
  -u, --url <URL>           Target HTTP URL to scan
  -p, --params <PARAMS>     Comma-separated list of parameters to scan
  -f, --format <FORMAT>     Output format [default: console]
                            [possible values: console, json, csv, html]
  -o, --output <FILE>       Output file path (required for json, csv, html)
  -v, --verbosity <LEVEL>   Verbosity level [default: info]
                            [possible values: error, warn, info, debug, trace]
  -h, --help                Print help
  -V, --version             Print version
```

---

## Output Formats

| Format | Use Case |
|--------|----------|
| `console` | Terminal review — colored, human-readable |
| `json` | CI/CD pipelines, programmatic consumption |
| `csv` | Spreadsheet analysis (formula-injection sanitized) |
| `html` | Stakeholder reports (XSS-escaped) |

---

## Security Notes

- **No redirects followed** — prevents SSRF via redirect chains.
- **Response bodies capped at 5 MB** — prevents memory exhaustion.
- **HTML reports are XSS-escaped** — `<script>` becomes `&lt;script&gt;`.
- **CSV exports are formula-sanitized** — values starting with `=`, `+`, `-`, `@` are prefixed with `'`.
- **JSON export errors are surfaced** — no silent failures.
- This tool is for **authorized testing only**. Scanning targets without permission is illegal.

---

## Project Structure

```
src/
├── main.rs       - CLI entry point and output dispatch
├── cli.rs        - Argument definitions (clap)
├── config.rs     - Client and detection configuration
├── client.rs     - Hardened async HTTP client
├── target.rs     - Scan target model
├── scanner.rs    - Scan orchestration
├── detector.rs   - SQLi detection logic
├── payloads.rs   - Static payload and signature sets
├── reporter.rs   - Colored console output
├── exporter.rs   - JSON / CSV / HTML export
├── finding.rs    - Finding data model
├── response.rs   - HTTP response model
├── request.rs    - HTTP request builder
├── error.rs      - Unified error type
└── lib.rs        - Module exports
```

---

## Testing

```bash
cargo test
```

Tests cover:
- CLI argument parsing
- HTTP request construction and timeout handling
- Error-based signature detection
- Boolean-based confidence scoring
- Full scan-to-export pipeline (via mockito mock server)

---

## Documentation

| File | Contents |
|------|----------|
| `docs/architecture.md` | Module layout and data flow |
| `docs/getting_started.md` | Quick start guide |
| `docs/detection.md` | Detection technique details |
| `docs/examples.md` | Usage examples |
| `docs/api.md` | Library API reference |

---

## Roadmap

- Header-based SQLi detection
- GraphQL injection support
- Custom payload loading from external files
- Native proxy support
- Advanced false-positive reduction

---

## Contributing

Please read [`CONTRIBUTING.md`](CONTRIBUTING.md) for guidelines on pull requests, code style, and running tests.

---

## License

Apache License 2.0 — see [`LICENSE`](LICENSE) for details.

---

## Known Limitations

- Network jitter may cause false positives or negatives in time-based detection.
- WAFs may block probes, causing false negatives.
- Only GET parameter injection is currently supported.
- Redirects are disabled by design — some targets may require manual redirect handling.

---

## Author

**Intern ID:** CITS6600
**GitHub:** https://github.com/whitewolf0414

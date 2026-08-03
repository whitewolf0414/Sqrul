# Getting Started

## Prerequisites

- Rust **1.83.0** or newer — [install via rustup](https://rustup.rs/)
- Network access to the target application
- **Written authorization** to test the target

## Build

```bash
git clone https://github.com/whitewolf0414/sqrul.git
cd sqrul
cargo build --release
```

The binary is produced at `target/release/sqrul`.

## First Scan

Run a basic scan against a single parameter:

```bash
./target/release/sqrul \
  --url "http://localhost:8080/search" \
  --params "q"
```

You will see the colored terminal output immediately:

```
══════════════════════════════════════════════════
  SQRUL  SQL Injection Vulnerability Scanner
  Detection-only · Defensive security assessments
══════════════════════════════════════════════════

══════════════════════════════════════════════════
  SCAN SUMMARY
══════════════════════════════════════════════════
  Target   :  http://localhost:8080/search
  Params   :  1
  Requests :  8
  Duration :  2.14s
──────────────────────────────────────────────────
  ✔ No SQL injection vulnerabilities detected.
══════════════════════════════════════════════════
```

## Export Results

Save findings to JSON for further processing:

```bash
./target/release/sqrul \
  --url "http://localhost:8080/search" \
  --params "q,id" \
  --format json \
  --output results.json
```

## Increase Verbosity

For debugging or detailed request tracing:

```bash
./target/release/sqrul \
  --url "http://localhost:8080/page" \
  --params "id" \
  --verbosity debug
```

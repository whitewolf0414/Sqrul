# Architecture

SQRUL is structured as a flat, modular Rust crate. Each module has a single responsibility.

## Data Flow

```text
[ CLI Args ] --> [ Target::new() ] --> [ Scanner::scan() ]
                                              |
                              +--------------+--------------+
                              |              |              |
                        ErrorBased     BooleanBased    TimeBased
                              |              |              |
                        [ Detector functions (pure, sync) ]
                                              |
                                      [ Vec<Finding> ]
                                              |
                              +--------------+--------------+
                              |              |              |
                         Console           JSON        CSV / HTML
                         Reporter        Exporter      Exporter
```

## Module Responsibilities

| Module | Role |
|--------|------|
| `cli` | Parses CLI arguments via `clap` |
| `config` | Holds `ClientConfig`, `DetectionConfig`, `AppConfig` |
| `target` | Represents a scan target (URL + parameters + method) |
| `client` | Async HTTP client — enforces redirect policy and response size limit |
| `request` | Builds typed `HttpRequest` values |
| `response` | Typed `HttpResponse` (status, body, latency, final URL) |
| `scanner` | Orchestrates baseline + probe execution per parameter |
| `detector` | Pure functions: `check_error_based`, `check_boolean_based`, `check_time_based` |
| `payloads` | Static payload arrays and DB error signature list |
| `finding` | `Finding` struct + `Severity` and `Confidence` enums |
| `reporter` | Colored console output using `colored` crate |
| `exporter` | JSON, CSV (formula-sanitized), HTML (XSS-escaped) export |
| `error` | `SqrulError` enum using `thiserror` |

## Key Design Decisions

- **Detection only:** No exploit payloads. Scanner observes and reports only.
- **No redirects:** `reqwest::redirect::Policy::none()` prevents SSRF.
- **5 MB body cap:** Prevents memory exhaustion from large responses.
- **Pure detector functions:** `check_*` functions take only references and return `Result<Option<Finding>>` — easy to test in isolation.
- **Weighted confidence scoring (boolean):** Score 1–3 based on status change + body divergence + baseline match.

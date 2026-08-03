# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-08-04

### Added
- Modular async SQL injection scanner core using Tokio and Reqwest.
- Three detection techniques: Error-based, Boolean-based, and Time-based.
- Weighted confidence scoring for boolean-based detection (Low / Medium / High).
- Minimalist colored terminal UI — dark orange and grey palette via the `colored` crate.
- Startup banner (`print_banner`) displayed before each scan.
- Structured output formats: Console, JSON, CSV, HTML.
- Comprehensive test suite — unit tests, mock server integration tests, CLI argument tests.
- GitHub Actions CI — multi-platform (Linux, macOS, Windows) with `cargo fmt`, `cargo clippy`, `cargo test`.
- GitHub Actions release workflow — cross-platform binary builds on version tags.
- Dependabot configuration for weekly Cargo and GitHub Actions dependency updates.
- Issue templates (bug report, feature request) and PR template.
- Full documentation suite: `docs/architecture.md`, `docs/detection.md`, `docs/getting_started.md`, `docs/examples.md`, `docs/api.md`.

### Security
- **HTML injection fixed:** All user-controlled fields in HTML reports are escaped via `html-escape`.
- **CSV formula injection fixed:** Values starting with `=`, `+`, `-`, `@` are prefixed with `'`.
- **Silent JSON failure fixed:** Serialization errors are now propagated instead of swallowed.
- **Response body capped at 5 MB:** Prevents memory exhaustion from oversized server responses.
- **Automatic redirects disabled:** `Policy::none()` eliminates SSRF via redirect chains.

### Changed
- Removed unused dependencies: `chrono`, `bytes`, `http`, `tokio-test`.
- Removed redundant tokio features (`rt-multi-thread`, `macros` are included in `full`).
- Removed dead code: `HttpRequestBuilder::get()`, `HttpResponse::is_success()`, `Target::headers`, `Target::with_method`.
- Removed accidental prototype file `[package].rs` from project root.
- `Cargo.toml` version bumped to `1.0.0`.

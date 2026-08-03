# Contributing to SQRUL

We welcome contributions that improve detection accuracy, performance, documentation, or test coverage.

## Getting Started

1. Fork the repository and create a branch from `main`.
2. Make your changes.
3. Run the full check suite:

```bash
cargo fmt --all          # Format code
cargo clippy --all-targets -- -D warnings  # Lint
cargo test               # Run tests
```

4. Open a pull request against `main`.

## Pull Request Guidelines

- Keep changes focused — one purpose per PR.
- If you add code, add tests.
- If you change a public API, update the relevant `docs/` file.
- All CI checks must pass before merging.

## Code Style

- Follow standard Rust idioms.
- Use `cargo fmt` — no manual formatting exceptions.
- Comments should be short, clear, and professional.
- No commented-out code in PRs.

## Security Contributions

If your contribution involves detection logic, payloads, or networking, please review [`SECURITY.md`](SECURITY.md) first.

SQRUL is a **detection-only** tool. Pull requests that introduce exploitation logic will not be accepted.

## Reporting Bugs

Use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.yml).

## Suggesting Features

Use the [feature request template](.github/ISSUE_TEMPLATE/feature_request.yml).

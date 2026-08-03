# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 1.0.x   | ✅ Yes    |
| < 1.0   | ❌ No     |

Only the latest `main` branch and tagged releases receive security updates.

## Security Design

SQRUL is designed with the following hardening measures:

- **Detection-only:** No exploitation payloads. Strictly observational.
- **No redirects:** Automatic HTTP redirects are disabled to prevent SSRF.
- **Response size limit:** Server responses are capped at 5 MB per request.
- **HTML escaping:** All user-controlled content in HTML reports is XSS-escaped.
- **CSV sanitization:** Formula-injection characters are neutralized in CSV exports.
- **No credentials stored:** SQRUL does not store, log, or transmit credentials.

## Reporting a Vulnerability

**Do NOT open a public GitHub issue for security vulnerabilities.**

Use GitHub's private security advisory system:
[https://github.com/whitewolf0414/sqrul/security/advisories/new](https://github.com/whitewolf0414/sqrul/security/advisories/new)

You should receive a response within **48 hours**. If accepted, a patched release will be prioritized.

## Responsible Use

SQRUL is intended **exclusively for authorized security assessments**. Using this tool against systems without explicit permission is illegal and violates this project's intended purpose.

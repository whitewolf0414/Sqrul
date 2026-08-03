# Examples

## Basic Scan

Scan a single GET parameter and print findings to the console:

```bash
./target/release/sqrul --url "http://example.com/page" --params "id"
```

---

## Multiple Parameters

```bash
./target/release/sqrul \
  --url "http://example.com/search" \
  --params "q,category,page"
```

---

## JSON Report

```bash
./target/release/sqrul \
  --url "http://example.com/page" \
  --params "id" \
  --format json \
  --output report.json
```

The JSON file will contain an array of findings:
```json
[
  {
    "parameter": "id",
    "technique": "Error-Based",
    "severity": "High",
    "confidence": "High",
    "evidence": "Response body contained DB error fragment \"you have an error in your sql syntax\"...",
    "affected_url": "http://example.com/page",
    "method": "GET",
    "recommendation": "Use parameterized queries..."
  }
]
```

---

## CSV Report

```bash
./target/release/sqrul \
  --url "http://example.com/page" \
  --params "id" \
  --format csv \
  --output report.csv
```

---

## HTML Report

```bash
./target/release/sqrul \
  --url "http://example.com/page" \
  --params "id" \
  --format html \
  --output report.html
```

Open `report.html` in any browser for a formatted, stakeholder-ready report.

---

## Debug Mode

```bash
./target/release/sqrul \
  --url "http://example.com/page" \
  --params "id" \
  --verbosity debug
```

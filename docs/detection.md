# Detection Logic

SQRUL uses three techniques to detect SQL injection without exploiting the target.

## 1. Error-Based Detection

**How it works:**
Injects syntax-breaking characters (`'`, `"`, `' --`) into each parameter. If the response body contains a known database error string, the parameter is flagged.

**Signatures checked:**
- `you have an error in your sql syntax`
- `warning: mysql_`
- `mysql_fetch`
- `unclosed quotation mark after the character string`
- `incorrect syntax near`
- `microsoft ole db provider for sql server`
- `ora-01756`
- `quoted string not properly terminated`
- `unterminated quoted string at or near`
- `sqlite_error`

**Confidence:** Always `High` — a matching database error string is definitive.

---

## 2. Boolean-Based Detection

**How it works:**
Sends two payloads — a logically TRUE condition and a logically FALSE condition — then compares both responses against the baseline.

**Weighted scoring:**
| Signal | Points |
|--------|--------|
| True response matches baseline | +1 |
| Status code differs (true vs false) | +1 |
| Body length differs by > threshold (50 bytes) | +1 |

**Confidence mapping:**
| Score | Confidence |
|-------|-----------|
| 3 | High |
| 2 | Medium |
| 1 | Low |

---

## 3. Time-Based Detection

**How it works:**
Injects a sleep payload (`SLEEP(5)`, `pg_sleep(5)`, `WAITFOR DELAY '0:0:5'`) and measures the response latency against the baseline.

**Tolerance:** ±1.5 seconds of network variance is allowed.

**Confidence:**
- `High` — probe latency ≥ configured threshold (default 5 s).
- `Medium` — probe latency is within the tolerance window.

---

## Remediation

All findings include the same remediation recommendation:

> Use parameterized queries / prepared statements and validate all user-supplied input server-side. Never interpolate user data directly into SQL strings.

use crate::config::DetectionConfig;
use crate::error::Result;
use crate::finding::{Confidence, Finding, Severity};
use crate::payloads::{TestCase, ERROR_SIGNATURES};
use crate::response::HttpResponse;
use tracing::{debug, trace};

pub fn check_error_based(
    target_url: &str,
    param: &str,
    test_case: &TestCase,
    probe: &HttpResponse,
) -> Result<Option<Finding>> {
    trace!("Checking error-based SQLi for parameter {}", param);
    let body_lower = probe.body.to_ascii_lowercase();
    let matched = ERROR_SIGNATURES
        .iter()
        .find(|&&sig| body_lower.contains(sig))
        .copied();

    match matched {
        None => Ok(None),
        Some(sig) => {
            debug!("Error-based signature matched: {}", sig);
            let evidence = format!(
                "Response body contained DB error fragment \"{sig}\" after injecting payload {:?}.",
                test_case.payload
            );
            Ok(Some(build_finding(
                target_url,
                param,
                test_case,
                Severity::High,
                Confidence::High,
                evidence,
            )))
        }
    }
}

pub fn check_boolean_based(
    target_url: &str,
    param: &str,
    test_case: &TestCase,
    baseline: &HttpResponse,
    true_response: &HttpResponse,
    false_response: &HttpResponse,
    config: &DetectionConfig,
) -> Result<Option<Finding>> {
    trace!("Checking boolean-based SQLi for parameter {}", param);
    let base_status = baseline.status.as_u16();
    let true_status = true_response.status.as_u16();
    let false_status = false_response.status.as_u16();

    let base_len = baseline.body.len() as i64;
    let true_len = true_response.body.len() as i64;
    let false_len = false_response.body.len() as i64;

    let threshold = config.body_difference_threshold;

    let true_matches_base = base_status == true_status && (true_len - base_len).abs() <= threshold;
    let false_diverges = true_status != false_status || (true_len - false_len).abs() > threshold;

    if true_matches_base && false_diverges {
        let mut score = 0;
        if true_matches_base {
            score += 1;
        }
        if true_status != false_status {
            score += 1;
        }
        if (true_len - false_len).abs() > threshold {
            score += 1;
        }

        let confidence = match score {
            3 => Confidence::High,
            2 => Confidence::Medium,
            _ => Confidence::Low,
        };

        debug!("Boolean divergence detected (confidence: {:?})", confidence);

        let evidence = format!(
            "Boolean divergence: True payload matched baseline, False payload diverged (status {} -> {}, body length {} -> {}).",
            true_status, false_status, true_len, false_len
        );

        Ok(Some(build_finding(
            target_url,
            param,
            test_case,
            Severity::High,
            confidence,
            evidence,
        )))
    } else {
        Ok(None)
    }
}

pub fn check_time_based(
    target_url: &str,
    param: &str,
    test_case: &TestCase,
    baseline: &HttpResponse,
    probe: &HttpResponse,
    config: &DetectionConfig,
) -> Result<Option<Finding>> {
    trace!("Checking time-based SQLi for parameter {}", param);
    let base_ms = baseline.latency.as_millis() as i64;
    let probe_ms = probe.latency.as_millis() as i64;
    let expected_ms = config.time_threshold.as_millis() as i64;

    // Account for network variance
    let tol_ms = 1_500;

    if probe_ms >= expected_ms - tol_ms && base_ms < expected_ms - tol_ms {
        let confidence = if probe_ms >= expected_ms {
            Confidence::High
        } else {
            Confidence::Medium
        };
        debug!(
            "Time delay detected (latency: {}ms, expected: {}ms)",
            probe_ms, expected_ms
        );
        let evidence = format!(
            "Time delay: Baseline {}ms, Probe {}ms (expected >={}ms).",
            base_ms, probe_ms, expected_ms
        );
        Ok(Some(build_finding(
            target_url,
            param,
            test_case,
            Severity::High,
            confidence,
            evidence,
        )))
    } else {
        Ok(None)
    }
}

fn build_finding(
    target_url: &str,
    param: &str,
    test_case: &TestCase,
    severity: Severity,
    confidence: Confidence,
    evidence: String,
) -> Finding {
    Finding {
        parameter: param.to_string(),
        technique: test_case.technique.label().to_string(),
        severity,
        confidence,
        evidence,
        recommendation: test_case.technique.remediation().to_string(),
        affected_url: target_url.to_string(),
        method: "GET".to_string(),
    }
}

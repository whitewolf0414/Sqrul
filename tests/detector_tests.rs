use reqwest::StatusCode;
use sqrul::config::DetectionConfig;
use sqrul::detector::{check_boolean_based, check_error_based};
use sqrul::finding::{Confidence, Severity};
use sqrul::payloads::{BooleanPair, Technique, TestCase};
use sqrul::response::HttpResponse;
use std::time::Duration;

fn create_response(body: &str, status: StatusCode) -> HttpResponse {
    HttpResponse {
        status,
        body: body.to_string(),
        latency: Duration::from_millis(50),
        final_url: "http://example.com".to_string(),
    }
}

#[test]
fn test_detect_sql_error_signature() {
    let probe = create_response(
        "<html><body>You have an error in your SQL syntax near '1'</body></html>",
        StatusCode::INTERNAL_SERVER_ERROR,
    );
    let test_case = TestCase {
        id: "err-1",
        technique: Technique::ErrorBased,
        payload: "'",
        boolean_pair: None,
        expected_delay: None,
    };

    let result = check_error_based("http://example.com", "id", &test_case, &probe).unwrap();
    assert!(result.is_some());
    let finding = result.unwrap();
    assert_eq!(finding.severity, Severity::High);
    assert_eq!(finding.confidence, Confidence::High);
}

#[test]
fn test_ignore_normal_html() {
    let probe = create_response(
        "<html><body>Welcome to our normal site!</body></html>",
        StatusCode::OK,
    );
    let test_case = TestCase {
        id: "err-1",
        technique: Technique::ErrorBased,
        payload: "'",
        boolean_pair: None,
        expected_delay: None,
    };

    let result = check_error_based("http://example.com", "id", &test_case, &probe).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_boolean_based_confidence_calculation() {
    let baseline = create_response("Regular content length... (padding to exceed threshold of 50 bytes) ........................................", StatusCode::OK);
    let true_resp = create_response("Regular content length... (padding to exceed threshold of 50 bytes) ........................................", StatusCode::OK);
    let false_resp = create_response("Missing content", StatusCode::OK); // Status code identical, length diverges significantly

    let config = DetectionConfig {
        time_threshold: Duration::from_secs(5),
        body_difference_threshold: 50,
    };

    let test_case = TestCase {
        id: "bool-1",
        technique: Technique::BooleanBased,
        payload: "",
        boolean_pair: Some(BooleanPair {
            true_payload: " AND 1=1",
            false_payload: " AND 1=2",
        }),
        expected_delay: None,
    };

    let result = check_boolean_based(
        "http://example.com",
        "id",
        &test_case,
        &baseline,
        &true_resp,
        &false_resp,
        &config,
    )
    .unwrap();
    assert!(result.is_some());
    let finding = result.unwrap();

    // Body difference alone warrants Medium confidence (conservative approach)
    assert_eq!(finding.confidence, Confidence::Medium);
}

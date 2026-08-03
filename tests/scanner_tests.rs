use mockito::Server;
use reqwest::Url;
use sqrul::client::HttpClient;
use sqrul::config::{AppConfig, DetectionConfig};
use sqrul::scanner::Scanner;
use sqrul::target::Target;
use std::time::Duration;

#[tokio::test]
async fn test_scanner_with_mocked_server() {
    let mut server = Server::new_async().await;

    // Baseline response
    let _mock_base = server
        .mock("GET", "/api")
        .match_query(mockito::Matcher::UrlEncoded("id".into(), "1".into()))
        .with_status(200)
        .with_body("Valid profile")
        .create_async()
        .await;

    // Error-based probe matching a known signature
    let _mock_err = server
        .mock("GET", "/api")
        .match_query(mockito::Matcher::UrlEncoded("id".into(), "1'".into()))
        .with_status(500)
        .with_body("warning: mysql_fetch_array()")
        .create_async()
        .await;

    // Catch-all for other probes
    let _mock_others = server
        .mock("GET", "/api")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body("Valid profile")
        .expect_at_least(0)
        .create_async()
        .await;

    let url = Url::parse(&format!("{}/api", server.url())).unwrap();
    let target = Target::new(url, vec!["id".to_string()]);

    let config = AppConfig::default();
    let client = HttpClient::new(config.client).unwrap();
    let detection_config = DetectionConfig {
        time_threshold: Duration::from_secs(5),
        body_difference_threshold: 50,
    };

    let scanner = Scanner::new(client, detection_config);
    let result = scanner.scan(target).await.unwrap();

    // The scan should find at least the error-based signature we mocked
    assert!(
        !result.findings.is_empty(),
        "Scanner should have produced findings"
    );
    let has_error_finding = result.findings.iter().any(|f| f.technique == "Error-Based");
    assert!(has_error_finding, "Expected Error-Based finding");
}

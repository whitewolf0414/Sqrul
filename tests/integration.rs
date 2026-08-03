use mockito::Server;
use reqwest::Url;
use sqrul::client::HttpClient;
use sqrul::config::AppConfig;
use sqrul::exporter::Exporter;
use sqrul::scanner::Scanner;
use sqrul::target::Target;

#[tokio::test]
async fn test_integration_full_scan_and_export() {
    let mut server = Server::new_async().await;

    // Baseline
    let _mock_base = server
        .mock("GET", "/vulnerable")
        .match_query(mockito::Matcher::UrlEncoded("q".into(), "1".into()))
        .with_status(200)
        .with_body("Search Results: 10")
        .create_async()
        .await;

    // Error-based SQLi match
    let _mock_err = server
        .mock("GET", "/vulnerable")
        .match_query(mockito::Matcher::UrlEncoded("q".into(), "1'".into()))
        .with_status(500)
        .with_body("unclosed quotation mark after the character string")
        .create_async()
        .await;

    // Catch-all to keep other requests happy
    let _mock_others = server
        .mock("GET", "/vulnerable")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_body("Search Results: 10")
        .expect_at_least(0)
        .create_async()
        .await;

    let url = Url::parse(&format!("{}/vulnerable", server.url())).unwrap();
    let target = Target::new(url, vec!["q".to_string()]);

    let config = AppConfig::default();
    let client = HttpClient::new(config.client).unwrap();
    let scanner = Scanner::new(client, config.detection);

    // Run the scan
    let result = scanner.scan(target).await.unwrap();
    assert!(
        !result.findings.is_empty(),
        "Should detect the unclosed quotation mark error"
    );

    // Export to JSON (temporary file)
    let temp_dir = tempfile::tempdir().unwrap();
    let json_path = temp_dir.path().join("report.json");

    Exporter::export_json(&result, &json_path).unwrap();
    assert!(json_path.exists());

    let content = std::fs::read_to_string(json_path).unwrap();
    assert!(content.contains("unclosed quotation mark"));
}

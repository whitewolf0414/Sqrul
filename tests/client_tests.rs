use reqwest::{Method, Url};
use sqrul::client::HttpClient;
use sqrul::config::ClientConfig;
use sqrul::request::HttpRequestBuilder;
use std::time::Duration;

#[tokio::test]
async fn test_request_construction() {
    let url = Url::parse("http://example.com/api").unwrap();
    let req = HttpRequestBuilder::new(Method::GET, url.clone())
        .query("id", "1")
        .build();

    assert_eq!(req.method, Method::GET);
    assert_eq!(req.url.as_str(), "http://example.com/api?id=1");
}

#[tokio::test]
async fn test_client_timeout_handling() {
    let config = ClientConfig {
        timeout: Duration::from_millis(1), // Extremely short timeout
        max_redirects: 3,
        pool_idle_timeout: None,
    };

    let client = HttpClient::new(config).unwrap();
    let url = Url::parse("http://10.255.255.1").unwrap(); // Non-routable IP to force timeout/error
    let req = HttpRequestBuilder::new(Method::GET, url).build();

    let res = client.execute(req).await;
    assert!(res.is_err(), "Expected timeout or connection error");
}

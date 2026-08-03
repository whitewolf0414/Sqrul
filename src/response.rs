use reqwest::StatusCode;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub body: String,
    pub latency: Duration,
    pub final_url: String,
}

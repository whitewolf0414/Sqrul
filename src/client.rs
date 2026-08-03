use crate::config::ClientConfig;
use crate::error::Result;
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use reqwest::Client;
use std::time::Instant;

/// Maximum response body size accepted (5 MB).
const MAX_BODY_SIZE: usize = 5 * 1024 * 1024;

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.timeout)
            // Disable automatic redirects — prevents SSRF via redirect chains.
            .redirect(reqwest::redirect::Policy::none())
            .pool_idle_timeout(config.pool_idle_timeout)
            .build()?;

        Ok(Self { client })
    }

    pub async fn execute(&self, req: HttpRequest) -> Result<HttpResponse> {
        let start = Instant::now();

        let response = self.client.request(req.method, req.url).send().await?;

        let latency = start.elapsed();
        let status = response.status();
        let final_url = response.url().to_string();

        // Read body as raw bytes and enforce size limit.
        let bytes = response.bytes().await?;
        if bytes.len() > MAX_BODY_SIZE {
            return Err(crate::error::SqrulError::ConfigError(format!(
                "Response body ({} bytes) exceeds the {} byte limit.",
                bytes.len(),
                MAX_BODY_SIZE
            )));
        }
        let body = String::from_utf8_lossy(&bytes).into_owned();

        Ok(HttpResponse {
            status,
            body,
            latency,
            final_url,
        })
    }
}

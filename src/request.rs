use reqwest::{Method, Url};

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: Method,
    pub url: Url,
}

pub struct HttpRequestBuilder {
    method: Method,
    url: Url,
}

impl HttpRequestBuilder {
    pub fn new(method: Method, url: Url) -> Self {
        Self { method, url }
    }

    pub fn get(url: Url) -> Self {
        Self::new(Method::GET, url)
    }

    pub fn query(mut self, key: &str, value: &str) -> Self {
        self.url.query_pairs_mut().append_pair(key, value);
        self
    }

    pub fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url: self.url,
        }
    }
}

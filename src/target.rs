use reqwest::{Method, Url};

#[derive(Debug, Clone)]
pub struct Target {
    pub url: Url,
    pub method: Method,
    pub parameters: Vec<String>,
}

impl Target {
    pub fn new(url: Url, parameters: Vec<String>) -> Self {
        Self {
            url,
            method: Method::GET,
            parameters,
        }
    }
}

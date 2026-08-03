use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub timeout: Duration,
    pub max_redirects: usize,
    pub pool_idle_timeout: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct DetectionConfig {
    pub time_threshold: Duration,
    pub body_difference_threshold: i64,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub client: ClientConfig,
    pub detection: DetectionConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            client: ClientConfig {
                timeout: Duration::from_secs(10),
                max_redirects: 10,
                pool_idle_timeout: Some(Duration::from_secs(90)),
            },
            detection: DetectionConfig {
                time_threshold: Duration::from_secs(5),
                body_difference_threshold: 50,
            },
        }
    }
}

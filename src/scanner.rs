use crate::client::HttpClient;
use crate::config::DetectionConfig;
use crate::detector::{check_boolean_based, check_error_based, check_time_based};
use crate::error::Result;
use crate::finding::Finding;
use crate::payloads::{all_test_cases, Technique, TestCase};
use crate::request::HttpRequestBuilder;
use crate::response::HttpResponse;
use crate::target::Target;
use std::time::{Duration, Instant};
use tracing::{debug, info};

#[derive(Debug, Clone)]
pub struct ScanStatistics {
    pub requests_sent: usize,
    pub parameters_tested: usize,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub target: Target,
    pub findings: Vec<Finding>,
    pub statistics: ScanStatistics,
}

pub struct Scanner {
    client: HttpClient,
    config: DetectionConfig,
    test_cases: Vec<TestCase>,
}

impl Scanner {
    pub fn new(client: HttpClient, config: DetectionConfig) -> Self {
        Self {
            client,
            config,
            test_cases: all_test_cases(),
        }
    }

    pub async fn scan(&self, target: Target) -> Result<ScanResult> {
        let started_at = Instant::now();
        let mut findings = Vec::new();
        let mut requests_sent = 0;
        let parameters_tested = target.parameters.len();

        if target.parameters.is_empty() {
            info!("No parameters to test for target {}", target.url);
            return Ok(self.build_scan_result(
                target,
                findings,
                requests_sent,
                parameters_tested,
                started_at,
            ));
        }

        let baseline = self.build_and_send_baseline(&target).await?;
        requests_sent += 1;

        for param_name in &target.parameters {
            info!("Testing parameter '{}'", param_name);
            for test_case in &self.test_cases {
                debug!("Applying test case '{}'", test_case.id);
                requests_sent += self
                    .execute_test_case(&target, param_name, test_case, &baseline, &mut findings)
                    .await?;
            }
        }

        Ok(self.build_scan_result(
            target,
            findings,
            requests_sent,
            parameters_tested,
            started_at,
        ))
    }

    async fn build_and_send_baseline(&self, target: &Target) -> Result<HttpResponse> {
        info!("Building baseline request for {}", target.url);
        let mut base_req = HttpRequestBuilder::new(target.method.clone(), target.url.clone());
        for p in &target.parameters {
            base_req = base_req.query(p, "1");
        }
        let baseline = self.client.execute(base_req.build()).await?;
        debug!("Baseline request completed (status: {})", baseline.status);
        Ok(baseline)
    }

    async fn execute_test_case(
        &self,
        target: &Target,
        param_name: &str,
        test_case: &TestCase,
        baseline: &HttpResponse,
        findings: &mut Vec<Finding>,
    ) -> Result<usize> {
        let mut reqs_sent = 0;

        match test_case.technique {
            Technique::ErrorBased | Technique::TimeBased => {
                let probe = self
                    .send_probe(target, param_name, &format!("1{}", test_case.payload))
                    .await?;
                reqs_sent += 1;

                if test_case.technique == Technique::ErrorBased {
                    if let Some(f) =
                        check_error_based(target.url.as_str(), param_name, test_case, &probe)?
                    {
                        findings.push(f);
                    }
                } else {
                    if let Some(f) = check_time_based(
                        target.url.as_str(),
                        param_name,
                        test_case,
                        baseline,
                        &probe,
                        &self.config,
                    )? {
                        findings.push(f);
                    }
                }
            }
            Technique::BooleanBased => {
                if let Some(pair) = &test_case.boolean_pair {
                    let t_resp = self
                        .send_probe(target, param_name, &format!("1{}", pair.true_payload))
                        .await?;
                    reqs_sent += 1;
                    let f_resp = self
                        .send_probe(target, param_name, &format!("1{}", pair.false_payload))
                        .await?;
                    reqs_sent += 1;

                    if let Some(f) = check_boolean_based(
                        target.url.as_str(),
                        param_name,
                        test_case,
                        baseline,
                        &t_resp,
                        &f_resp,
                        &self.config,
                    )? {
                        findings.push(f);
                    }
                }
            }
        }
        Ok(reqs_sent)
    }

    async fn send_probe(
        &self,
        target: &Target,
        param_name: &str,
        payload: &str,
    ) -> Result<HttpResponse> {
        let mut req = HttpRequestBuilder::new(target.method.clone(), target.url.clone());
        for p in &target.parameters {
            if p == param_name {
                req = req.query(p, payload);
            } else {
                req = req.query(p, "1");
            }
        }
        self.client.execute(req.build()).await
    }

    fn build_scan_result(
        &self,
        target: Target,
        findings: Vec<Finding>,
        requests_sent: usize,
        parameters_tested: usize,
        started_at: Instant,
    ) -> ScanResult {
        let duration = started_at.elapsed();
        info!("Scan completed in {:?}", duration);
        ScanResult {
            target,
            findings,
            statistics: ScanStatistics {
                requests_sent,
                parameters_tested,
                duration,
            },
        }
    }
}

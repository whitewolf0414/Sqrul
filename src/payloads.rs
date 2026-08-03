use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Technique {
    ErrorBased,
    BooleanBased,
    TimeBased,
}

impl Technique {
    pub fn label(self) -> &'static str {
        match self {
            Self::ErrorBased => "Error-Based",
            Self::BooleanBased => "Boolean-Based",
            Self::TimeBased => "Time-Based (Blind)",
        }
    }

    pub fn remediation(self) -> &'static str {
        "Use parameterized queries / prepared statements and validate all \
         user-supplied input server-side. Never interpolate user data \
         directly into SQL strings."
    }
}

#[derive(Debug, Clone)]
pub struct TestCase {
    pub id: &'static str,
    pub technique: Technique,
    pub payload: &'static str,
    pub boolean_pair: Option<BooleanPair>,
    pub expected_delay: Option<Duration>,
}

#[derive(Debug, Clone, Copy)]
pub struct BooleanPair {
    pub true_payload: &'static str,
    pub false_payload: &'static str,
}

pub const ERROR_TESTS: &[TestCase] = &[
    TestCase {
        id: "sqli-err-quote",
        technique: Technique::ErrorBased,
        payload: "'",
        boolean_pair: None,
        expected_delay: None,
    },
    TestCase {
        id: "sqli-err-double-quote",
        technique: Technique::ErrorBased,
        payload: "\"",
        boolean_pair: None,
        expected_delay: None,
    },
    TestCase {
        id: "sqli-err-comment",
        technique: Technique::ErrorBased,
        payload: "' --",
        boolean_pair: None,
        expected_delay: None,
    },
];

pub const BOOLEAN_TESTS: &[TestCase] = &[TestCase {
    id: "sqli-bool-or",
    technique: Technique::BooleanBased,
    payload: "",
    boolean_pair: Some(BooleanPair {
        true_payload: "' OR '1'='1",
        false_payload: "' AND '1'='2",
    }),
    expected_delay: None,
}];

pub const TIME_TESTS: &[TestCase] = &[
    TestCase {
        id: "sqli-time-mysql",
        technique: Technique::TimeBased,
        payload: "' AND SLEEP(5)-- -",
        boolean_pair: None,
        expected_delay: Some(Duration::from_secs(5)),
    },
    TestCase {
        id: "sqli-time-pgsql",
        technique: Technique::TimeBased,
        payload: "'; SELECT pg_sleep(5)-- -",
        boolean_pair: None,
        expected_delay: Some(Duration::from_secs(5)),
    },
    TestCase {
        id: "sqli-time-mssql",
        technique: Technique::TimeBased,
        payload: "'; WAITFOR DELAY '0:0:5'-- -",
        boolean_pair: None,
        expected_delay: Some(Duration::from_secs(5)),
    },
];

pub fn all_test_cases() -> Vec<TestCase> {
    let mut all = Vec::new();
    all.extend_from_slice(ERROR_TESTS);
    all.extend_from_slice(BOOLEAN_TESTS);
    all.extend_from_slice(TIME_TESTS);
    all
}

pub const ERROR_SIGNATURES: &[&str] = &[
    "you have an error in your sql syntax",
    "warning: mysql_",
    "mysql_fetch",
    "unclosed quotation mark after the character string",
    "incorrect syntax near",
    "microsoft ole db provider for sql server",
    "ora-01756",
    "quoted string not properly terminated",
    "unterminated quoted string at or near",
    "sqlite_error",
];

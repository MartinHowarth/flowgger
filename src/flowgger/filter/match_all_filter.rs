use super::Filter;
use flowgger::record::{Record, SDValue, StructuredData, SEVERITY_MAX};

#[derive(Clone)]
pub struct MatchAllFilter;

#[derive(Clone)]
pub struct MatchAllFilterConfig {}


impl MatchAllFilter {
    pub fn new(config: &MatchAllFilterConfig) -> MatchAllFilter {
        MatchAllFilter
    }
}

impl Filter for MatchAllFilter {
    fn condition(&self, record: &Record) -> bool {
        true
    }
    fn action(&self, record: &mut Record) -> () {
    }
}

#[test]
fn test_match_all() {
    let test_config = MatchAllFilterConfig {
    };

    let record = Record {
        ts: 1385053862.3072,
        hostname: "example.org".to_string(),
        facility: None,
        severity: Some(1 as u8),
        appname: None,
        procid: None,
        msgid: None,
        msg: Some("A short message that helps you identify what is going on".to_string()),
        full_msg: Some("Backtrace here\n\nmore stuff".to_string()),
        sd: None,
    };
    MatchAllFilter::new(&test_config).condition(&record);
}

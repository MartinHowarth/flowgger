use super::Filter;
use flowgger::record::{Record, SDValue, StructuredData, SEVERITY_MAX};

#[derive(Clone)]
pub struct HostFilter {
    hostname_match: String
}

#[derive(Clone)]
pub struct HostFilterConfig {
    hostname_match: String,
}

impl HostFilter {
    pub fn new(config: &HostFilterConfig) -> HostFilter {
        HostFilter {
            hostname_match: config.hostname_match.to_owned(),
        }
    }
}

impl Filter for HostFilter {
    fn condition(&self, record: &Record) -> bool {
        record.hostname == self.hostname_match
    }
    fn action(&self, record: &mut Record) -> () {

    }
}

#[test]
fn test_host_filter() {

    let config = HostFilterConfig {
        hostname_match: "example.org".to_string()
    };

    let mut record = Record {
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
    assert_eq!(HostFilter::new(&config).condition(&mut record), true);
}

#[test]
fn test_host_filter_condition_fail() {

    let config = HostFilterConfig {
        hostname_match: "no.match".to_string()
    };

    let mut record = Record {
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
    assert_eq!(HostFilter::new(&config).condition(&mut record), false);
}

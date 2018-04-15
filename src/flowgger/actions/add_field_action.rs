use super::Action;
use flowgger::record::{Record, SDValue, StructuredData, SEVERITY_MAX};

#[derive(Clone)]
pub struct AddFieldAction {
    field_name: String,
    field_value: String,
}

#[derive(Clone)]
pub struct AddFieldActionConfig {
    field_name: String,
    field_value: String,
}

impl AddFieldAction {
    pub fn new(config: &AddFieldActionConfig) -> AddFieldAction {
        AddFieldAction {
            field_name: config.field_name.to_owned(),
            field_value: config.field_value.to_owned(),
        }
    }
}

impl Action for AddFieldAction {
    fn apply(&self, record: &mut Record) -> () {

        match record.sd {
            Some(ref mut x) => x.pairs.push((self.field_name.to_owned(), SDValue::String(self.field_value.to_owned()))),
            None        => {
                let mut sd = StructuredData::new(None);
                sd.pairs.push((self.field_name.to_owned(), SDValue::String(self.field_value.to_owned())));
                record.sd = Some(sd)
            },
        }
    }
}

#[test]
fn test_add_field_action() {
    let test_config = AddFieldActionConfig {
        field_name: "test_name".to_string(),
        field_value: "test_value".to_string(),
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
        sd: Some(StructuredData::new(None)),
    };
    AddFieldAction::new(&test_config).apply(&mut record);

    let sd = record.sd.unwrap();
    let pairs = sd.pairs;
    assert!(
        pairs
            .iter()
            .cloned()
            .any(| (k, v) | if let SDValue::String(v) = v {
                k == test_config.field_name && v == test_config.field_value
            } else {
                false
            })
    );
}

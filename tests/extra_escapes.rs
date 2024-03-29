extern crate serde;
#[macro_use]
extern crate serde_json_lenient;

use serde_json_lenient::{from_str_lenient, Value};

#[test]
fn test_vertical_tab() {
    let s = r#" { "key": "value\v" }"#;
    let value: Value = from_str_lenient(s).unwrap();
    assert_eq!(value, json!({"key": "value\x0b"}));
}

#[test]
fn test_two_digit_escape() {
    let s = r#" { "key": "value\x0b" }"#;
    let value: Value = from_str_lenient(s).unwrap();
    assert_eq!(value, json!({"key": "value\x0b"}));
}

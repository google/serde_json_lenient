extern crate serde;

#[macro_use]
extern crate serde_json_lenient;

use serde::de::Deserialize;
use serde_json_lenient::de::SliceRead;
use serde_json_lenient::{Deserializer, Error, Value};

fn from_slice_with_unicode_substitution(s: &[u8]) -> Result<Value, Error> {
    let mut de = Deserializer::new(SliceRead::new(s, true, false, false, false, false));
    Deserialize::deserialize(&mut de)
}

#[test]
fn test_invalid_characters() {
    let prefix = r#" { "key": "value"#;
    let suffix = r#"" }"#;
    let mut s: Vec<u8> = vec![];
    s.extend(prefix.as_bytes());
    s.extend(&[0xed, 0xa0, 0x80]);
    s.extend(suffix.as_bytes());
    let value = from_slice_with_unicode_substitution(&s).unwrap();
    assert_eq!(value, json!({"key": "value\u{fffd}\u{fffd}\u{fffd}"}));
}

#[test]
fn test_invalid_utf16_escape_sequence_lone_low_surrogate() {
    let s = "{\"key\": \"value=\\udfff_\"}".as_bytes();
    let value: Value = from_slice_with_unicode_substitution(s).unwrap();
    assert_eq!(value, json!({"key": "value=\u{fffd}_"}));
}

#[test]
fn test_invalid_utf16_escape_sequence_lone_high_surrogate() {
    let s = "{\"key\": \"value=\\ud800_\"}".as_bytes();
    let value: Value = from_slice_with_unicode_substitution(s).unwrap();
    assert_eq!(value, json!({"key": "value=\u{fffd}_"}));
}

#[test]
fn test_invalid_utf16_escape_sequence_lone_low_surrogate_precedes_escape() {
    let s = "{\"key\": \"value=\\udfff\\\"x\"}".as_bytes();
    let value: Value = from_slice_with_unicode_substitution(s).unwrap();
    assert_eq!(value, json!({"key": "value=\u{fffd}\"x"}));
}

#[test]
fn test_invalid_utf16_escape_sequence_lone_high_surrogate_precedes_escape() {
    let s = "{\"key\": \"value=\\ud800\\\"x\"}".as_bytes();
    let value: Value = from_slice_with_unicode_substitution(s).unwrap();
    assert_eq!(value, json!({"key": "value=\u{fffd}\"x"}));
}

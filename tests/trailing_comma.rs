#[macro_use]
extern crate serde_jsonrc;

use serde_jsonrc::{from_str, Value};

#[test]
fn test_trailing_comma_object() {
    let s = r#"
    {
        "key": "value",
    }"#;
    let value: Value = from_str(s).unwrap();
    assert_eq!(value, json!({"key": "value"}));
}

#[test]
fn test_trailing_comma_array() {
    let s = r#"
    {
        "key": [
            "one",
            "two",
            "three",
        ]
    }"#;
    let value: Value = from_str(s).unwrap();
    assert_eq!(value, json!({"key": ["one", "two", "three"]}));
}

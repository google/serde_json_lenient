#[macro_use]
extern crate serde_jsonrc;

use serde_jsonrc::{from_str, Value};

#[test]
fn test_trailing_comma() {
    let s = r#"
    {
        "key": "value",
    }"#;
    let value: Value = from_str(s).unwrap();
    assert_eq!(value, json!({"key": "value"}));
}

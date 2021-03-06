extern crate serde;

#[macro_use]
extern crate serde_json_lenient;

use serde_json_lenient::{from_str, Value};

#[test]
fn test_non_character() {
    let s = r#"
    {
        "key": "value\ufdef\uffff"
    }"#;
    let value: Value = from_str(s).unwrap();
    assert_eq!(value, json!({"key": "value\u{fffd}\u{fffd}"}));
}

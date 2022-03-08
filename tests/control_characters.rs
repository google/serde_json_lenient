extern crate serde;

#[macro_use]
extern crate serde_json_lenient;

use serde_json_lenient::{from_str_lenient, Value};

#[test]
fn test_control_character() {
    let s = "{ \"key\": \"a\0b\nc\" }";
    let value: Value = from_str_lenient(s).unwrap();
    assert_eq!(value, json!({"key": "a\0b\nc"}));
}

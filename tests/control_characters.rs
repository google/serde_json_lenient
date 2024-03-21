extern crate serde;

#[macro_use]
extern crate serde_json_lenient;

use serde::de::Deserialize;
use serde_json_lenient::de::SliceRead;
use serde_json_lenient::{from_str_lenient, Deserializer, Error, Value};

struct TestOptions {
    allow_newlines_in_string: bool,
    allow_control_characters_in_string: bool,
}

impl TestOptions {
    fn deserialize(&self, s: &[u8]) -> Result<Value, Error> {
        let mut de = Deserializer::new(SliceRead::new(
            s,
            true,
            self.allow_newlines_in_string,
            self.allow_control_characters_in_string,
            false,
            false,
        ));
        Deserialize::deserialize(&mut de)
    }
}

#[test]
fn test_control_character_lenient() {
    let s = "{ \"key\": \"a\0b\nc\" }";
    let value: Value = from_str_lenient(s).unwrap();
    assert_eq!(value, json!({"key": "a\0b\nc"}));
}

#[test]
fn test_not_newlines_not_control() {
    let opts = TestOptions {
        allow_newlines_in_string: false,
        allow_control_characters_in_string: false,
    };
    assert_eq!(opts.deserialize(b"\"abc\"").unwrap(), json!("abc"));
    assert!(opts.deserialize(b"\"a\nb\"").is_err());
    assert!(opts.deserialize(b"\"a\rb\"").is_err());
    assert!(opts.deserialize(b"\"a\x10b\"").is_err());
    assert!(opts.deserialize(b"\"a\x08b\x1fc\"").is_err());
}

#[test]
fn test_newlines_and_control() {
    let opts = TestOptions {
        allow_newlines_in_string: true,
        allow_control_characters_in_string: true,
    };
    assert_eq!(opts.deserialize(b"\"abc\"").unwrap(), json!("abc"));
    assert_eq!(opts.deserialize(b"\"a\nb\"").unwrap(), json!("a\nb"));
    assert_eq!(opts.deserialize(b"\"a\rb\"").unwrap(), json!("a\rb"));
    assert_eq!(opts.deserialize(b"\"a\x10b\"").unwrap(), json!("a\x10b"));
    assert_eq!(
        opts.deserialize(b"\"a\x08b\x1fc\"").unwrap(),
        json!("a\x08b\x1fc")
    );
}

#[test]
fn test_newlines_but_not_control() {
    let opts = TestOptions {
        allow_newlines_in_string: true,
        allow_control_characters_in_string: false,
    };
    assert_eq!(opts.deserialize(b"\"abc\"").unwrap(), json!("abc"));
    assert_eq!(opts.deserialize(b"\"a\nb\"").unwrap(), json!("a\nb"));
    assert_eq!(opts.deserialize(b"\"a\rb\"").unwrap(), json!("a\rb"));
    assert!(opts.deserialize(b"\"a\x10b\"").is_err());
    assert!(opts.deserialize(b"\"a\x08b\x1fc\"").is_err());
}

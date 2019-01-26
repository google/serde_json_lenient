#[macro_use]
extern crate serde_derive;

extern crate serde;

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum Animal {
    Dog,
    Frog(String, Vec<isize>),
    Cat { age: usize, name: String },
    AntHive(Vec<String>),
}

#[test]
fn test_parse_enum_animal_and_no_other_test() {
    let animal: Animal = from_str("{\"Cat\":[0, \"Kate\",]}").unwrap();
    assert_eq!(
        animal,
        Animal::Cat {
            age: 0,
            name: "Kate".to_string()
        }
    );
}

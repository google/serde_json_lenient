//! # Serde jsonrc
//!
//! **This crate is a fork of [Serde JSON](https://github.com/serde-rs/json)**
//! that accepts a more lenient form of JSON. Specifically, it allows:
//!
//! - `/*` and `//` style comments.
//! - Trailing commas for object and array literals.
//!
//! This is designed to make it easier to use JSON files that are maintained
//! by hand, such as configuration files.
//!
//! JSON is a ubiquitous open-standard format that uses human-readable text to
//! transmit data objects consisting of key-value pairs.
//!
//! ```json
//! {
//!     "name": "John Doe",
//!     "age": 43,
//!     "address": {
//!         "street": "10 Downing Street",
//!         "city": "London"
//!     },
//!     "phones": [
//!         "+44 1234567",
//!         "+44 2345678"
//!     ]
//! }
//! ```
//!
//! There are three common ways that you might find yourself needing to work
//! with JSON data in Rust.
//!
//!  - **As text data.** An unprocessed string of JSON data that you receive on
//!    an HTTP endpoint, read from a file, or prepare to send to a remote
//!    server.
//!  - **As an untyped or loosely typed representation.** Maybe you want to
//!    check that some JSON data is valid before passing it on, but without
//!    knowing the structure of what it contains. Or you want to do very basic
//!    manipulations like insert a key in a particular spot.
//!  - **As a strongly typed Rust data structure.** When you expect all or most
//!    of your data to conform to a particular structure and want to get real
//!    work done without JSON's loosey-goosey nature tripping you up.
//!
//! Serde jsonrc provides efficient, flexible, safe ways of converting data
//! between each of these representations.
//!
//! # Operating on untyped JSON values
//!
//! Any valid JSON data can be manipulated in the following recursive enum
//! representation. This data structure is [`serde_json_lenient::Value`][value].
//!
//! ```
//! # use serde_json_lenient::{Number, Map};
//! #
//! # #[allow(dead_code)]
//! enum Value {
//!     Null,
//!     Bool(bool),
//!     Number(Number),
//!     String(String),
//!     Array(Vec<Value>),
//!     Object(Map<String, Value>),
//! }
//! ```
//!
//! A string of JSON data can be parsed into a `serde_json_lenient::Value` by the
//! [`serde_json_lenient::from_str`][from_str] function. There is also
//! [`from_slice`][from_slice] for parsing from a byte slice &[u8] and
//! [`from_reader`][from_reader] for parsing from any `io::Read` like a File or
//! a TCP stream.
//!
//! ```
//! use serde_json_lenient::{Result, Value};
//!
//! fn untyped_example() -> Result<()> {
//!     // Some JSON input data as a &str. Maybe this comes from the user.
//!     let data = r#"
//!         {
//!             "name": "John Doe",
//!             "age": 43,
//!             "phones": [
//!                 "+44 1234567",
//!                 "+44 2345678"
//!             ]
//!         }"#;
//!
//!     // Parse the string of data into serde_json_lenient::Value.
//!     let v: Value = serde_json_lenient::from_str(data)?;
//!
//!     // Access parts of the data by indexing with square brackets.
//!     println!("Please call {} at the number {}", v["name"], v["phones"][0]);
//!
//!     Ok(())
//! }
//! #
//! # fn main() {
//! #     untyped_example().unwrap();
//! # }
//! ```
//!
//! The result of square bracket indexing like `v["name"]` is a borrow of the
//! data at that index, so the type is `&Value`. A JSON map can be indexed with
//! string keys, while a JSON array can be indexed with integer keys. If the
//! type of the data is not right for the type with which it is being indexed,
//! or if a map does not contain the key being indexed, or if the index into a
//! vector is out of bounds, the returned element is `Value::Null`.
//!
//! When a `Value` is printed, it is printed as a JSON string. So in the code
//! above, the output looks like `Please call "John Doe" at the number "+44
//! 1234567"`. The quotation marks appear because `v["name"]` is a `&Value`
//! containing a JSON string and its JSON representation is `"John Doe"`.
//! Printing as a plain string without quotation marks involves converting from
//! a JSON string to a Rust string with [`as_str()`] or avoiding the use of
//! `Value` as described in the following section.
//!
//! [`as_str()`]: crate::Value::as_str
//!
//! The `Value` representation is sufficient for very basic tasks but can be
//! tedious to work with for anything more significant. Error handling is
//! verbose to implement correctly, for example imagine trying to detect the
//! presence of unrecognized fields in the input data. The compiler is powerless
//! to help you when you make a mistake, for example imagine typoing `v["name"]`
//! as `v["nmae"]` in one of the dozens of places it is used in your code.
//!
//! # Parsing JSON as strongly typed data structures
//!
//! Serde provides a powerful way of mapping JSON data into Rust data structures
//! largely automatically.
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use serde_json_lenient::Result;
//!
//! #[derive(Serialize, Deserialize)]
//! struct Person {
//!     name: String,
//!     age: u8,
//!     phones: Vec<String>,
//! }
//!
//! fn typed_example() -> Result<()> {
//!     // Some JSON input data as a &str. Maybe this comes from the user.
//!     let data = r#"
//!         {
//!             "name": "John Doe",
//!             "age": 43,
//!             "phones": [
//!                 "+44 1234567",
//!                 "+44 2345678"
//!             ]
//!         }"#;
//!
//!     // Parse the string of data into a Person object. This is exactly the
//!     // same function as the one that produced serde_json_lenient::Value above, but
//!     // now we are asking it for a Person as output.
//!     let p: Person = serde_json_lenient::from_str(data)?;
//!
//!     // Do things just like with any other Rust data structure.
//!     println!("Please call {} at the number {}", p.name, p.phones[0]);
//!
//!     Ok(())
//! }
//! #
//! # fn main() {
//! #     typed_example().unwrap();
//! # }
//! ```
//!
//! This is the same `serde_json_lenient::from_str` function as before, but this time we
//! assign the return value to a variable of type `Person` so Serde will
//! automatically interpret the input data as a `Person` and produce informative
//! error messages if the layout does not conform to what a `Person` is expected
//! to look like.
//!
//! Any type that implements Serde's `Deserialize` trait can be deserialized
//! this way. This includes built-in Rust standard library types like `Vec<T>`
//! and `HashMap<K, V>`, as well as any structs or enums annotated with
//! `#[derive(Deserialize)]`.
//!
//! Once we have `p` of type `Person`, our IDE and the Rust compiler can help us
//! use it correctly like they do for any other Rust code. The IDE can
//! autocomplete field names to prevent typos, which was impossible in the
//! `serde_json_lenient::Value` representation. And the Rust compiler can check that
//! when we write `p.phones[0]`, then `p.phones` is guaranteed to be a
//! `Vec<String>` so indexing into it makes sense and produces a `String`.
//!
//! # Constructing JSON values
//!
//! Serde jsonrc provides a [`json!` macro][macro] to build `serde_json_lenient::Value`
//! objects with very natural JSON syntax.
//!
//! ```
//! use serde_json_lenient::json;
//!
//! fn main() {
//!     // The type of `john` is `serde_json_lenient::Value`
//!     let john = json!({
//!         "name": "John Doe",
//!         "age": 43,
//!         "phones": [
//!             "+44 1234567",
//!             "+44 2345678"
//!         ]
//!     });
//!
//!     println!("first phone number: {}", john["phones"][0]);
//!
//!     // Convert to a string of JSON and print it out
//!     println!("{}", john.to_string());
//! }
//! ```
//!
//! The `Value::to_string()` function converts a `serde_json_lenient::Value` into a
//! `String` of JSON text.
//!
//! One neat thing about the `json!` macro is that variables and expressions can
//! be interpolated directly into the JSON value as you are building it. Serde
//! will check at compile time that the value you are interpolating is able to
//! be represented as JSON.
//!
//! ```
//! # use serde_json_lenient::json;
//! #
//! # fn random_phone() -> u16 { 0 }
//! #
//! let full_name = "John Doe";
//! let age_last_year = 42;
//!
//! // The type of `john` is `serde_json_lenient::Value`
//! let john = json!({
//!     "name": full_name,
//!     "age": age_last_year + 1,
//!     "phones": [
//!         format!("+44 {}", random_phone())
//!     ]
//! });
//! ```
//!
//! This is amazingly convenient, but we have the problem we had before with
//! `Value`: the IDE and Rust compiler cannot help us if we get it wrong. Serde
//! JSON provides a better way of serializing strongly-typed data structures
//! into JSON text.
//!
//! # Creating JSON by serializing data structures
//!
//! A data structure can be converted to a JSON string by
//! [`serde_json_lenient::to_string`][to_string]. There is also
//! [`serde_json_lenient::to_vec`][to_vec] which serializes to a `Vec<u8>` and
//! [`serde_json_lenient::to_writer`][to_writer] which serializes to any `io::Write`
//! such as a File or a TCP stream.
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use serde_json_lenient::Result;
//!
//! #[derive(Serialize, Deserialize)]
//! struct Address {
//!     street: String,
//!     city: String,
//! }
//!
//! fn print_an_address() -> Result<()> {
//!     // Some data structure.
//!     let address = Address {
//!         street: "10 Downing Street".to_owned(),
//!         city: "London".to_owned(),
//!     };
//!
//!     // Serialize it to a JSON string.
//!     let j = serde_json_lenient::to_string(&address)?;
//!
//!     // Print, write to a file, or send to an HTTP server.
//!     println!("{}", j);
//!
//!     Ok(())
//! }
//! #
//! # fn main() {
//! #     print_an_address().unwrap();
//! # }
//! ```
//!
//! Any type that implements Serde's `Serialize` trait can be serialized this
//! way. This includes built-in Rust standard library types like `Vec<T>` and
//! `HashMap<K, V>`, as well as any structs or enums annotated with
//! `#[derive(Serialize)]`.
//!
//! # No-std support
//!
//! As long as there is a memory allocator, it is possible to use serde_json
//! without the rest of the Rust standard library. Disable the default "std"
//! feature and enable the "alloc" feature:
//!
//! ```toml
//! [dependencies]
//! serde_json_lenient = { version = "1.0", default-features = false, features = ["alloc"] }
//! ```
//!
//! For JSON support in Serde without a memory allocator, please see the
//! [`serde-json-core`] crate.
//!
//! [value]: crate::value::Value
//! [from_str]: crate::de::from_str
//! [from_slice]: crate::de::from_slice
//! [from_reader]: crate::de::from_reader
//! [to_string]: crate::ser::to_string
//! [to_vec]: crate::ser::to_vec
//! [to_writer]: crate::ser::to_writer
//! [macro]: crate::json
//! [`serde-json-core`]: https://github.com/rust-embedded-community/serde-json-core

#![doc(html_root_url = "https://docs.rs/serde_json_lenient/0.1.8")]
// Ignored clippy lints
#![allow(
    clippy::collapsible_else_if,
    clippy::comparison_chain,
    clippy::deprecated_cfg_attr,
    clippy::doc_markdown,
    clippy::excessive_precision,
    clippy::explicit_auto_deref,
    clippy::float_cmp,
    clippy::manual_range_contains,
    clippy::match_like_matches_macro,
    clippy::match_single_binding,
    clippy::needless_doctest_main,
    clippy::needless_late_init,
    clippy::return_self_not_must_use,
    clippy::transmute_ptr_to_ptr,
    clippy::unnecessary_wraps
)]
// Ignored clippy_pedantic lints
#![allow(
    // Deserializer::from_str, into_iter
    clippy::should_implement_trait,
    // integer and float ser/de requires these sorts of casts
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    // correctly used
    clippy::enum_glob_use,
    clippy::if_not_else,
    clippy::integer_division,
    clippy::let_underscore_untyped,
    clippy::map_err_ignore,
    clippy::match_same_arms,
    clippy::similar_names,
    clippy::unused_self,
    clippy::wildcard_imports,
    // things are often more readable this way
    clippy::cast_lossless,
    clippy::module_name_repetitions,
    clippy::redundant_else,
    clippy::shadow_unrelated,
    clippy::single_match_else,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix,
    clippy::use_self,
    clippy::zero_prefixed_literal,
    // we support older compilers
    clippy::checked_conversions,
    clippy::mem_replace_with_default,
    // noisy
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
)]
// Restrictions
#![deny(clippy::question_mark_used)]
#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

#[cfg(feature = "std")]
#[doc(inline)]
pub use crate::de::from_reader;
#[doc(inline)]
pub use crate::de::{from_slice, from_str, from_str_lenient, Deserializer, StreamDeserializer};
#[doc(inline)]
pub use crate::error::{Error, Result};
#[doc(inline)]
pub use crate::ser::{to_string, to_string_pretty, to_vec, to_vec_pretty};
#[cfg(feature = "std")]
#[doc(inline)]
pub use crate::ser::{to_writer, to_writer_pretty, Serializer};
#[doc(inline)]
pub use crate::value::{from_value, to_value, Map, Number, Value};

// We only use our own error type; no need for From conversions provided by the
// standard library's try! macro. This reduces lines of LLVM IR by 4%.
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}

#[macro_use]
mod macros;

pub mod de;
pub mod error;
pub mod map;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod ser;
#[cfg(not(feature = "std"))]
mod ser;
pub mod value;

mod features_check;

mod io;
#[cfg(feature = "std")]
mod iter;
#[cfg(feature = "float_roundtrip")]
mod lexical;
mod number;
mod read;

#[cfg(feature = "raw_value")]
mod raw;

//! Demonstrates validating an already encoded string and reacting to errors.
//! Run: `cargo run --example validation`

use oi4_dnp_encoding::{validate_dnp, Rules, ErrorKind};

fn main() {
    let encoded_ok = "Hello,20World,21";
    validate_dnp(encoded_ok, &Rules::default()).expect("should validate");
    println!("validated OK: {encoded_ok}");

    // Example with an illegal unescaped reserved ASCII (space) under enforced masking rules
    let rules = Rules { enforce_reserved_masking: true, allow_lowercase_hex: true };
    let bad = "Hello World"; // space should be escaped as ,20 under these rules
    match validate_dnp(bad, &rules) {
        Ok(_) => println!("unexpected success"),
        Err(e) => match e.kind() { // pattern match by kind
            ErrorKind::UnescapedReservedAscii(ch) => println!("expected error: unescaped reserved ASCII '{ch}' at pos {:?}", e.position()),
            other => println!("other validation error: {other:?}"),
        }
    }
}


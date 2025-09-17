//! Demonstrates behavior under the `strict` feature.
//! Run with: `cargo run --example strict --features strict`
//! (The example will intentionally show errors that only appear in strict mode.)

#[cfg(all(feature = "alloc", feature = "strict"))]
fn main() {
    use oi4_dnp_encoding::{decode, encode, validate_dnp, ErrorKind, Rules};

    let s = "Hello World!";
    let enc = encode(s);
    assert_eq!(&enc, "Hello,20World,21");
    println!("encoded strict = {enc}");

    // Convert entire encoded string to lowercase -> should be rejected in strict mode.
    let bad_lower = enc.to_lowercase();
    if bad_lower != enc {
        // ensure different
        match decode(&bad_lower) {
            Ok(_) => println!("unexpected success (lowercase accepted)"),
            Err(e) => println!("expected lowercase rejection: {e:?}"),
        }
    }

    // Unescaped reserved ASCII should fail validation under strict-like rules.
    let rules = Rules::strict_like();
    let bad_unescaped = "Hello World!"; // space and ! must be escaped
    match validate_dnp(bad_unescaped, &rules) {
        Ok(_) => println!("unexpected success (unescaped reserved)"),
        Err(e) => match e.kind() {
            ErrorKind::UnescapedReservedAscii(ch) => println!(
                "expected unescaped reserved error for '{ch}' at {:?}",
                e.position()
            ),
            other => println!("unexpected error kind: {other:?}"),
        },
    }
}

#[cfg(not(all(feature = "alloc", feature = "strict")))]
fn main() {
    println!("Enable features: --features strict (alloc implied by default std feature).\nExample: cargo run --example strict --features strict");
}

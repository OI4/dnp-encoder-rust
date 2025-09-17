//! Basic encode / decode / validate usage.
//! Run: `cargo run --example basic`

#[cfg(feature = "alloc")]
fn main() {
    use oi4_dnp_encoding::{decode, encode, validate_dnp, Rules};

    let original = "Hello World!"; // space + '!' must be escaped
    let encoded = encode(original);
    println!("encoded = {encoded}");
    assert_eq!(encoded, "Hello,20World,21");

    let decoded = decode(&encoded).expect("decode should succeed");
    assert_eq!(decoded, original);

    // Validate already-encoded string (default rules: allow lowercase hex, don't enforce masking for reserved ASCII)
    validate_dnp(&encoded, &Rules::default()).expect("validation should pass");
    println!("decoded = {decoded}");
}

#[cfg(not(feature = "alloc"))]
fn main() {
    // Compile-time hint if someone tries to run with --no-default-features and without alloc.
    println!("This example requires the 'alloc' feature (enabled by default).\nRun without --no-default-features.");
}

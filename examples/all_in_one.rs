//! All-in-one demonstration of typical usage patterns.
//! Run: `cargo run --example all_in_one`
//! With strict feature: `cargo run --example all_in_one --features strict`
//! Without std/alloc (no heap): `cargo run --example all_in_one --no-default-features`
//!
//! This example adapts to enabled features so users can see what is available.

use oi4_dnp_encoding::{validate_dnp, Rules};

#[cfg(feature = "alloc")]
use oi4_dnp_encoding::{decode, encode};
#[cfg(not(feature = "alloc"))]
use oi4_dnp_encoding::{encode_into, encoded_len};

#[cfg(not(feature = "alloc"))]
use core::fmt::Write as _;

// Small fixed buffer writer used only in no-alloc mode.
#[cfg(not(feature = "alloc"))]
struct Fixed<const N: usize> {
    buf: [u8; N],
    pos: usize,
}
#[cfg(not(feature = "alloc"))]
impl<const N: usize> Fixed<N> {
    fn new() -> Self {
        Self {
            buf: [0; N],
            pos: 0,
        }
    }
    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[..self.pos]).unwrap()
    }
}
#[cfg(not(feature = "alloc"))]
impl<const N: usize> core::fmt::Write for Fixed<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let b = s.as_bytes();
        if self.pos + b.len() > N {
            return Err(core::fmt::Error);
        }
        self.buf[self.pos..self.pos + b.len()].copy_from_slice(b);
        self.pos += b.len();
        Ok(())
    }
}

fn main() {
    let input = "Hello World!"; // contains space & '!'

    // --- Encoding / Decoding ---
    #[cfg(feature = "alloc")]
    {
        let enc = encode(input);
        println!("encode -> {enc}");
        assert_eq!(enc, "Hello,20World,21");
        let dec = decode(&enc).expect("decode ok");
        assert_eq!(dec, input);
        println!("decode -> {dec}");
    }

    #[cfg(not(feature = "alloc"))]
    {
        let needed = encoded_len(input);
        assert!(needed <= 256, "adjust buffer size");
        let mut fb: Fixed<256> = Fixed::new();
        encode_into(input, &mut fb).expect("fits");
        let enc = fb.as_str();
        println!("encode (no alloc) -> {enc}");
    }

    // --- Validation (default rules) ---
    let encoded_sample = "Hello,20World,21"; // canonical form
    validate_dnp(encoded_sample, &Rules::default()).expect("valid default");
    println!("validate default rules OK");

    // Enforce masking for reserved ASCII
    let masking_rules = Rules {
        enforce_reserved_masking: true,
        ..Rules::default()
    };
    if let Err(e) = validate_dnp("Hello World!", &masking_rules) {
        // space & ! must be escaped
        println!(
            "expected masking error: {:?} at {:?}",
            e.kind(),
            e.position()
        );
    }

    // --- Strict-like demonstration (independent of compile-time strict feature) ---
    let strict_like = Rules::strict_like();
    if let Err(e) = validate_dnp("Hello World!", &strict_like) {
        println!("strict_like unescaped error: {:?}", e.kind());
    }

    // --- Lowercase hex acceptance vs strict feature ---
    #[cfg(feature = "alloc")]
    {
        let lower = "Hello,20World,21".to_lowercase(); // contains lowercase hex digits
        let res = decode(&lower);
        #[cfg(feature = "strict")]
        match res {
            Ok(_) => println!("unexpected lowercase accepted"),
            Err(e) => println!(
                "strict feature active -> lowercase decode error kind = {:?}",
                e.kind()
            ),
        }
        #[cfg(not(feature = "strict"))]
        println!("non-strict -> lowercase accepted -> {:?}", res.unwrap());
    }

    println!("(all_in_one example done)");
}

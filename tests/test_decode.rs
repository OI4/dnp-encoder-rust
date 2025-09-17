#![cfg(feature = "alloc")]
use oi4_dnp_encoding::{decode, encode};

// Property & unit tests for decode

#[test]
fn decode_basic_roundtrip() {
    let s = "Hello World!";
    let enc = encode(s);
    let dec = decode(&enc).unwrap();
    assert_eq!(dec, s);
}

#[test]
fn decode_unicode_passthrough() {
    let s = "äöüΩ🙂";
    let enc = encode(s);
    assert_eq!(enc, s);
    assert_eq!(decode(&enc).unwrap(), s);
}

#[test]
fn decode_invalid_hex_digit() {
    assert!(decode(",2G").is_err());
    assert!(decode(",G2").is_err());
}

#[test]
fn decode_lone_or_short_escape() {
    assert!(decode(",").is_err());
    assert!(decode(",2").is_err());
}

#[test]
fn decode_escape_single_ascii() {
    assert_eq!(decode(",41").unwrap(), "A");
}

#[cfg(not(feature = "strict"))]
#[test]
fn decode_lowercase_hex_triplets() {
    assert_eq!(decode(",61,62,63").unwrap(), "abc");
}

#[cfg(feature = "strict")]
#[test]
fn decode_lowercase_rejected_strict() {
    // Should reject because of lowercase 'f' (0x6F should be encoded as ,6F in strict mode)
    assert!(decode(",6f").is_err());
}

#[test]
fn decode_all_ascii_triplets() {
    for b in 0x00u8..=0x7Fu8 {
        let hi = b >> 4;
        let lo = b & 0x0F;
        let hex = |n: u8| -> char {
            match n {
                0..=9 => (b'0' + n) as char,
                10..=15 => (b'A' + (n - 10)) as char,
                _ => '?',
            }
        };
        let triplet = format!(",{}{}", hex(hi), hex(lo));
        let out = decode(&triplet).unwrap();
        assert_eq!(
            out.chars().next().unwrap() as u8,
            b,
            "Mismatch for {triplet}"
        );
    }
}

// Property test for roundtrip (limited length)
#[cfg(feature = "std")]
mod prop {
    use super::*;
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn prop_roundtrip_random(data in "(?s).{0,256}") {
            let enc = encode(&data);
            let dec = decode(&enc).unwrap();
            prop_assert_eq!(dec, data);
        }
    }
}

//! Encode without heap allocation using `encode_into` + a fixed stack buffer.
//! Run: `cargo run --example encode_into_no_alloc`
//! Can also be built with `--no-default-features` (no std) because it only uses core APIs.

use core::fmt::{self, Write};
use oi4_dnp_encoding::{encode_into, encoded_len};

struct FixedBuf<const N: usize> {
    buf: [u8; N],
    pos: usize,
}

impl<const N: usize> FixedBuf<N> {
    const fn new() -> Self {
        Self {
            buf: [0; N],
            pos: 0,
        }
    }
    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[..self.pos]).unwrap()
    }
}

impl<const N: usize> Write for FixedBuf<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        if self.pos + bytes.len() > N {
            return Err(fmt::Error);
        }
        self.buf[self.pos..self.pos + bytes.len()].copy_from_slice(bytes);
        self.pos += bytes.len();
        Ok(())
    }
}

fn main() {
    let input = "Hello World!";
    let required = encoded_len(input);
    assert!(required <= 128, "adjust FixedBuf size if this panics");

    let mut fb: FixedBuf<128> = FixedBuf::new();
    encode_into(input, &mut fb).expect("write fits into buffer");

    let encoded = fb.as_str();
    assert_eq!(encoded, "Hello,20World,21");
    println!("encoded (stack only) = {encoded}");
}

use core::fmt;

#[inline]
pub(crate) const fn is_unreserved(b: u8) -> bool {
    matches!(b,
        b'A'..=b'Z' |
        b'a'..=b'z' |
        b'0'..=b'9' |
        b'-' | b'.' | b'_' | b'~'
    )
}

const HEX_UPPER: &[u8; 16] = b"0123456789ABCDEF";

/// Compute encoded length without allocating.
pub fn encoded_len(input: &str) -> usize {
    let bytes = input.as_bytes();
    let mut i = 0usize;
    let mut len = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        if b < 0x80 {
            // ASCII
            if is_unreserved(b) {
                len += 1;
            } else {
                len += 3;
            }
            i += 1;
        } else {
            // multi-byte UTF-8 sequence – copy verbatim
            // Determine sequence length from first byte (UTF-8 invariant; input is &str)
            let seq_len = if b & 0b1110_0000 == 0b1100_0000 {
                2
            } else if b & 0b1111_0000 == 0b1110_0000 {
                3
            } else {
                4
            };
            len += seq_len;
            i += seq_len;
        }
    }
    len
}

/// Encode into provided writer (zero-allocation path other than writer itself).
pub fn encode_into(input: &str, out: &mut impl fmt::Write) -> fmt::Result {
    let mut buf = [0u8; 4];
    for ch in input.chars() {
        if ch.is_ascii() {
            let b = ch as u8;
            if is_unreserved(b) {
                out.write_char(ch)?;
            } else {
                let hi = HEX_UPPER[(b >> 4) as usize] as char;
                let lo = HEX_UPPER[(b & 0x0F) as usize] as char;
                out.write_char(',')?;
                out.write_char(hi)?;
                out.write_char(lo)?;
            }
        } else {
            let s = ch.encode_utf8(&mut buf);
            out.write_str(s)?;
        }
    }
    Ok(())
}

#[cfg(feature = "alloc")]
pub fn encode(input: &str) -> alloc::string::String {
    let mut s = alloc::string::String::with_capacity(encoded_len(input));
    encode_into(input, &mut s).expect("write to String cannot fail");
    s
}

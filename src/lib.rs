#![cfg_attr(not(feature = "std"), no_std)]
//! OI4 / DIN SPEC 91406 Digital Nameplate (DNP) encoding / decoding / validation.
//!
//! Features:
//! - std (default): Enables std::error::Error integration; implies `alloc`.
//! - alloc: Provides heap-backed APIs (e.g. `encode` -> String, `decode`).
//! - strict: Enforces uppercase hex escapes and forbids unescaped reserved ASCII.
//!
//! Encoding rules summary:
//! * Unreserved characters (ALPHA / DIGIT / '-' / '.' / '_' / '~') stay literal.
//! * Every other ASCII byte (including the comma itself) MUST be represented as `,XX` (uppercase hex) when produced by the encoder.
//! * Non-ASCII Unicode stays verbatim (its UTF-8 bytes are not individually re-escaped), unless future spec revisions say otherwise.
//! * Decoder (default mode) accepts lowercase hex in escape triplets; encoder always outputs uppercase.
//! * Strict mode tightens validation (see feature `strict`).
//!
//! No panics on valid usage; no unsafe in production code.
//!
//! ```rust
//! use oi4_dnp_encoding::encode;
//! # #[cfg(feature="alloc")]
//! # {
//! let s = "Hello World!"; // space & exclamation must be escaped
//! let enc = encode(s);
//! assert_eq!(enc, "Hello,20World,21");
//! # }
//! ```

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod encode;
pub mod decode;
pub mod validate;
mod error;

pub use crate::encode::{encode_into, encoded_len};
#[cfg(feature = "alloc")]
pub use crate::encode::encode;
#[cfg(feature = "alloc")]
pub use crate::decode::decode;
pub use crate::validate::{validate_dnp, Rules};
pub use crate::error::{Error, ErrorKind};


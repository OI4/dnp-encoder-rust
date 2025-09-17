#![allow(clippy::derive_partial_eq_without_eq)]
use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    pos: Option<usize>,
}

impl Error {
    pub const fn new(kind: ErrorKind, pos: Option<usize>) -> Self {
        Self { kind, pos }
    }
    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }
    pub const fn position(&self) -> Option<usize> {
        self.pos
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    InvalidHexDigit(char),
    LoneComma,
    LowercaseHexInStrict,
    UnescapedReservedAscii(char),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHexDigit(c) => write!(f, "invalid hex digit '{c}'"),
            Self::LoneComma => write!(f, "lone comma without two hex digits"),
            Self::LowercaseHexInStrict => write!(f, "lowercase hex not allowed in strict mode"),
            Self::UnescapedReservedAscii(c) => write!(f, "unescaped reserved ASCII '{c}'"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = self.pos {
            write!(f, "{} at position {p}", self.kind)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

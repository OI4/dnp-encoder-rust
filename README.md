# oi4-dnp-encoding

Rust Implementation of OI4 / DIN SPEC 91406 Digital Nameplate (DNP) encoding, decoding and validation.

## Features

- Zero runtime dependencies
- `no_std` support (with optional `alloc`)
- Strict mode (feature `strict`) enforcing uppercase hex and prohibiting unescaped reserved ASCII
- High performance branch-lean encoder/decoder
- Safe Rust only; no panics on valid usage

## Encoding Summary

Unreserved characters: `A–Z a–z 0–9 - . _ ~` stay literal. All other ASCII bytes (including comma) are escaped as `,XX` (uppercase hex). Non-ASCII Unicode is passed through verbatim.

Examples:
```
Space (0x20) -> ,20
Comma (0x2C) -> ,2C
# -> ,23
/ -> ,2F
```

## Basic Usage (std / alloc)
```rust
use oi4_dnp_encoding::{encode, decode};

let original = "Hello World!";
let enc = encode(original);
assert_eq!(enc, "Hello,20World,21");
let dec = decode(&enc).unwrap();
assert_eq!(dec, original);
```

## Examples
Alle Beispiele liegen im Verzeichnis `examples/` und werden nicht in die Library kompiliert (kein Größen-Overhead für Nutzer).

Schnellstart:
```bash
# Einfaches Encode/Decode/Validate
cargo run --example basic

# Validierung & Fehlerbehandlung
cargo run --example validation

# Strict Mode Verhalten (Großbuchstaben-Hex + Pflicht Maskierung)
cargo run --example strict --features strict

# Heap-freies Encoding (demonstriert no_std Pfad)
cargo run --example encode_into_no_alloc
# oder wirklich ohne std:
cargo run --example encode_into_no_alloc --no-default-features
```

## Validation
```rust
use oi4_dnp_encoding::{validate_dnp, Rules};

let s = "Hello,20World,21"; // already encoded
validate_dnp(s, &Rules::default()).unwrap();
```

Enable strict feature:
```bash
cargo test --features strict
```

## Differential Test (Go Reference)
Set environment variable `GO_DNP_REPO` to a local clone of the Go reference repo (OI4/dnp-encoder-go) and run:
```bash
cargo test -- --nocapture differential_against_go
```
Or standalone script:
```bash
GO_DNP_REPO=/path/to/go/repo ./tools/compare_go.sh < inputs.txt
```
Fallback: Wenn `GO_DNP_REPO` nicht gesetzt ist, klont `tools/compare_go.sh` automatisch den Branch `development` aus
```
https://github.com/OI4/dnp-encoder-go.git
```
(shallow clone, depth=1) in ein temporäres Verzeichnis und räumt danach auf. Voraussetzungen für den Fallback: `git` und `go` im `PATH`.

## Golden File Generation
```bash
echo "Hello World!" | cargo run --bin gen_golden --features std,alloc
```

## Fuzzing
Requires nightly and libFuzzer (cargo-fuzz style not strictly necessary here). Example using libfuzzer-sys directly:
```bash
cd fuzz
cargo fuzz run decode # if integrated with cargo-fuzz (future)
```
Current discrete fuzz targets (manual): `fuzz/fuzz_targets/decode.rs`, `validate.rs`.

## Coverage
```bash
cargo install cargo-llvm-cov
cargo llvm-cov --all-features --html
```

## no_std
- Without `std`, errors do not implement `std::error::Error`.
- Without `alloc`, only `encode_into`, `encoded_len`, and `validate_dnp` are available.

## MSRV
Minimum Supported Rust Version: 1.74 (pinned via `rust-toolchain.toml`).

## License
MIT

## Status
Early implementation; align with authoritative PDF specification. Any deviations vs. Go reference will be documented in `DESIGN.md`.

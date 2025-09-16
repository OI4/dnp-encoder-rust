# OI4 DNP Encoding Library - Rust

OI4 / DIN SPEC 91406 Digital Nameplate (DNP) encoding/decoding/validation library in Rust. This is a high-performance, `no_std` compatible library with zero runtime dependencies.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Bootstrap, Build, and Test
- `cargo build --all-features --verbose` -- builds library with all features. NEVER CANCEL: Takes ~20 seconds. Set timeout to 60+ minutes.
- `cargo test --all --verbose` -- runs complete test suite. NEVER CANCEL: Takes ~21 seconds. Set timeout to 60+ minutes.
- `cargo clippy --all-features -- -D warnings` -- runs linter, must pass clean. Takes ~3 seconds.
- `cargo fmt -- --check` -- checks code formatting, run `cargo fmt` to fix any issues. Takes ~1 second.

### Feature Testing (Critical for CI)
- `cargo test --no-default-features --features alloc --verbose` -- tests alloc-only mode (no std). Takes ~1 second.
- `cargo test --features strict --verbose` -- tests strict mode (uppercase hex, unescaped reserved error). Takes ~2 seconds.
- `cargo build --no-default-features` -- builds minimal no_std, no_alloc version. Takes ~1 second, shows warnings about unused imports.

### Performance and Coverage
- `cargo bench --quiet` -- runs performance benchmarks. NEVER CANCEL: Takes ~66 seconds. Set timeout to 90+ minutes.
- Coverage requires installation: `cargo install cargo-llvm-cov --locked` -- NEVER CANCEL: Takes ~71 seconds first time.
- `cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info` -- generates coverage. Takes ~20 seconds.

### CLI Usage
- Build CLI: `cargo build --features std,alloc` (creates `target/debug/oi4-dnp-encoding-cli`)
- Encode: `echo "Hello World!" | cargo run --bin oi4-dnp-encoding-cli -- encode` → `Hello,20World,21`
- Decode: `echo "Hello,20World,21" | cargo run --bin oi4-dnp-encoding-cli -- decode` → `Hello World!`
- CLI only supports `encode` and `decode` commands, no `validate` command

## Validation

### Manual User Scenarios (CRITICAL)
Always test these complete workflows after making changes:
1. **Basic Encode/Decode**: Verify `"Hello World!"` → `"Hello,20World,21"` → `"Hello World!"`
2. **CLI Functionality**: Test both encode and decode via CLI with stdin
3. **Feature Combinations**: Build and test with different feature flags
4. **Example Execution**: Run all examples to ensure they work correctly

### Examples (Always Run These)
- `cargo run --example basic` -- demonstrates basic encode/decode
- `cargo run --example validation` -- shows validation with error handling
- `cargo run --example strict --features strict` -- strict mode behavior
- `cargo run --example encode_into_no_alloc --no-default-features` -- no_std usage

### Golden File Generator
- `echo "Test String!" | cargo run --bin gen_golden --features std,alloc` -- generates test cases

### Go Reference Comparison (Optional)
- Requires Go toolchain: `which go` must succeed
- `echo -e "Hello World!\nTest Input" | ./tools/compare_go.sh` -- compares with Go reference implementation
- Script auto-clones Go repo if `GO_DNP_REPO` not set

## Common Tasks

### Always Run Before Committing
1. `cargo fmt` -- fix formatting
2. `cargo clippy --all-features -- -D warnings` -- must pass clean
3. `cargo test --all --verbose` -- NEVER CANCEL: Takes ~21 seconds
4. `cargo test --no-default-features --features alloc --verbose` -- test minimal config
5. `cargo test --features strict --verbose` -- test strict mode

### CI Requirements (Match .github/workflows/ci.yml)
The CI runs on Ubuntu, macOS, and Windows with these exact steps:
- Format check: `cargo fmt -- --check`
- Clippy: `cargo clippy --all-features -- -D warnings`
- Default features: `cargo test --all --verbose`
- Alloc only: `cargo test --no-default-features --features alloc --verbose`  
- Strict mode: `cargo test --features strict --verbose`
- MSRV build: `cargo build --no-default-features --features alloc` (Rust 1.80.0)

### Quick Reference: Common Commands and Expected Outputs

#### Repository Structure
```
├── src/           # Library source
│   ├── lib.rs     # Main library entry point
│   ├── encode.rs  # Encoding functions
│   ├── decode.rs  # Decoding functions
│   ├── validate.rs # Validation functions
│   ├── error.rs   # Error types
│   ├── hex.rs     # Hex utilities
│   └── main.rs    # CLI binary
├── tests/         # Integration tests
├── examples/      # Usage examples
├── benches/       # Performance benchmarks
├── tools/         # Development tools
│   ├── compare_go.sh   # Go reference comparison
│   └── gen_golden.rs   # Golden file generator
└── fuzz/          # Fuzzing targets (requires nightly)
```

#### Feature Flags
- `std` (default): Enables `std::error::Error`, implies `alloc`
- `alloc`: Provides `encode()` → String, `decode()` functions
- `strict`: Enforces uppercase hex and forbids unescaped reserved ASCII
- No features: Only `encode_into`, `encoded_len`, `validate_dnp` available

#### Build Times (Add 50% buffer for timeouts)
- Full build: ~20 seconds → Use 60+ minute timeout
- Full test suite: ~21 seconds → Use 60+ minute timeout
- Benchmarks: ~66 seconds → Use 90+ minute timeout
- Coverage generation: ~20 seconds → Use 60+ minute timeout
- cargo-llvm-cov installation: ~71 seconds (first time) → Use 90+ minute timeout

#### Key Functions
- `encode(input: &str) -> String` -- requires `alloc` feature
- `decode(input: &str) -> Result<String, Error>` -- requires `alloc` feature
- `encode_into(input: &str, output: &mut [u8]) -> Result<usize, Error>` -- no_alloc compatible
- `encoded_len(input: &str) -> usize` -- calculate required buffer size
- `validate_dnp(input: &str, rules: &Rules) -> Result<(), Error>` -- validation only

## Critical Notes
- **NEVER CANCEL** long-running commands - builds and tests can take 20+ seconds, benchmarks 60+ seconds
- Always set timeouts of 60+ minutes for build/test commands and 90+ minutes for benchmarks
- The library is `#![no_std]` by default unless `std` feature is enabled
- Fuzzing requires nightly Rust (not covered in standard workflow)
- Some unused import warnings in no-default-features build are expected
- CLI binary requires `std` and `alloc` features
- Coverage workflow runs on every push/PR to main branch
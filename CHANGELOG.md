# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2025-09-16

### Added
- Initial implementation: encoding, decoding, validation
- no_std + alloc feature gating
- strict feature
- Basic tests & documentation scaffolding
- Differential Test tooling (tools/compare_go.sh, tests/differential_go.rs)
- Golden file generator (gen_golden)
- Fuzzing skeleton (fuzz targets: decode, validate)
- Coverage workflow (coverage.yml) and README sections for differential, fuzzing, coverage

[Unreleased]: https://github.com/OI4/dnp-encoder-rust/compare/2.14.0...HEAD

[0.1.0]: https://github.com/OI4/dnp-encoder-rust/releases/tag/1.0.0

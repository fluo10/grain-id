# Changelog

All notable changes to this workspace will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## 0.13.0 - 2026-03-28

### Changed

- Rename crate from `caretta-id` to `grain-id`
- Rename type `CarettaId` to `GrainId`
- Rename module `caretta_id` to `grain_id`
- Rename example `caretta-id-cli` to `grain-id-cli`
- Rename example `caretta-id-prost-example` to `grain-id-prost-example`
- Update proto submodule to `grain-id-proto`

## 0.12.1 - 2026-03-25

### Changed

- Bump `rusqlite` from 0.37.0 to 0.39.0
- Bump `jsonschema` from 0.37 to 0.45

### Fixed

- Update `rusqlite` implementation to be compatible with 0.39.0 API changes

## 0.12.0 - 2026-03-14

### Added

- `increment()` method to `CarettaId` for wrapping increment by 1, enabling collision-free ID generation without sleeping when two IDs are generated within the same decisecond.

### Changed

- Fix CI workflows
- Run `cargo fmt` and `cargo clippy`

### Removed

- (`caretta-id-cli`) Deprecated `generate` subcommand test

## 0.11.0 - 2026-03-11

### Changed

- Bump versions of dependencies

### Removed

- Deprecated old caretta id format
  - `CarettaIdS`
  - `CarettaIdD`
  - `CarettaIdT`
  - `CarettaIdQ`

## 0.10.0 - 2025-12-01

### Added

- Convertion from `core::time::Duration` and `std::time::SystemTime`
- `wrapping_sub` function for handling negative duration.
- (`caretta-id-cli`) Add `random` and `timestamp` subcommand.

### Deprecated

- (`caretta-id-cli`) `generate` subcommand.

## 0.9.1 - 2025-11-19

- Fix errors about `no_std` and `serde` feature 

## 0.9.0 - 2025-11-19

### Changed

- For non human readable format, update implemention of `Serialize`/`Deserialize` trait to serialize to ( and deserialize from ) `u64` if `is_human_readable` returns false.
- Update tests and documents about serde and bytes conversion.

### Removed

- Tests for deprecated triplet-based IDs

## 0.8.1 - 2025-11-18

### Added

- bytes conversion

## 0.8.0 - 2025-11-16

### Added

- New `CarettaId` format like `0123abc`

### Deprecated
- All old caretta id format
  - `CarettaIdS`
  - `CarettaIdD`
  - `CarettaIdT`
  - `CarettaIdQ`

## 0.7.0 - 2025-11-14

### Added

- `redb` feature

### Changed

- **Rename `mtid` to `caretta-id`**

## 0.6.0 - 2025-10-21

### Added

- `from_proto_lossy` method for lossy conversion between rust struct and protobuf definition.

### Changed

- Rename `from_int_lossy` to `from_uint_lossy`
- Move protobuf conversion implemention to each mtid's submodule.

## 0.5.0 - 2025-10-20

### Changed

- Split protobuf definitions to [submodule](https://github.com/fluo10/mtid-proto).
- Fix field name of protobuf difinitions from `id` to `value`

## 0.4.0 - 2025-10-19

### Changed

- Refactor internal module structure.
- Rename `prost` module to `proto`

## 0.3.0 - 2025-10-18

### Added

- Public `triplet` module and `alphabet` module for references.
- Fuzzing tests
- `arbitrary` feature.

### Changed

- (`mtid-cli`) Length options become required.
- Rewrite internal character decoding function.

## 0.2.1 - 2025-10-16

### Added
- Document on docs.rs now contains all features with labels.
- (`mtid-cli`) Licenses.

### Changed
- (`mtid-cli`) Bump `mtid` to `v0.2.1`.

### Removed
- Unnecessary files like `.gitignore`, `.vscode` and `.github` are removed from published package.

## 0.2.0 - 2025-10-15

### Added

- `rand` feature (as default feature)
  - Dependencies on `rand` crate is now optional.
- Optional feature flags:
  - `std` (Default feature): Enable `std` features.
  - `rand` (Default feature): Enable random MTID generation. 
- `no_std` support by disable default `std` feature

### Changed

- Bump version of `rand` crate to `0.9.2`
- `MTID::random()` functionsnow uses thread_rng by default, so no arguments are required.
- Dependencies on `rand` crate is now optional.
- `Error` type has been almost completely rewritten to support `no_std`.
- The functions for conversion with strings has been almost completely rewitten to support `no_std`.
- (`mtid-cli`) Bump `mtid` to `v0.2.0`.

### Removed
- Dependency on `thiserror` crate

## 0.1.0 - 2025-10-13

### Added

- Initial release with multi-length triplet ID support
- `Stid`: Single triplet ID (3 characters)
- `Dtid`: Double Triplet ID (7 characters with delimiter)
- `Ttid`: Triple Triplet ID (11 characters with delimiter)
- `Qtid`: Quadruple triplet ID (15 characters with delimiter)
- Support for random ID generation
- String parsing with ambiguous character handling
- Integer conversion (to/from)
- Optional feature flags:
  - `serde`: Serialization/deserialization support
  - `rusqlite`: SQLite database integration
  - `sea-orm`: SeaORM ORM integration
  - `prost`: Protocol Buffers support
- (`mitd-cli`) Initial release example cli tool with 3 subcommands
  - `generate`: generate random id
  - `encode`: encode integer to string
  - `decode`: to decode string to integer

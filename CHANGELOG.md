# Changelog
All notable changes to `nucleide` will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [0.1.0] - 2023-06-12
### Added
 - `daku` custom section parsing module
 - `name` custom section parsing module
 - `producers` custom section parsing module
 - `parse` utilities module
 - `wasm` module to extend `parse` utilities
 - No-std support
 - `Result` type alias

### Changed
 - Replace `CustomSection` struct with `Section` enum
 - Rename `Module::custom_sections()` to `Module::sections()`
 - Change return type of `Module::sections()` to iterator over
   `nucleide::Section` contained within a `Result`
 - Rename `Module::add_custom_section()` to `Module::set_section()`
 - Replace `Module::set_section()` parameters for `name` and `payload` with a
   single parameter for `section`
 - Change `Module::set_section()` to be fallible and return `None` on failure
 - Rename `Module::clear_custom_section()` to `Module::clear_section()`
 - Change return type of `Module::clear_section()` to `nucleide::Section`

### Removed
 - Implementation of `From<parity_wasm::Error>` for `nucleide::Error`
 - Implementation of `Error` (standard library) for `nucleide::Error` to support
   no-std

### Fixed
 - No longer exposes any public API types from `parity_wasm`

## [0.0.1] - 2022-06-19
### Added
 - `CustomSection`
 - `Error`
 - `Module`

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0
[Semantic Versioning]: https://github.com/AldaronLau/semver

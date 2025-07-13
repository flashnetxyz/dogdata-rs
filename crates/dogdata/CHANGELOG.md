# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2](https://github.com/flashnetxyz/dogdata-rs/compare/dogdata_v0.2.1...dogdata_v0.2.2) - 2025-07-13

### Added

- add axum feature flag and optional dependencies

### Fixed

- Removes trailing whitespace
- *(axum)* improve cross-platform compatibility and code organization

### Other

- update dependencies versions
- refactors dependencies versions
- update tower and tokio dependencies
- centralize dependency versions to workspace  - Add exact versions from dogdata crate to workspace dependencies - Update dogdata Cargo.toml to use workspace versions - Organize dependencies into logical sections (OpenTelemetry, Tracing, HTTP, Async, Serialization, Misc) - Ensure consistent version management across the workspace

## [0.2.1](https://github.com/flashnetxyz/dogdata-rs/compare/dogdata_v0.2.0...dogdata_v0.2.1) - 2025-07-12

### Other

- fmt
- correct changelog path

## [0.2.0](https://github.com/flashnetxyz/dogdata-rs/compare/dogdata_v0.1.2...dogdata_v0.2.0) - 2025-07-12

### Added

- clippy

### Other

- move dependencies to workspace
- pre-commit tooling

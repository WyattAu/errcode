# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [Unreleased]

### Added
- Merged `http-errors` into `errcode`: `ErrorCode::Unauthorized` /
  `ErrorCode::Forbidden` variants, `status_code()` alias for `status()`,
  `as_str()` canonical code strings, and the `HttpError` status-mapping
  trait (`status_code()` / `error_code()` / `public_message()`, implemented
  for `ErrorCode`). `http-errors` remains as a thin re-export shim.

## [1.0.0] - 2026-09-05

### Added

- API declared stable; semver contract enforced via cargo-semver-checks CI gate.
- Derive macros for error codes, HTTP status mapping, and RFC 7807 Problem
  Details.
- Optional integrations: `serde`, `axum`, `sqlx`, `anyhow`, `tracing`;
  `no_std`-compatible core.

## [0.1.0] - 2026-09-04

### Added
- Structured error handling — derive macros for error codes, HTTP status mapping, RFC 7807 Problem Details.

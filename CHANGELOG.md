# Changelog

All notable changes are documented here.


All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [1.1.0] - 2026-09-09

### Added
- Merged `http-errors` into `errcode`: `ErrorCode::Unauthorized` /
  `ErrorCode::Forbidden` variants, `status_code()` alias for `status()`,
  `as_str()` canonical code strings, and the `HttpError` status-mapping
  trait (`status_code()` / `error_code()` / `public_message()`, implemented
  for `ErrorCode`). `http-errors` remains as a thin re-export shim.
- `schemars` feature: `JsonSchema` derive for `ErrorCode` and `ProblemDetail`
  (schemars 1.x, JSON Schema draft 2020-12). Implies `serde_impl` + `std`.
- `utoipa` feature: `utoipa::ToSchema` derive for `ProblemDetail`
  (utoipa 5.x) for OpenAPI documentation. Implies `serde_impl` + `std`.

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

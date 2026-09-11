# Requirements — errcode

Numbered, testable requirements. Every requirement maps to at least one named
test or doc-comment contract; security-relevant items cite threat-model rows.

Scope: Structured error codes (`error-codes`) — `ErrorCode` enum, derive macro, RFC 7807 mapping

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-ER-001 | `ErrorCode` maps deterministically to HTTP status codes | MUST |
| REQ-ER-002 | Derive `ErrCode` generates `code()`/`status()` from the enum definition | MUST |
| REQ-ER-003 | serde/schema/OpenAPI integrations are opt-in features; core builds `no_std`-capable | MUST |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-ER-100 | Problem-detail rendering escapes messages into JSON properly (serde) | MUST |

## Observability & API hygiene

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-ER-900 | All fallible public APIs return typed errors; production `unwrap`/`expect` is denied or explicitly justified with an invariant comment | MUST |
| REQ-ER-901 | Public items carry doc comments with runnable examples where practical | SHOULD |

# error-codes

Structured error handling for Rust — derive macros for error codes, HTTP status mapping, and RFC 7807 Problem Details.

## Purpose

`errcode` provides a unified error type system that bridges Rust error handling with HTTP APIs. It gives you:

- **Machine-readable error codes** (`ErrorCode` enum) that map directly to HTTP status codes
- **RFC 7807 Problem Details** — the IETF standard for HTTP API error responses
- **Derive macro support** (`ErrCode` trait) for ergonomic error type definitions
- **Framework integration** with Axum, SQLx, and tracing out of the box

## Quick Start

```rust
use errcode::{ErrorCode, ProblemDetail};

let problem = ProblemDetail::new(ErrorCode::NotFound)
    .with_detail("User 42 does not exist")
    .with_instance("/users/42");

assert_eq!(problem.status, 404);
println!("{}", problem.to_json_pretty());
```

### Implementing `ErrCode` on your error type

```rust
use errcode::{ErrCode, ErrorCode, ProblemDetail};

#[derive(Debug)]
struct AppError {
    code: ErrorCode,
    message: String,
}

impl ErrCode for AppError {
    fn code(&self) -> ErrorCode {
        self.code
    }

    fn detail(&self) -> Option<&str> {
        Some(&self.message)
    }
}

// Convert to ProblemDetail
let err = AppError {
    code: ErrorCode::Validation,
    message: "email is required".into(),
};
let problem: ProblemDetail = err.problem();
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | yes | Standard library support |
| `axum` | yes | Axum integration (IntoResponse impl) |
| `serde_impl` | yes | Serialize/deserialize ProblemDetail |
| `sqlx` | no | SQLx error conversion |
| `anyhow` | no | Anyhow integration |
| `tracing` | no | Tracing instrumentation |

## Comparison with other crates

### vs `thiserror`

`thiserror` is excellent for deriving `std::error::Error` on custom types. `errcode` builds on top of `thiserror` and adds **HTTP-aware error classification**. Use `errcode` when your errors need to map to status codes and produce RFC 7807 responses.

### vs `anyhow`

`anyhow` is great for application-level error handling where you don't need structured error types. `errcode` is for **library and API code** where consumers need to inspect error categories and produce standardized responses.

### vs raw `thiserror` + manual mapping

Without `errcode`, you'd write match arms to convert errors to status codes. `errcode` encodes those mappings in the `ErrorCode` enum itself, so the mapping is always consistent.

## MSRV

Rust **1.85** (edition 2024).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.

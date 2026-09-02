#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Structured error handling for Rust.
//!
//! Provides derive macros for error codes, HTTP status mapping,
//! and RFC 7807 Problem Details support.
//!
//! # Quick Start
//!
//! ```rust
//! use errcode::{ErrorCode, ProblemDetail};
//!
//! let problem = ProblemDetail::new(ErrorCode::NotFound)
//!     .with_detail("User not found");
//!
//! assert_eq!(problem.status, 404);
//! ```

/// Error types, codes, and RFC 7807 Problem Details.
pub mod error;

#[cfg(feature = "serde_impl")]
pub use error::ProblemDetail;

pub use error::{ErrCode, ErrorCode};

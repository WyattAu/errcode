#![cfg_attr(not(feature = "std"), no_std)]
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
//! use error_codes::ErrorCode;
//!
//! assert_eq!(ErrorCode::NotFound.status(), 404);
//! ```

extern crate alloc;

/// Error types, codes, and RFC 7807 Problem Details.
pub mod error;

#[cfg(feature = "serde_impl")]
pub use error::ProblemDetail;

pub use error::{ErrCode, ErrorCode};

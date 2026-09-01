use std::fmt;

/// Error codes for structured error handling.
///
/// Each variant maps to an HTTP status code and provides
/// machine-readable error classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    /// 404 — Resource not found
    NotFound,
    /// 409 — Conflict with current state
    Conflict,
    /// 422 — Validation failure
    Validation,
    /// 401 / 403 — Authentication or authorization error
    Auth,
    /// 500 — Internal server error
    Internal,
    /// 429 — Rate limit exceeded
    RateLimited,
    /// 400 — Bad request
    BadRequest,
    /// 503 — Service unavailable
    Unavailable,
}

impl ErrorCode {
    /// Returns the HTTP status code for this error code.
    pub fn status(&self) -> u16 {
        match self {
            Self::NotFound => 404,
            Self::Conflict => 409,
            Self::Validation => 422,
            Self::Auth => 401,
            Self::Internal => 500,
            Self::RateLimited => 429,
            Self::BadRequest => 400,
            Self::Unavailable => 503,
        }
    }

    /// Returns the canonical reason phrase for this error code.
    pub fn reason(&self) -> &'static str {
        match self {
            Self::NotFound => "Not Found",
            Self::Conflict => "Conflict",
            Self::Validation => "Validation Error",
            Self::Auth => "Unauthorized",
            Self::Internal => "Internal Server Error",
            Self::RateLimited => "Too Many Requests",
            Self::BadRequest => "Bad Request",
            Self::Unavailable => "Service Unavailable",
        }
    }

    /// Returns the `type` URI string per RFC 7807.
    pub fn type_uri(&self) -> &'static str {
        match self {
            Self::NotFound => "https://httpstatuses.com/404",
            Self::Conflict => "https://httpstatuses.com/409",
            Self::Validation => "https://httpstatuses.com/422",
            Self::Auth => "https://httpstatuses.com/401",
            Self::Internal => "https://httpstatuses.com/500",
            Self::RateLimited => "https://httpstatuses.com/429",
            Self::BadRequest => "https://httpstatuses.com/400",
            Self::Unavailable => "https://httpstatuses.com/503",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.reason(), self.status())
    }
}

/// RFC 7807 Problem Details object.
///
/// Represents a structured error response as defined in
/// [RFC 7807](https://datatracker.ietf.org/doc/html/rfc7807).
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_impl",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ProblemDetail {
    /// The error type URI.
    #[cfg_attr(feature = "serde_impl", serde(rename = "type"))]
    pub type_uri: String,

    /// Human-readable summary.
    pub title: String,

    /// HTTP status code.
    pub status: u16,

    /// Optional detailed error message.
    pub detail: Option<String>,

    /// Optional URI reference for the specific occurrence.
    pub instance: Option<String>,
}

impl ProblemDetail {
    /// Creates a new `ProblemDetail` from an `ErrorCode`.
    pub fn new(code: ErrorCode) -> Self {
        Self {
            type_uri: code.type_uri().to_string(),
            title: code.reason().to_string(),
            status: code.status(),
            detail: None,
            instance: None,
        }
    }

    /// Sets the detail message.
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Sets the instance URI.
    pub fn with_instance(mut self, instance: impl Into<String>) -> Self {
        self.instance = Some(instance.into());
        self
    }

    /// Serializes to JSON (requires `serde_impl` feature).
    #[cfg(feature = "serde_impl")]
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("ProblemDetail should always serialize")
    }

    /// Serializes to pretty-printed JSON (requires `serde_impl` feature).
    #[cfg(feature = "serde_impl")]
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).expect("ProblemDetail should always serialize")
    }
}

impl fmt::Display for ProblemDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.status, self.title)?;
        if let Some(detail) = &self.detail {
            write!(f, ": {detail}")?;
        }
        Ok(())
    }
}

impl std::error::Error for ProblemDetail {}

/// Marker trait for types that can produce an [`ErrorCode`].
///
/// Implement this on your error types to enable automatic conversion
/// to [`ProblemDetail`] and HTTP status mapping.
pub trait ErrCode {
    /// Returns the error code for this error.
    fn code(&self) -> ErrorCode;

    /// Returns an optional detail message.
    fn detail(&self) -> Option<&str> {
        None
    }

    /// Converts this error into a [`ProblemDetail`].
    fn problem(&self) -> ProblemDetail {
        let mut problem = ProblemDetail::new(self.code());
        if let Some(detail) = self.detail() {
            problem = problem.with_detail(detail);
        }
        problem
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_code_status_mapping() {
        assert_eq!(ErrorCode::NotFound.status(), 404);
        assert_eq!(ErrorCode::Conflict.status(), 409);
        assert_eq!(ErrorCode::Validation.status(), 422);
        assert_eq!(ErrorCode::Auth.status(), 401);
        assert_eq!(ErrorCode::Internal.status(), 500);
    }

    #[test]
    fn problem_detail_construction() {
        let problem = ProblemDetail::new(ErrorCode::NotFound)
            .with_detail("User 42 does not exist")
            .with_instance("/users/42");

        assert_eq!(problem.status, 404);
        assert_eq!(problem.title, "Not Found");
        assert_eq!(problem.detail.as_deref(), Some("User 42 does not exist"));
        assert_eq!(problem.instance.as_deref(), Some("/users/42"));
    }

    #[test]
    fn problem_detail_display() {
        let problem = ProblemDetail::new(ErrorCode::Validation)
            .with_detail("name is required");
        assert_eq!(format!("{problem}"), "[422] Validation Error: name is required");
    }

    #[cfg(feature = "serde_impl")]
    #[test]
    fn problem_detail_json_serialization() {
        let problem = ProblemDetail::new(ErrorCode::Conflict)
            .with_detail("version mismatch");
        let json = problem.to_json();
        assert!(json.contains("\"status\":409"));
        assert!(json.contains("Conflict"));
        assert!(json.contains("version mismatch"));
    }

    // ---- Additional ErrorCode status/reason/type_uri coverage ----

    #[test]
    fn error_code_status_remaining_variants() {
        assert_eq!(ErrorCode::RateLimited.status(), 429);
        assert_eq!(ErrorCode::BadRequest.status(), 400);
        assert_eq!(ErrorCode::Unavailable.status(), 503);
    }

    #[test]
    fn error_code_reason_all_variants() {
        assert_eq!(ErrorCode::NotFound.reason(), "Not Found");
        assert_eq!(ErrorCode::Conflict.reason(), "Conflict");
        assert_eq!(ErrorCode::Validation.reason(), "Validation Error");
        assert_eq!(ErrorCode::Auth.reason(), "Unauthorized");
        assert_eq!(ErrorCode::Internal.reason(), "Internal Server Error");
        assert_eq!(ErrorCode::RateLimited.reason(), "Too Many Requests");
        assert_eq!(ErrorCode::BadRequest.reason(), "Bad Request");
        assert_eq!(ErrorCode::Unavailable.reason(), "Service Unavailable");
    }

    #[test]
    fn error_code_type_uri_all_variants() {
        assert_eq!(ErrorCode::NotFound.type_uri(), "https://httpstatuses.com/404");
        assert_eq!(ErrorCode::Conflict.type_uri(), "https://httpstatuses.com/409");
        assert_eq!(ErrorCode::Validation.type_uri(), "https://httpstatuses.com/422");
        assert_eq!(ErrorCode::Auth.type_uri(), "https://httpstatuses.com/401");
        assert_eq!(ErrorCode::Internal.type_uri(), "https://httpstatuses.com/500");
        assert_eq!(ErrorCode::RateLimited.type_uri(), "https://httpstatuses.com/429");
        assert_eq!(ErrorCode::BadRequest.type_uri(), "https://httpstatuses.com/400");
        assert_eq!(ErrorCode::Unavailable.type_uri(), "https://httpstatuses.com/503");
    }

    #[test]
    fn error_code_display_all_variants() {
        assert_eq!(format!("{}", ErrorCode::NotFound), "Not Found (404)");
        assert_eq!(format!("{}", ErrorCode::Conflict), "Conflict (409)");
        assert_eq!(format!("{}", ErrorCode::Validation), "Validation Error (422)");
        assert_eq!(format!("{}", ErrorCode::Auth), "Unauthorized (401)");
        assert_eq!(format!("{}", ErrorCode::Internal), "Internal Server Error (500)");
        assert_eq!(format!("{}", ErrorCode::RateLimited), "Too Many Requests (429)");
        assert_eq!(format!("{}", ErrorCode::BadRequest), "Bad Request (400)");
        assert_eq!(format!("{}", ErrorCode::Unavailable), "Service Unavailable (503)");
    }

    #[test]
    fn error_code_debug_clone_copy() {
        let code = ErrorCode::NotFound;
        let cloned = code;
        let copied = code;
        assert_eq!(code, cloned);
        assert_eq!(code, copied);
        let debug_str = format!("{:?}", code);
        assert_eq!(debug_str, "NotFound");
    }

    #[test]
    fn error_code_hash_consistency() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(ErrorCode::NotFound, "not_found");
        map.insert(ErrorCode::Conflict, "conflict");
        assert_eq!(map.get(&ErrorCode::NotFound), Some(&"not_found"));
        assert_eq!(map.get(&ErrorCode::Conflict), Some(&"conflict"));
        assert_eq!(map.get(&ErrorCode::Auth), None);
    }

    // ---- Additional ProblemDetail tests ----

    #[test]
    fn problem_detail_display_without_detail() {
        let problem = ProblemDetail::new(ErrorCode::Internal);
        assert_eq!(format!("{problem}"), "[500] Internal Server Error");
    }

    #[test]
    fn problem_detail_display_only_instance() {
        let problem = ProblemDetail::new(ErrorCode::NotFound)
            .with_instance("/users/42");
        assert_eq!(format!("{problem}"), "[404] Not Found");
    }

    #[test]
    fn problem_detail_defaults_from_code() {
        let problem = ProblemDetail::new(ErrorCode::Auth);
        assert_eq!(problem.type_uri, "https://httpstatuses.com/401");
        assert_eq!(problem.title, "Unauthorized");
        assert_eq!(problem.status, 401);
        assert!(problem.detail.is_none());
        assert!(problem.instance.is_none());
    }

    #[test]
    fn problem_detail_implements_std_error() {
        let problem = ProblemDetail::new(ErrorCode::NotFound)
            .with_detail("gone");
        let err: &dyn std::error::Error = &problem;
        assert!(err.source().is_none());
        assert!(err.to_string().contains("Not Found"));
    }

    #[test]
    fn problem_detail_clone() {
        let p1 = ProblemDetail::new(ErrorCode::Conflict)
            .with_detail("race condition")
            .with_instance("/resource/1");
        let p2 = p1.clone();
        assert_eq!(p1.status, p2.status);
        assert_eq!(p1.title, p2.title);
        assert_eq!(p1.detail, p2.detail);
        assert_eq!(p1.instance, p2.instance);
    }

    #[cfg(feature = "serde_impl")]
    #[test]
    fn problem_detail_json_pretty_serialization() {
        let problem = ProblemDetail::new(ErrorCode::Validation)
            .with_detail("field required")
            .with_instance("/form");
        let json = problem.to_json_pretty();
        assert!(json.contains("\"type\""));
        assert!(json.contains("Validation Error"));
        assert!(json.contains("field required"));
        assert!(json.contains("/form"));
        assert!(json.contains('\n'));
    }

    #[cfg(feature = "serde_impl")]
    #[test]
    fn problem_detail_json_roundtrip() {
        let original = ProblemDetail::new(ErrorCode::NotFound)
            .with_detail("item missing")
            .with_instance("/items/7");
        let json = original.to_json();
        let restored: ProblemDetail = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.status, original.status);
        assert_eq!(restored.title, original.title);
        assert_eq!(restored.detail, original.detail);
        assert_eq!(restored.instance, original.instance);
        assert_eq!(restored.type_uri, original.type_uri);
    }

    // ---- ErrCode trait tests ----

    #[derive(Debug)]
    struct MyError {
        msg: String,
    }

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.msg)
        }
    }

    impl ErrCode for MyError {
        fn code(&self) -> ErrorCode {
            if self.msg.contains("not found") {
                ErrorCode::NotFound
            } else {
                ErrorCode::Internal
            }
        }

        fn detail(&self) -> Option<&str> {
            Some(&self.msg)
        }
    }

    #[test]
    fn err_code_trait_problem_with_detail() {
        let err = MyError {
            msg: "user not found".into(),
        };
        let problem = err.problem();
        assert_eq!(problem.status, 404);
        assert_eq!(problem.title, "Not Found");
        assert_eq!(problem.detail.as_deref(), Some("user not found"));
    }

    #[test]
    fn err_code_trait_code_selection() {
        let not_found = MyError { msg: "not found".into() };
        assert_eq!(not_found.code(), ErrorCode::NotFound);

        let other = MyError { msg: "something".into() };
        assert_eq!(other.code(), ErrorCode::Internal);
    }

    #[derive(Debug)]
    struct SimpleError;

    impl std::fmt::Display for SimpleError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "simple error")
        }
    }

    impl ErrCode for SimpleError {
        fn code(&self) -> ErrorCode {
            ErrorCode::BadRequest
        }
    }

    #[test]
    fn err_code_trait_default_detail_none() {
        let err = SimpleError;
        assert!(err.detail().is_none());
        let problem = err.problem();
        assert_eq!(problem.status, 400);
        assert!(problem.detail.is_none());
    }
}

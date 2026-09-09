// Property tests assert invariants directly; unwraps keep failures loud.
#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Property-based tests for error-codes crate.

extern crate alloc;

use proptest::prelude::*;

use error_codes::ErrorCode;

/// Generate arbitrary ErrorCode variants.
fn arb_error_code() -> impl Strategy<Value = ErrorCode> {
    prop_oneof![
        Just(ErrorCode::NotFound),
        Just(ErrorCode::Conflict),
        Just(ErrorCode::Validation),
        Just(ErrorCode::Auth),
        Just(ErrorCode::Unauthorized),
        Just(ErrorCode::Forbidden),
        Just(ErrorCode::Internal),
        Just(ErrorCode::RateLimited),
        Just(ErrorCode::BadRequest),
        Just(ErrorCode::Unavailable),
    ]
}

proptest! {
    #[test]
    fn status_code_always_valid_http_range(code in arb_error_code()) {
        let status = code.status();
        prop_assert!((100..600).contains(&status),
            "status {} not in HTTP range 100-599", status);
    }

    #[test]
    fn reason_always_non_empty(code in arb_error_code()) {
        let reason = code.reason();
        prop_assert!(!reason.is_empty(), "reason must not be empty");
    }

    #[test]
    fn type_uri_always_starts_with_http(code in arb_error_code()) {
        let uri = code.type_uri();
        prop_assert!(uri.starts_with("https://"),
            "type_uri '{}' must start with https://", uri);
    }

    #[test]
    fn display_always_contains_status(code in arb_error_code()) {
        let display = alloc::format!("{}", code);
        let status_str = code.status().to_string();
        prop_assert!(display.contains(&status_str),
            "display '{}' must contain status {}", display, status_str);
    }

    #[test]
    fn status_code_alias_matches_status(code in arb_error_code()) {
        prop_assert_eq!(code.status_code(), code.status());
    }

    #[test]
    fn as_str_is_uppercase_identifier(code in arb_error_code()) {
        let s = code.as_str();
        prop_assert!(!s.is_empty(), "as_str must not be empty");
        prop_assert!(s.chars().all(|c| c == '_' || c.is_ascii_uppercase()),
            "as_str '{}' must be UPPER_SNAKE", s);
    }

    #[test]
    fn http_error_impl_matches_inherent_mapping(code in arb_error_code()) {
        use error_codes::HttpError;
        prop_assert_eq!(<ErrorCode as HttpError>::status_code(&code), code.status());
        prop_assert_eq!(<ErrorCode as HttpError>::error_code(&code), code.as_str());
        prop_assert_eq!(<ErrorCode as HttpError>::public_message(&code), code.reason());
    }
}

#[cfg(feature = "serde_impl")]
mod problem_detail_tests {
    use error_codes::ProblemDetail;
    use proptest::prelude::*;

    use super::arb_error_code;

    proptest! {
        #[test]
        fn problem_detail_new_matches_code(code in arb_error_code()) {
            let problem = ProblemDetail::new(code);
            prop_assert_eq!(problem.status, code.status());
            prop_assert_eq!(problem.title, code.reason());
            prop_assert_eq!(problem.type_uri, code.type_uri());
            prop_assert!(problem.detail.is_none());
            prop_assert!(problem.instance.is_none());
        }

        #[test]
        fn problem_detail_with_detail_preserves_fields(
            code in arb_error_code(),
            detail in ".*{0,1000}",
        ) {
            let problem = ProblemDetail::new(code).with_detail(&detail);
            prop_assert_eq!(problem.status, code.status());
            prop_assert_eq!(problem.detail.as_deref(), Some(detail.as_str()));
        }

        #[test]
        fn problem_detail_with_instance_preserves_fields(
            code in arb_error_code(),
            instance in "/[a-z]{1,50}",
        ) {
            let problem = ProblemDetail::new(code).with_instance(&instance);
            prop_assert_eq!(problem.status, code.status());
            prop_assert_eq!(problem.instance.as_deref(), Some(instance.as_str()));
        }

        #[test]
        fn problem_detail_display_always_starts_with_status(
            code in arb_error_code(),
            detail in ".*{0,200}",
        ) {
            let problem = ProblemDetail::new(code).with_detail(&detail);
            let display = alloc::format!("{problem}");
            let expected_prefix = alloc::format!("[{}]", code.status());
            prop_assert!(display.starts_with(&expected_prefix),
                "display '{}' must start with '{}'", display, expected_prefix);
        }

        #[test]
        fn problem_detail_json_roundtrip(
            code in arb_error_code(),
            detail in ".*{0,500}",
            instance in "/[a-z0-9/]{1,100}",
        ) {
            let original = ProblemDetail::new(code)
                .with_detail(&detail)
                .with_instance(&instance);
            let json = original.to_json();
            let restored: ProblemDetail = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(restored.status, original.status);
            prop_assert_eq!(restored.title, original.title);
            prop_assert_eq!(restored.detail, original.detail);
            prop_assert_eq!(restored.instance, original.instance);
        }
    }
}

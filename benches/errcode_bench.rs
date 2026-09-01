use criterion::{criterion_group, criterion_main, Criterion};
use error_codes::{ErrorCode, ProblemDetail};

fn bench_error_code_status(c: &mut Criterion) {
    c.bench_function("error_code_status", |b| {
        b.iter(|| {
            let _ = ErrorCode::NotFound.status();
            let _ = ErrorCode::Conflict.status();
            let _ = ErrorCode::Validation.status();
            let _ = ErrorCode::Auth.status();
            let _ = ErrorCode::Internal.status();
            let _ = ErrorCode::RateLimited.status();
            let _ = ErrorCode::BadRequest.status();
            let _ = ErrorCode::Unavailable.status();
        });
    });
}

fn bench_error_code_reason(c: &mut Criterion) {
    c.bench_function("error_code_reason", |b| {
        b.iter(|| {
            let _ = ErrorCode::NotFound.reason();
            let _ = ErrorCode::Conflict.reason();
        });
    });
}

fn bench_error_code_type_uri(c: &mut Criterion) {
    c.bench_function("error_code_type_uri", |b| {
        b.iter(|| {
            let _ = ErrorCode::NotFound.type_uri();
            let _ = ErrorCode::Internal.type_uri();
        });
    });
}

fn bench_problem_detail_creation(c: &mut Criterion) {
    c.bench_function("problem_detail_creation", |b| {
        b.iter(|| ProblemDetail::new(ErrorCode::NotFound));
    });
}

fn bench_problem_detail_with_detail(c: &mut Criterion) {
    c.bench_function("problem_detail_with_detail", |b| {
        b.iter(|| {
            ProblemDetail::new(ErrorCode::NotFound)
                .with_detail("User not found")
        });
    });
}

fn bench_problem_detail_with_instance(c: &mut Criterion) {
    c.bench_function("problem_detail_with_instance", |b| {
        b.iter(|| {
            ProblemDetail::new(ErrorCode::NotFound)
                .with_detail("User not found")
                .with_instance("/users/42")
        });
    });
}

fn bench_problem_detail_to_json(c: &mut Criterion) {
    let problem = ProblemDetail::new(ErrorCode::NotFound)
        .with_detail("User not found")
        .with_instance("/users/42");
    c.bench_function("problem_detail_to_json", |b| {
        b.iter(|| problem.to_json());
    });
}

fn bench_problem_detail_to_json_pretty(c: &mut Criterion) {
    let problem = ProblemDetail::new(ErrorCode::NotFound)
        .with_detail("User not found")
        .with_instance("/users/42");
    c.bench_function("problem_detail_to_json_pretty", |b| {
        b.iter(|| problem.to_json_pretty());
    });
}

fn bench_problem_detail_display(c: &mut Criterion) {
    let problem = ProblemDetail::new(ErrorCode::Validation)
        .with_detail("name is required");
    c.bench_function("problem_detail_display", |b| {
        b.iter(|| format!("{problem}"));
    });
}

criterion_group!(
    benches,
    bench_error_code_status,
    bench_error_code_reason,
    bench_error_code_type_uri,
    bench_problem_detail_creation,
    bench_problem_detail_with_detail,
    bench_problem_detail_with_instance,
    bench_problem_detail_to_json,
    bench_problem_detail_to_json_pretty,
    bench_problem_detail_display,
);
criterion_main!(benches);

# PATTERN_ADOPTION

errcode (package `error-codes`) declares its relationship to the
WyattAu pattern canon (`WyattAu/pattern-standards`). Validation:
`scripts/adoption-check.py` in the canon repo; format:
`references/adoption-format.md`.

| Pattern | Status | Evidence |
|---|---|---|
| P72 | adopted | src/error.rs:14 (ErrorCode enum, one code per failure class) + src/error.rs:40 (code to HTTP status mapping) + src/error.rs:127 (RFC 7807 ProblemDetail envelope) + src/error.rs:203 (ErrCode trait; problem() conversion at src/error.rs:214) |

Scope note: this crate is the HTTP layer of the estate error stack
(app-error is the application layer; http-errors is a deprecated shim
over this crate). Patterns outside the row above do not apply to a
leaf status-mapping library.

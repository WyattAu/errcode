# Threat Model — errcode

Reference: STRIDE. Scope: the crate's public API surface. Trust boundary:
(1) bytes/inputs entering public constructors and parsers, (2) concurrent
callers sharing interior state. errcode is an in-process library — it opens
no sockets and inherits the embedding process's trust domain.

Purpose: Structured error codes (`error-codes`) — `ErrorCode` enum, derive macro, RFC 7807 mapping

## Assets

| ID | Asset | Exposed via |
|----|-------|-------------|
| A1 | stable error-code contract across services | hostile input, concurrent callers |

## STRIDE Analysis

| # | Threat | Category | Surface | Mitigation | Residual risk |
|---|--------|----------|---------|------------|---------------|
| T1 | Code/status mismatch breaks API contract | Tampering | `mapping table` | single mapping function, exhaustively tested | documented |
| T2 | Internal details leak via problem messages | Info disclosure | `RFC 7807 rendering` | message text is caller-provided; no automatic backtrace/internal context inclusion | documented |

## Repudiation

The crate keeps no audit trail; attribution of calls to callers is out of
scope for an in-process library.

## Out of Scope

- Network transport security (the crate never opens sockets).
- Storage-host compromise: an attacker who controls the host can bypass all
  in-process mitigations.
- Denial of service via resource exhaustion of the host process beyond the
  bounds enforced above.

Reviewed: 2026-09-11

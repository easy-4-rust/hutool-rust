# Security model

Hutool-Rust treats network, cryptographic, archive, script, and parser boundaries as
hostile-input surfaces.

## Defaults

- HTTP uses Rustls, connect/total timeouts, redirect limits and bounded bodies.
- HTTP retries are opt-in and reject non-idempotent methods and non-cloneable
  request bodies; retryable status/transport failures use capped jittered delay.
- `DenyLocalTargets` rejects private and local literal IP targets. Applications
  that accept arbitrary hostnames should install a resolver-aware `UrlPolicy`
  to defend against DNS rebinding.
- AES uses authenticated AES-256-GCM with a fresh random nonce.
- Passwords use Argon2id with a fresh random salt and PHC output.
- JWT validation pins HS256 and requires explicit issuer, audience, clock skew
  and expiration policy.
- ZIP extraction rejects traversal paths and symbolic links and enforces entry
  and uncompressed-byte limits.
- Image operations bound encoded input, decoded dimensions and pixels, target
  dimensions, and encoded output before returning an allocation.
- POI/Office handling is not implemented or exposed by the facade. The
  placeholder `hutool-poi` crate was removed on 2026-08-04 (capability moved to the standalone `easyexcel-rust` project); no XLSX/DOCX/OFD/PDF surface exists in this workspace
  guarantees and must not be used for untrusted documents.
- SMTP credentials use redacted secret containers. TLS and mandatory STARTTLS
  are explicit modes; unencrypted SMTP requires an explicit `Plaintext` choice.
- Rhai scripts have operation, call-depth, expression-depth, string, array and
  map limits. Dynamic `eval` is disabled; no filesystem/network/process API is
  registered by default.
- OpenAI-compatible streaming parses SSE incrementally and rejects oversized
  events instead of buffering an unbounded provider frame.
- Fallible cron jobs use an explicit bounded retry count, capped exponential
  delay, optional execution timeout, and a tracing span for exhausted runs.
- `hutool-observability` defaults to tracing, metrics, and health only. CPU,
  Tokio Console, and heap profiling require separate Cargo features and a
  matching unforgeable runtime permit.
- Tokio Console is restricted to loopback. The observability crate starts no
  management HTTP server, hidden runtime, or global allocator.
- The workspace denies Rust `unsafe` code.

## Caller responsibilities

- Keep HTTP/DNS policy aligned with the deployment network.
- Apply authentication, authorization, rate limiting and request-size limits at
  service boundaries.
- Store CAPTCHA answers server-side with single-use semantics and rate limits.
- Treat injected audio CAPTCHA speech engines as trusted dependencies and keep
  their own model, process, and network limits outside the renderer boundary.
- Sanitize untrusted HTML mail content and validate application-specific
  recipient policy before sending.
- Keep database transaction boundaries explicit and protect connection URLs.
- Choose cron retry and timeout values that fit the downstream service budget;
  a job handle intentionally serializes scheduled runs to prevent overlap.
- Wrap secrets in `secrecy` or `hutool_log::Redacted`; never attach them to
  tracing fields directly.
- Authenticate diagnostic management routes, enforce bounded profiling
  durations and response sizes, and never put diagnostic tokens in URLs,
  logs, spans, or health details.
- Choose archive and document limits below the host's actual resource budget.

## Legacy algorithms

MD5, SHA-1, ECB encryption and unauthenticated encryption are not exposed by
the default `crypto` feature. The `crypto-legacy` feature is reserved for
explicit migration helpers and currently adds no algorithms.

Report vulnerabilities according to [`SECURITY.md`](../SECURITY.md).

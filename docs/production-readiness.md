# Production readiness audit

This ledger checks the implementation against the original Hutool-Rust architecture
requirements. It describes the current `0.1` line and does not claim
method-for-method Hutool compatibility.

## Architecture and API

| Requirement | Status | Evidence or boundary |
|---|---|---|
| Workspace structure | complete | 26 workspace crates: 20 implemented capability crates, facade, compatibility, macro, test support, and Rust extensions (`hutool-poi` placeholder removed 2026-08-04) |
| Minimal facade defaults | complete | `hutool` defaults to `core` and `json`; `full` is explicit |
| Additive feature model | complete | every facade and component feature is checked independently in CI |
| No reverse/cyclic component dependency | complete | facade-only aggregation; components never depend on `hutool` |
| Idiomatic and compatibility APIs separated | complete | `hutool-compat-hutool` delegates to core/JSON implementations |
| No hidden global client, pool, config, or runtime | complete | stateful resources are constructed and injected explicitly |
| Complete Hutool functional parity | ledger only; not implementation parity | pinned v5.8.46 APIs are registered in the parity ledger, but registration includes planned and unportable entries. `hutool-poi` was explicitly excluded and its placeholder crate was removed on 2026-08-04; Excel/Word/OFD capability is provided by the standalone `easyexcel-rust` project. |

## Runtime boundaries

| Requirement | Status | Evidence or boundary |
|---|---|---|
| HTTP timeouts, redirects, body limits, streaming, SSRF hook | complete | `hutool-http` bounded Rustls client and `UrlPolicy` |
| Idempotent-only HTTP retry | complete | explicit capped jitter/backoff and `Retry-After` handling |
| Explicit SQLx pools and transactions | complete | driver features plus real PostgreSQL/MySQL/SQLite transaction tests |
| Cron external runtime, shutdown, timeout, non-overlap, tracing | complete | `spawn_on`, `JobHandle`, `JobPolicy`, per-run spans |
| Cron retry independent from job implementation | complete | `RetryPolicy` and fallible spawn APIs with bounded exponential delay |
| Bounded network/parser/media inputs | complete for current exposed APIs | HTTP, SSE, sockets, ZIP, image, mail, CAPTCHA, Rhai; POI/Office is not implemented or exposed |
| Bounded binary serialization | complete when enabled | 16 MiB default payload limit, exact length, schema/version, codec, flags, trailing-byte and CRC32 validation |
| Default observability | complete | `hutool-observability` defaults to reloadable tracing, explicit Prometheus recorder installation, and application-owned health registry |
| Diagnostic backends | feature and authorization gated | pprof, tokio-console, and DHAT each require a non-default Cargo feature plus a matching runtime permit; console binds only to loopback |
| External profilers | external only | samply, perf, eBPF/bpftrace, gdb, and lldb are documented tools and never Cargo dependencies |

## Security and quality

| Requirement | Status | Evidence or boundary |
|---|---|---|
| No workspace unsafe code | complete | every crate forbids unsafe; CI builds all features |
| Secure crypto defaults | complete | AES-256-GCM, HMAC-SHA-256, Argon2id, scoped HS256 JWT, and RustCrypto-backed HS/RS/ES JOSE signers |
| Legacy crypto isolated | complete | Hutool-named MD5/SHA1/DSA/SM4-CMAC and unavailable ES512 JWT entry points return typed rejection errors instead of enabling unsafe compatibility |
| Supply-chain policy | complete | `cargo-deny` advisories, licenses, bans, and sources checks |
| Default/no-default/all-feature tests | complete | CI quality job and local release gate |
| Per-feature compilation | complete | `cargo hack --each-feature --locked` |
| Stable, MSRV, Nightly, GNU/MUSL, macOS, Windows | complete | CI matrix; MSRV is Rust 1.94 |
| Property, compile-fail, fuzz and integration testing | complete for current parsers | codec properties; compile-fail docs; structured parser fuzz targets; real HTTP/database tests |
| 100% test coverage | in progress | current all-feature baseline: lines 98.73% (29,302/29,680), regions 98.71% (48,418/49,051), functions 98.30% (4,048/4,118); the facade registry, JWT, cache, system, cron, setting, AI, log, JSON, and HTTP crates are each exactly 100%, `hutool-core/src/builder.rs`, `mutable.rs`, `boolean_util.rs`, `byte_util.rs`, `char_util.rs`, `charset_util.rs`, `coordinate_util.rs`, `credit_code_util.rs`, `desensitized_util.rs`, `hash_util.rs`, `hex_util.rs`, `idcard_util.rs`, `page_util.rs`, `phone_util.rs`, `radix_util.rs`, and `version_util.rs` are each exactly 100%, with the new identity-card module contributing 566 covered lines, 1,040 covered regions, and 59 covered functions; CI still requires 100% workspace-wide |
| SemVer regression check | ready after first release | tag workflow runs `cargo-semver-checks`; no published baseline exists yet |
| Serialization performance baseline | ready, results environment-specific | `hutool-core/benches/serialization.rs` compares serde_json, bincode, postcard and all Müsli formats, including reusable-buffer wire encoding; no engine is declared universally fastest |

## Optional binary serialization gate

- Müsli is exactly pinned to `0.0.149`, whose declared MSRV does not exceed
  Hutool-Rust's Rust 1.94 baseline.
- Müsli and every concrete format are additive, non-default Cargo features.
- `full` intentionally does not activate a format or silently choose a wire
  compatibility contract.
- Wire compatibility is tested in both model directions with explicit field
  identifiers and defaults.
- Storage, packed, and descriptive have separate facade and codec identifiers;
  framed decoders reject cross-format input.
- Production adoption requires recording Criterion results on representative
  application models. A format becomes recommended only after it improves the
  target metric without regressing compatibility, MSRV, binary size, or memory.

## Gates before a public 1.0

- Reserve and publish the chosen crates.io names in dependency order.
- Complete an independent security and license/provenance review.
- Define the exact 1.0 Hutool capability scope and freeze those public APIs.
- Establish a released SemVer baseline and supported-version policy.
- Run long-duration fuzz campaigns and record their corpus, duration, and
  toolchain; CI currently verifies that every fuzz target builds.
- Add performance baselines for hotspots demonstrated by production-shaped
  benchmarks rather than for every utility function.

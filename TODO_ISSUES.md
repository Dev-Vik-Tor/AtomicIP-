# API Server Issues Implementation TODO

## Execution Order
1. #260 Logging & Metrics (foundation, no deps)
2. #259 Rate Limiting (can be independent)
3. #261 Auth & Authorization (required before webhooks)
4. #262 Webhook Support (last, leverages auth)

---

## Issue #260 — Logging & Metrics (`blackboxai/issue-260-logging-metrics`)
- [x] Create branch
- [x] Add dependencies to `api-server/Cargo.toml`
- [x] Create `api-server/src/metrics.rs` (Prometheus + structured logging)
- [x] Modify `api-server/src/main.rs` (init tracing, /metrics route, TraceLayer)
- [x] Modify `api-server/src/handlers.rs` (tracing::instrument)
- [x] Commit and push

## Issue #259 — Rate Limiting (`blackboxai/issue-259-rate-limiting`)
- [x] Create branch
- [x] Add dependencies to `api-server/Cargo.toml`
- [x] Create `api-server/src/rate_limit.rs`
- [x] Modify `api-server/src/main.rs` (mount RateLimitLayer)
- [x] Commit and push

## Issue #261 — Auth & Authorization (`blackboxai/issue-261-auth`)
- [x] Create branch
- [x] Add dependencies to `api-server/Cargo.toml`
- [x] Create `api-server/src/auth.rs`
- [x] Modify `api-server/src/main.rs` (auth routes + middleware)
- [x] Modify `api-server/src/handlers.rs` (login/refresh)
- [x] Modify `api-server/src/schemas.rs` (auth schemas)
- [x] Commit and push

## Issue #262 — Webhook Support (`blackboxai/issue-262-webhooks`)
- [x] Create branch
- [x] Add dependencies to `api-server/Cargo.toml`
- [x] Create `api-server/src/webhook.rs`
- [x] Modify `api-server/src/main.rs` (webhook routes)
- [x] Modify `api-server/src/handlers.rs` (register/unregister + triggers)
- [x] Modify `api-server/src/schemas.rs` (webhook schemas)
- [x] Commit and push

---

**Status:** COMPLETE — All 4 issues implemented, committed, and pushed to origin.


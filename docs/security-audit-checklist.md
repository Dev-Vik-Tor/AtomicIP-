# Security Audit Checklist

This checklist must be completed before every major release and after any change to contract logic.
See also: [Threat Model](threat-model.md) · [Security Policy](../SECURITY.md)

---

## 1. Authentication & Authorization

- [ ] Every state-mutating entry point calls `require_auth()` on the appropriate signer
- [ ] Admin-only functions (`pause`, `resolve_dispute`, `upgrade`) verify the caller against `DataKey::Admin`
- [ ] `initiate_swap`: seller auth required; ownership verified against IP registry
- [ ] `reveal_key`: only the seller of the specific swap may call it
- [ ] `cancel_swap`: only seller or buyer of the specific swap may call it
- [ ] `cancel_expired_swap`: only the buyer may call it, and only after expiry
- [ ] No function accepts an arbitrary `Address` argument as a privilege escalation path

## 2. Integer Overflow & Arithmetic

- [ ] All fee calculations use checked arithmetic; `fee_bps` is validated `0 ≤ fee_bps ≤ 10_000`
- [ ] `price` is validated `> 0` before any arithmetic
- [ ] `seller_amount = price - fee` cannot underflow (fee ≤ price enforced)
- [ ] Ledger timestamp comparisons use `u64`; no silent truncation from `u32`
- [ ] `NextId` counter increment cannot overflow `u64` in practice (document expected max IDs)

## 3. Reentrancy

- [ ] All storage writes and state transitions complete **before** any cross-contract token transfer
- [ ] `reveal_key`: swap status set to `Completed` before `token::transfer` is called
- [ ] `cancel_expired_swap` / `cancel_swap`: status set to `Cancelled` before refund transfer
- [ ] No callback or hook mechanism exists that could re-enter the contract mid-execution

## 4. Access Control on Upgrade

- [ ] `upgrade` checks `DataKey::Admin` and calls `require_auth()` on admin
- [ ] `ContractError::UnauthorizedUpgrade` (code 15) is returned for non-admin callers
- [ ] Upgrade path is documented and tested

## 5. Storage Expiry & TTL Management

- [ ] Every `env.storage().persistent().set(...)` is followed by a `bump` to `LEDGER_BUMP` (6_307_200)
- [ ] Every `env.storage().persistent().get(...)` also bumps TTL to prevent mid-operation expiry
- [ ] `DataKey::ActiveSwap(ip_id)` is cleared (not just overwritten) when a swap reaches `Completed` or `Cancelled`
- [ ] `DataKey::SwapApprovals` and `DataKey::SwapHistory` entries are bumped alongside their parent swap
- [ ] No storage key can expire while a swap is in `Pending` or `Accepted` state

## 6. Event Emission

- [ ] `SwapInitiatedEvent` emitted on every successful `initiate_swap`
- [ ] `SwapAcceptedEvent` emitted on every successful `accept_swap`
- [ ] `KeyRevealedEvent` emitted on every successful `reveal_key` (includes fee breakdown)
- [ ] `ProtocolFeeEvent` emitted whenever a non-zero fee is collected
- [ ] `SwapCancelledEvent` emitted on every cancellation path (seller cancel, buyer cancel, expired)
- [ ] `DisputeRaisedEvent` / `DisputeResolvedEvent` emitted for dispute lifecycle
- [ ] `SwapExpiryExtendedEvent` emitted when expiry is extended
- [ ] No event is emitted before the state transition is committed (no phantom events on panic)

## 7. Error Codes & Panic Safety

- [ ] All `ContractError` variants have stable, non-overlapping `u32` codes (1–28 as of v1.0)
- [ ] No `unwrap()` or `expect()` on `Option`/`Result` in production paths — use `ok_or_err` or explicit panics with meaningful errors
- [ ] Host function panics (e.g., out-of-bounds `BytesN` access) are not reachable via normal inputs
- [ ] Fuzzing / property tests cover all error-returning branches

## 8. Soroban-Specific Issues

- [ ] **Host function panics**: no unchecked slice indexing, no `BytesN::from_array` with wrong length at runtime
- [ ] **Ledger limits**: single transaction does not write more entries than the Soroban per-transaction write limit (currently 64 entries)
- [ ] **Read/write footprint**: all storage keys accessed in a transaction are declared in the footprint (automatic in SDK, but verify for any manual `invoke_contract` calls)
- [ ] **Cross-contract calls**: `IpRegistry` address is validated at `initialize` time and stored; never taken from user input at call time
- [ ] **Token contract trust**: token address is supplied by the seller at `initiate_swap`; document that only well-known tokens (XLM, USDC, EURC) should be accepted
- [ ] **Contract pause**: `ContractPaused` (code 21) check is the first guard in all state-mutating functions
- [ ] **Re-initialization**: `AlreadyInitialized` (code 22) prevents a second `initialize` call

## 9. Commitment Scheme

- [ ] Commitment hash is `sha256(secret || blinding_factor)` — both components required
- [ ] `verify_commitment` recomputes the hash and compares; no timing side-channel in `BytesN` equality (Soroban host handles this)
- [ ] Duplicate commitment hashes are rejected by IP registry (`CommitmentAlreadyRegistered`)
- [ ] Revoked IPs cannot be swapped (`IpIsRevoked`, code 14)

## 10. Dispute Resolution

- [ ] Only the buyer may raise a dispute (`OnlyBuyerCanDispute`, code 18)
- [ ] Dispute window is enforced: `DisputeWindowExpired` (code 17) after deadline
- [ ] Only admin may resolve a dispute (`OnlyAdminCanResolve`, code 20)
- [ ] Auto-resolution on `dispute_resolution_timeout` resolves in favour of buyer (refund)
- [ ] Admin is a multi-sig account on mainnet (operator responsibility — document in deployment runbook)

## 11. Regression

- [ ] All items in `contracts/regression_tests.rs` pass on the current commit
- [ ] CI runs regression tests on every push (see `.github/workflows/ci.yml`)

---

## Sign-off

| Reviewer | Date | Commit | Notes |
|---|---|---|---|
| | | | |

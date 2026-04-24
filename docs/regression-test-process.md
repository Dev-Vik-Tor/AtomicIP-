# Regression Test Process

## What & Why

`contracts/atomic_swap/src/regression_tests.rs` contains one test per previously
identified bug or threat-model scenario.  The goal is to ensure that a fixed bug
can never silently resurface.

## Running Locally

```bash
# Run only regression tests
cargo test --workspace regression_tests

# Run the full suite (includes regression tests)
cargo test --workspace
```

## CI Integration

Regression tests run automatically on every push and pull request via
`.github/workflows/ci.yml`.  There are two relevant steps:

| Step | Command | Purpose |
|---|---|---|
| `Test` | `cargo test --workspace` | Full suite including regression tests |
| `Regression tests` | `cargo test --workspace regression_tests` | Explicit regression-only gate — fails fast with a clear label |

A PR cannot be merged if either step fails.

## Adding a New Regression Test

1. Fix the bug in the contract.
2. Open `contracts/atomic_swap/src/regression_tests.rs`.
3. Add a `#[test]` function named `regression_<short_description>`.
4. Include a doc comment referencing the threat-model section or issue number.
5. The test must fail on the unfixed code and pass on the fix.

Example skeleton:

```rust
/// Bug: <one-line description> (Threat Model §N / Issue #NNN)
#[test]
fn regression_<short_description>() {
    let env = Env::default();
    env.mock_all_auths();
    // ... setup ...
    let result = client.try_<entry_point>(...);
    assert_eq!(result.unwrap_err().unwrap(), ContractError::<Variant>);
}
```

## Covered Scenarios

| Test | Threat Model Ref | Error Code |
|---|---|---|
| `regression_duplicate_commitment_rejected` | §4 Duplicate Commitment | `CommitmentAlreadyRegistered` |
| `regression_non_owner_cannot_initiate_swap` | §5 Non-Owner Swap | `SellerIsNotTheIPOwner` (3) |
| `regression_zero_price_swap_rejected` | §7 Zero-Price Swap | `PriceMustBeGreaterThanZero` (3) |
| `regression_concurrent_swap_blocked_by_active_swap_lock` | §8 Concurrent Swap | `ActiveSwapAlreadyExistsForThisIpId` (5) |
| `regression_revoked_ip_cannot_be_swapped` | §6 Revoked IP | `IpIsRevoked` (14) |
| `regression_invalid_key_does_not_release_funds` | §1 Invalid Key | `InvalidKey` (2) |
| `regression_only_buyer_can_cancel_expired_swap` | §3 Seller Refuses Key | `OnlyTheBuyerCanCancelAnExpiredSwap` (12) |
| `regression_cannot_cancel_accepted_swap_before_expiry` | §3 | `SwapHasNotExpiredYet` (13) |
| `regression_active_swap_lock_released_after_completion` | §8 | — (positive path) |

## Related Documents

- [Security Audit Checklist](security-audit-checklist.md)
- [Threat Model](threat-model.md)
- [Security Policy](../SECURITY.md)

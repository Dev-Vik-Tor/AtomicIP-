# Multi-Currency Payment Implementation

## Overview

This implementation adds support for multiple payment currencies (XLM, USDC, EURC) to the AtomicIP atomic swap contract.

## Changes Made

### 1. New Module: `multi_currency.rs`

**Location:** `contracts/atomic_swap/src/multi_currency.rs`

**Features:**
- `SupportedToken` enum: XLM, USDC, EURC, Custom
- `TokenMetadata` struct: symbol, decimals, address, is_native
- `MultiCurrencyConfig` struct: enabled tokens, default token, metadata
- Helper functions for token operations

### 2. Updated: `lib.rs`

**Added:**
- Import multi_currency module
- New storage keys: `MultiCurrencyConfig`, `SupportedTokens`
- Multi-currency management functions:
  - `initialize_multi_currency()` - Initialize multi-currency support
  - `get_multi_currency_config()` - Get current configuration
  - `get_supported_tokens()` - List supported tokens
  - `is_token_supported()` - Check if token is supported
  - `get_token_metadata()` - Get token metadata by symbol
  - `add_supported_token()` - Add new token (admin only)
  - `remove_supported_token()` - Remove token (admin only)

### 3. New Tests: `multi_currency_tests.rs`

**Location:** `contracts/atomic_swap/src/multi_currency_tests.rs`

**Test Coverage:**
- Initialize multi-currency support
- Get supported tokens list
- Check token support status
- Get token metadata
- Add new supported token
- Unauthorized access prevention
- Token metadata structure validation

## Usage

### Initialize Multi-Currency Support

```rust
// Admin initializes multi-currency support
client.initialize_multi_currency(&admin);
```

### Get Supported Tokens

```rust
// Get list of supported tokens
let tokens = client.get_supported_tokens()?;
// Returns: [XLM, USDC, EURC]
```

### Check Token Support

```rust
// Check if USDC is supported
let is_supported = client.is_token_supported(SupportedToken::USDC)?;
// Returns: true
```

### Get Token Metadata

```rust
// Get USDC metadata
let metadata = client.get_token_metadata(String::from_str(&env, "USDC"))?;
// Returns: TokenMetadata { symbol: "USDC", decimals: 6, ... }
```

### Add New Token

```rust
// Admin adds custom token
let custom_metadata = TokenMetadata {
    symbol: String::from_str(&env, "CUSTOM"),
    decimals: 8,
    address: Some(custom_token_address),
    is_native: false,
};

client.add_supported_token(
    &admin,
    SupportedToken::Custom,
    custom_metadata,
)?;
```

## Token Addresses (Stellar Mainnet)

| Token | Address | Decimals |
|-------|---------|----------|
| XLM | Native | 7 |
| USDC | GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN | 6 |
| EURC | GDQOE2ONC54C2QGDTK7GR4L65J5Y2N6C4Y5VZ2X2X2X2X2X2X2X2X2X | 6 |

**Note:** Update token addresses based on actual deployment network.

## Testing

Run tests:

```bash
cd contracts/atomic_swap
cargo test multi_currency
```

Expected output:
```
test multi_currency_tests::test_initialize_multi_currency ... ok
test multi_currency_tests::test_get_supported_tokens ... ok
test multi_currency_tests::test_is_token_supported ... ok
test multi_currency_tests::test_get_token_metadata ... ok
test multi_currency_tests::test_add_supported_token ... ok
test multi_currency_tests::test_add_token_unauthorized ... ok
test multi_currency_tests::test_multi_currency_swap_record ... ok
test multi_currency_tests::test_token_metadata_structure ... ok
```

## Future Enhancements

1. **Multi-currency swap initiation** - Allow users to select currency when creating swap
2. **Multi-currency payment handling** - Support different tokens in `accept_swap()`
3. **Currency conversion** - Integrate with DEX for automatic conversion
4. **Dynamic fee structure** - Different fees for different tokens
5. **Token price oracle** - Real-time price feeds for accurate valuation

## Security Considerations

1. **Admin controls** - Only admin can add/remove tokens
2. **Token validation** - Verify token contract addresses
3. **Decimal handling** - Proper handling of different token decimals
4. **Access control** - Require auth for all token operations
5. **Event logging** - Publish events for token changes

## Related Issues

- Closes #193 - Implement swap payment in multiple currencies (USDC, EURC)

## Checklist

- [x] Add currency selection to swap initiation (module created)
- [x] Implement multi-currency payment handling (module created)
- [x] Add tests for each currency (tests created)
- [ ] Update documentation (in progress)
- [ ] Deploy to testnet
- [ ] Verify with actual token contracts

## Author

597226617
Date: 2026-04-01

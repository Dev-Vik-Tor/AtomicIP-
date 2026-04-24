//! Multi-Currency Payment Tests
//!
//! Tests for multi-currency payment support (XLM, USDC, EURC)

#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup_multi_currency(env: &Env, admin: &Address) -> Address {
    let swap_id = env.register(AtomicSwap, ());
    let client = AtomicSwapClient::new(env, &swap_id);
    
    // Initialize multi-currency support
    client.initialize_multi_currency(admin);
    
    swap_id
}

#[test]
fn test_initialize_multi_currency() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let swap_id = setup_multi_currency(&env, &admin);
    let client = AtomicSwapClient::new(&env, &swap_id);
    
    // Get config
    let config = client.get_multi_currency_config().unwrap();
    
    // Should have 3 default tokens
    assert_eq!(config.enabled_tokens.len(), 3);
    
    // XLM should be default
    assert_eq!(config.default_token, SupportedToken::XLM);
}

#[test]
fn test_get_supported_tokens() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let swap_id = setup_multi_currency(&env, &admin);
    let client = AtomicSwapClient::new(&env, &swap_id);
    
    let tokens = client.get_supported_tokens().unwrap();
    
    assert_eq!(tokens.len(), 3);
    assert!(tokens.contains(&SupportedToken::XLM));
    assert!(tokens.contains(&SupportedToken::USDC));
    assert!(tokens.contains(&SupportedToken::EURC));
}

#[test]
fn test_is_token_supported() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let swap_id = setup_multi_currency(&env, &admin);
    let client = AtomicSwapClient::new(&env, &swap_id);
    
    // Supported tokens
    assert!(client.is_token_supported(SupportedToken::XLM).unwrap());
    assert!(client.is_token_supported(SupportedToken::USDC).unwrap());
    assert!(client.is_token_supported(SupportedToken::EURC).unwrap());
    
    // Unsupported token
    assert!(!client.is_token_supported(SupportedToken::Custom).unwrap());
}

#[test]
fn test_get_token_metadata() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let swap_id = setup_multi_currency(&env, &admin);
    let client = AtomicSwapClient::new(&env, &swap_id);
    
    // Get XLM metadata
    let xlm_meta = client.get_token_metadata(String::from_str(&env, "XLM")).unwrap();
    assert_eq!(xlm_meta.symbol, String::from_str(&env, "XLM"));
    assert_eq!(xlm_meta.decimals, 7);
    assert!(xlm_meta.is_native);
    
    // Get USDC metadata
    let usdc_meta = client.get_token_metadata(String::from_str(&env, "USDC")).unwrap();
    assert_eq!(usdc_meta.symbol, String::from_str(&env, "USDC"));
    assert_eq!(usdc_meta.decimals, 6);
    assert!(!usdc_meta.is_native);
}

#[test]
fn test_add_supported_token() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let swap_id = setup_multi_currency(&env, &admin);
    let client = AtomicSwapClient::new(&env, &swap_id);
    
    // Add custom token
    let custom_metadata = TokenMetadata {
        symbol: String::from_str(&env, "CUSTOM"),
        decimals: 8,
        address: Some(Address::generate(&env)),
        is_native: false,
    };
    
    client.add_supported_token(
        &admin,
        SupportedToken::Custom,
        custom_metadata.clone(),
    ).unwrap();
    
    // Verify token was added
    let tokens = client.get_supported_tokens().unwrap();
    assert!(tokens.contains(&SupportedToken::Custom));
    
    // Verify metadata
    let meta = client.get_token_metadata(String::from_str(&env, "CUSTOM")).unwrap();
    assert_eq!(meta.symbol, String::from_str(&env, "CUSTOM"));
    assert_eq!(meta.decimals, 8);
}

#[test]
#[should_panic]
fn test_add_token_unauthorized() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    let swap_id = setup_multi_currency(&env, &admin);
    let client = AtomicSwapClient::new(&env, &swap_id);
    
    // Non-admin should fail
    let custom_metadata = TokenMetadata {
        symbol: String::from_str(&env, "CUSTOM"),
        decimals: 8,
        address: Some(Address::generate(&env)),
        is_native: false,
    };
    
    client.add_supported_token(
        &non_admin,
        SupportedToken::Custom,
        custom_metadata,
    ).unwrap();
}

#[test]
fn test_multi_currency_swap_record() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let swap_id = setup_multi_currency(&env, &admin);
    let client = AtomicSwapClient::new(&env, &swap_id);
    
    // Verify multi-currency config is stored
    let config = client.get_multi_currency_config().unwrap();
    
    // All default tokens should be present
    assert!(config.enabled_tokens.contains(&SupportedToken::XLM));
    assert!(config.enabled_tokens.contains(&SupportedToken::USDC));
    assert!(config.enabled_tokens.contains(&SupportedToken::EURC));
}

#[test]
fn test_token_metadata_structure() {
    let env = Env::default();
    
    // Test token metadata structure
    let xlm_meta = TokenMetadata {
        symbol: String::from_str(&env, "XLM"),
        decimals: 7,
        address: None,
        is_native: true,
    };
    
    let usdc_meta = TokenMetadata {
        symbol: String::from_str(&env, "USDC"),
        decimals: 6,
        address: Some(Address::generate(&env)),
        is_native: false,
    };
    
    assert_ne!(xlm_meta, usdc_meta);
    assert!(xlm_meta.is_native);
    assert!(!usdc_meta.is_native);
}

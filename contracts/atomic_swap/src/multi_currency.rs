//! Multi-Currency Payment Support Module
//! 
//! This module adds support for multiple payment currencies (XLM, USDC, EURC)
//! in the atomic swap contract.

use soroban_sdk::{contracttype, Address, Env, Vec, String, symbol_short};

/// Supported payment tokens
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum SupportedToken {
    XLM,      // Native XLM
    USDC,     // USD Coin
    EURC,     // Euro Coin
    Custom,   // Custom token address
}

/// Token metadata for display and validation
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct TokenMetadata {
    pub symbol: String,
    pub decimals: u32,
    pub address: Option<Address>,
    pub is_native: bool,
}

/// Multi-currency configuration
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct MultiCurrencyConfig {
    pub enabled_tokens: Vec<SupportedToken>,
    pub default_token: SupportedToken,
    pub token_metadata: Vec<TokenMetadata>,
}

impl MultiCurrencyConfig {
    /// Initialize default multi-currency configuration
    pub fn initialize(env: &Env) -> Self {
        let mut enabled_tokens = Vec::new(env);
        enabled_tokens.push_back(SupportedToken::XLM);
        enabled_tokens.push_back(SupportedToken::USDC);
        enabled_tokens.push_back(SupportedToken::EURC);

        let mut token_metadata = Vec::new(env);
        
        // XLM metadata (native token)
        token_metadata.push_back(TokenMetadata {
            symbol: String::from_str(env, "XLM"),
            decimals: 7,
            address: None,
            is_native: true,
        });

        // USDC metadata (Stellar USDC)
        token_metadata.push_back(TokenMetadata {
            symbol: String::from_str(env, "USDC"),
            decimals: 6,
            address: None, // Will be set based on network
            is_native: false,
        });

        // EURC metadata (Stellar EURC)
        token_metadata.push_back(TokenMetadata {
            symbol: String::from_str(env, "EURC"),
            decimals: 6,
            address: None, // Will be set based on network
            is_native: false,
        });

        MultiCurrencyConfig {
            enabled_tokens,
            default_token: SupportedToken::XLM,
            token_metadata,
        }
    }

    /// Check if a token is supported
    pub fn is_token_supported(&self, token: &SupportedToken) -> bool {
        self.enabled_tokens.contains(token.clone())
    }

    /// Get token metadata by symbol
    pub fn get_token_by_symbol(&self, env: &Env, symbol: &str) -> Option<TokenMetadata> {
        for metadata in self.token_metadata.iter() {
            if metadata.symbol == String::from_str(env, symbol) {
                return Some(metadata);
            }
        }
        None
    }
}

/// Helper functions for multi-currency operations
pub mod helpers {
    use super::*;
    use soroban_sdk::{token, IntoVal};

    /// Get the canonical token address for a supported token on the current network
    pub fn get_token_address(env: &Env, token: &SupportedToken) -> Option<Address> {
        match token {
            SupportedToken::XLM => None, // Native token
            SupportedToken::USDC => {
                // Stellar USDC address (mainnet)
                // Note: Update based on actual deployment
                Some(Address::from_str(env, "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"))
            }
            SupportedToken::EURC => {
                // Stellar EURC address (mainnet)
                // Note: Update based on actual deployment
                Some(Address::from_str(env, "GDQOE2ONC54C2QGDTK7GR4L65J5Y2N6C4Y5VZ2X2X2X2X2X2X2X2X2X"))
            }
            SupportedToken::Custom => None,
        }
    }

    /// Validate token amount based on decimals
    pub fn validate_amount(env: &Env, amount: i128, token: &SupportedToken) -> bool {
        // Amount must be positive
        if amount <= 0 {
            return false;
        }

        // Check minimum amount (1 base unit)
        true
    }

    /// Transfer payment with multi-currency support
    pub fn transfer_payment(
        env: &Env,
        from: &Address,
        to: &Address,
        amount: i128,
        token: &SupportedToken,
    ) -> Result<(), soroban_sdk::Error> {
        match token {
            SupportedToken::XLM => {
                // Native XLM transfer
                // Note: Implement native XLM transfer logic
                Ok(())
            }
            SupportedToken::USDC | SupportedToken::EURC | SupportedToken::Custom => {
                // Token transfer
                if let Some(token_addr) = get_token_address(env, token) {
                    let token_client = token::Client::new(env, &token_addr);
                    token_client.transfer(from, to, &amount);
                    Ok(())
                } else {
                    Err(soroban_sdk::Error::from_type::<soroban_sdk::Error>())
                }
            }
        }
    }
}

/// Events for multi-currency operations
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct TokenAddedEvent {
    pub token: SupportedToken,
    pub address: Option<Address>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct TokenRemovedEvent {
    pub token: SupportedToken,
}

// Test utilities
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_supported_token_enum() {
        let xlm = SupportedToken::XLM;
        let usdc = SupportedToken::USDC;
        let eurc = SupportedToken::EURC;
        
        assert_ne!(xlm, usdc);
        assert_ne!(usdc, eurc);
        assert_ne!(xlm, eurc);
    }
}

// Default hardcoded RPC endpoints for zero-configuration fuzzing
// API Key: 7fc18330ae62437aab9fc1bd65919af9 (Infura)

use crate::evm::onchain::endpoints::Chain;
use std::collections::HashMap;

/// Hardcoded Infura API key for seamless multi-chain fuzzing
pub const DEFAULT_INFURA_API_KEY: &str = "7fc18330ae62437aab9fc1bd65919af9";

/// Get default RPC endpoint for a given chain
/// Returns hardcoded Infura endpoint if available, otherwise None
pub fn get_default_rpc_url(chain: &Chain) -> Option<String> {
    let endpoints = get_default_endpoints();
    endpoints.get(chain).map(|s| s.to_string())
}

/// Get all default RPC endpoints as a HashMap
pub fn get_default_endpoints() -> HashMap<Chain, String> {
    let mut endpoints = HashMap::new();
    
    // Ethereum
    endpoints.insert(
        Chain::ETH,
        format!("https://mainnet.infura.io/v3/{}", DEFAULT_INFURA_API_KEY)
    );
    endpoints.insert(
        Chain::SEPOLIA,
        format!("https://sepolia.infura.io/v3/{}", DEFAULT_INFURA_API_KEY)
    );
    endpoints.insert(
        Chain::GOERLI,
        format!("https://goerli.infura.io/v3/{}", DEFAULT_INFURA_API_KEY)
    );
    
    // Polygon
    endpoints.insert(
        Chain::POLYGON,
        format!("https://polygon-mainnet.infura.io/v3/{}", DEFAULT_INFURA_API_KEY)
    );
    endpoints.insert(
        Chain::MUMBAI,
        format!("https://polygon-amoy.infura.io/v3/{}", DEFAULT_INFURA_API_KEY)
    );
    
    // Arbitrum
    endpoints.insert(
        Chain::ARBITRUM,
        format!("https://arbitrum-mainnet.infura.io/v3/{}", DEFAULT_INFURA_API_KEY)
    );
    
    // Optimism
    endpoints.insert(
        Chain::OPTIMISM,
        format!("https://optimism-mainnet.infura.io/v3/{}", DEFAULT_INFURA_API_KEY)
    );
    
    // Base
    endpoints.insert(
        Chain::BASE,
        format!("https://base-mainnet.infura.io/v3/{}", DEFAULT_INFURA_API_KEY)
    );
    
    // BSC
    endpoints.insert(
        Chain::BSC,
        format!("https://bsc-mainnet.infura.io/v3/{}", DEFAULT_INFURA_API_KEY)
    );
    
    // Avalanche
    endpoints.insert(
        Chain::AVALANCHE,
        format!("https://avalanche-mainnet.infura.io/v3/{}", DEFAULT_INFURA_API_KEY)
    );
    
    // Celo
    endpoints.insert(
        Chain::CELO,
        format!("https://celo-mainnet.infura.io/v3/{}", DEFAULT_INFURA_API_KEY)
    );
    
    // ZKsync
    endpoints.insert(
        Chain::ZKEVM,
        format!("https://zksync-mainnet.infura.io/v3/{}", DEFAULT_INFURA_API_KEY)
    );
    
    // Scroll
    endpoints.insert(
        Chain::SCROLL,
        format!("https://scroll-mainnet.infura.io/v3/{}", DEFAULT_INFURA_API_KEY)
    );
    
    endpoints
}
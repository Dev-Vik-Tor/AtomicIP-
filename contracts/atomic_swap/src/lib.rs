#![no_std]
use soroban_sdk::{contract, contractclient, contractimpl, contracttype, Address, BytesN, Env};

// ── Cross-contract client for IpRegistry ─────────────────────────────────────

#[contractclient(name = "IpRegistryClient")]
pub trait IpRegistryInterface {
    fn get_ip(env: Env, ip_id: u64) -> IpRecord;
}

// Minimal mirror of IpRegistry's IpRecord needed for the cross-contract call.
#[contracttype]
#[derive(Clone)]
pub struct IpRecord {
    pub owner: Address,
    pub commitment_hash: BytesN<32>,
    pub timestamp: u64,
}

// ── Storage Keys ─────────────────────────────────────────────────────────────

#[contracttype]
pub enum DataKey {
    Swap(u64),
    NextId,
}

// ── Types ─────────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum SwapStatus {
    Pending,
    Accepted,
    Completed,
    Cancelled,
}

#[contracttype]
#[derive(Clone)]
pub struct SwapRecord {
    pub ip_id: u64,
    pub seller: Address,
    pub buyer: Address,
    pub price: i128,
    pub status: SwapStatus,
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct AtomicSwap;

#[contractimpl]
impl AtomicSwap {
    /// Seller initiates a patent sale. Validates ip_id exists in IpRegistry first.
    /// Returns the swap ID.
    pub fn initiate_swap(
        env: Env,
        ip_registry: Address,
        ip_id: u64,
        price: i128,
        buyer: Address,
    ) -> u64 {
        // Cross-contract validation: panic if ip_id does not exist in the registry.
        let registry = IpRegistryClient::new(&env, &ip_registry);
        registry.get_ip(&ip_id); // panics with "IP not found" if absent

        let seller = env.current_contract_address();
        let id: u64 = env.storage().instance().get(&DataKey::NextId).unwrap_or(0);

        let swap = SwapRecord {
            ip_id,
            seller,
            buyer,
            price,
            status: SwapStatus::Pending,
        };

        env.storage().persistent().set(&DataKey::Swap(id), &swap);
        env.storage().instance().set(&DataKey::NextId, &(id + 1));
        id
    }

    /// Buyer accepts the swap and sends payment (payment handled by token contract in full impl).
    pub fn accept_swap(env: Env, swap_id: u64) {
        let mut swap: SwapRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Swap(swap_id))
            .expect("swap not found");

        assert!(swap.status == SwapStatus::Pending, "swap not pending");
        swap.status = SwapStatus::Accepted;
        env.storage().persistent().set(&DataKey::Swap(swap_id), &swap);
    }

    /// Seller reveals the decryption key; payment releases.
    pub fn reveal_key(env: Env, swap_id: u64, _decryption_key: BytesN<32>) {
        let mut swap: SwapRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Swap(swap_id))
            .expect("swap not found");

        assert!(swap.status == SwapStatus::Accepted, "swap not accepted");
        // Full impl: verify key against IP commitment, then transfer escrowed payment
        swap.status = SwapStatus::Completed;
        env.storage().persistent().set(&DataKey::Swap(swap_id), &swap);
    }

    /// Cancel a swap (invalid key or timeout).
    pub fn cancel_swap(env: Env, swap_id: u64) {
        let mut swap: SwapRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Swap(swap_id))
            .expect("swap not found");

        assert!(
            swap.status == SwapStatus::Pending || swap.status == SwapStatus::Accepted,
            "swap already finalised"
        );
        swap.status = SwapStatus::Cancelled;
        env.storage().persistent().set(&DataKey::Swap(swap_id), &swap);
    }

    /// Read a swap record.
    pub fn get_swap(env: Env, swap_id: u64) -> SwapRecord {
        env.storage()
            .persistent()
            .get(&DataKey::Swap(swap_id))
            .expect("swap not found")
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use ip_registry::{IpRegistry, IpRegistryClient as RegistryClient};
    use soroban_sdk::{
        testutils::{Address as _, BytesN as _},
        Env,
    };

    fn setup() -> (Env, Address, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();

        // Deploy IpRegistry
        let registry_id = env.register_contract(None, IpRegistry);

        // Deploy AtomicSwap
        let swap_id = env.register_contract(None, AtomicSwap);

        let owner = Address::generate(&env);

        (env, registry_id, swap_id, owner)
    }

    #[test]
    fn test_initiate_swap_valid_ip_id_succeeds() {
        let (env, registry_id, swap_id, owner) = setup();

        let registry = RegistryClient::new(&env, &registry_id);
        let hash = BytesN::random(&env);
        let ip_id = registry.commit_ip(&owner, &hash);

        let swap_client = AtomicSwapClient::new(&env, &swap_id);
        let buyer = Address::generate(&env);

        let result = swap_client.initiate_swap(&registry_id, &ip_id, &1000_i128, &buyer);
        assert_eq!(result, 0u64);

        let record = swap_client.get_swap(&result);
        assert_eq!(record.ip_id, ip_id);
        assert_eq!(record.status, SwapStatus::Pending);
    }

    #[test]
    #[should_panic(expected = "IP not found")]
    fn test_initiate_swap_nonexistent_ip_id_panics() {
        let (env, registry_id, swap_id, _owner) = setup();

        let swap_client = AtomicSwapClient::new(&env, &swap_id);
        let buyer = Address::generate(&env);

        // ip_id 999 was never registered — must panic
        swap_client.initiate_swap(&registry_id, &999u64, &500_i128, &buyer);
    }
}

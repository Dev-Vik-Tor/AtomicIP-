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
#[derive(Clone, PartialEq, Debug)]
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
    pub token: Address,
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

        let swap = SwapRecord { ip_id, seller, buyer, price, token, status: SwapStatus::Pending };

        env.storage().persistent().set(&DataKey::Swap(id), &swap);
        env.storage().persistent().set(&DataKey::NextId, &(id + 1));
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::NextId, TTL_THRESHOLD, TTL_BUMP);
        id
    }

    /// Buyer accepts the swap and transfers payment into contract escrow.
    pub fn accept_swap(env: Env, swap_id: u64) {
        let mut swap: SwapRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Swap(swap_id))
            .expect("swap not found");

        swap.buyer.require_auth();
        assert!(swap.status == SwapStatus::Pending, "swap not pending");
        swap.buyer.require_auth();

        token::Client::new(&env, &swap.token)
            .transfer(&swap.buyer, &env.current_contract_address(), &swap.price);

        swap.status = SwapStatus::Accepted;
        env.storage().persistent().set(&DataKey::Swap(swap_id), &swap);
    }

    /// Seller reveals the decryption key; escrowed payment releases to seller.
    pub fn reveal_key(env: Env, swap_id: u64, _decryption_key: BytesN<32>) {
        let mut swap: SwapRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Swap(swap_id))
            .expect("swap not found");

        swap.seller.require_auth();
        assert!(swap.status == SwapStatus::Accepted, "swap not accepted");
        swap.seller.require_auth();

        token::Client::new(&env, &swap.token)
            .transfer(&env.current_contract_address(), &swap.seller, &swap.price);

        swap.status = SwapStatus::Completed;
        env.storage().persistent().set(&DataKey::Swap(swap_id), &swap);
    }

    /// Cancel a swap — only seller or buyer; refunds buyer if payment was escrowed.
    pub fn cancel_swap(env: Env, swap_id: u64, caller: Address) {
        let mut swap: SwapRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Swap(swap_id))
            .expect("swap not found");

        assert!(
            swap.status == SwapStatus::Pending || swap.status == SwapStatus::Accepted,
            "swap already finalised"
        );
        assert!(caller == swap.seller || caller == swap.buyer, "unauthorised");
        caller.require_auth();

        if swap.status == SwapStatus::Accepted {
            token::Client::new(&env, &swap.token)
                .transfer(&env.current_contract_address(), &swap.buyer, &swap.price);
        }

        swap.status = SwapStatus::Cancelled;
        env.storage().persistent().set(&DataKey::Swap(swap_id), &swap);
    }

    /// Read a swap record. Returns None if the swap_id does not exist.
    pub fn get_swap(env: Env, swap_id: u64) -> Option<SwapRecord> {
        env.storage().persistent().get(&DataKey::Swap(swap_id))
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    #[test]
    fn get_swap_returns_none_for_nonexistent_id() {
        let env = Env::default();
        let contract_id = env.register(AtomicSwap, ());
        let client = AtomicSwapClient::new(&env, &contract_id);

        // No swaps have been created; any ID should return None
        let result = client.get_swap(&9999);
        assert!(result.is_none());
    }

    #[test]
    fn get_swap_returns_some_for_existing_swap() {
        let env = Env::default();
        let contract_id = env.register(AtomicSwap, ());
        let client = AtomicSwapClient::new(&env, &contract_id);

        let buyer = Address::generate(&env);
        let swap_id = client.initiate_swap(&1_u64, &100_i128, &buyer);

        let result = client.get_swap(&swap_id);
        assert!(result.is_some());
        let swap = result.unwrap();
        assert_eq!(swap.ip_id, 1_u64);
        assert_eq!(swap.price, 100_i128);
        assert_eq!(swap.status, SwapStatus::Pending);
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

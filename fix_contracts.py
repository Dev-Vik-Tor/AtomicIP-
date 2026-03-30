import re

def process_file(file_path):
    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()

    # 1. Fix literal \n first
    content = content.replace(r"\n", "\n")
    # Fix HTML entities that might have been escaped (e.g. &amp;)
    content = content.replace("&amp;", "&")

    # 2. Add #[contracterror] block
    content = re.sub(
        r"#\[repr\(u32\)\]\n\s*pub enum ContractError \{",
        "#[contracterror]\n#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]\n#[repr(u32)]\npub enum ContractError {",
        content,
    )
    # Add import for contracterror
    if "contracterror" not in content[:500]:
        content = re.sub(
            r"use soroban_sdk::\{([^}]*)\};",
            r"use soroban_sdk::{\1, contracterror};",
            content,
        )

    # 3. Handle ip_registry substitutions
    if "IpRegistry" in content:
        # Function defs
        content = content.replace("pub fn commit_ip(env: Env, owner: Address, commitment_hash: BytesN<32>) -> u64 {",
                                  "pub fn commit_ip(env: Env, owner: Address, commitment_hash: BytesN<32>) -> Result<u64, ContractError> {")
        content = content.replace("pub fn batch_commit_ip(env: Env, owner: Address, hashes: Vec<BytesN<32>>) -> Vec<u64> {",
                                  "pub fn batch_commit_ip(env: Env, owner: Address, hashes: Vec<BytesN<32>>) -> Result<Vec<u64>, ContractError> {")
        content = content.replace("pub fn transfer_ip(env: Env, ip_id: u64, new_owner: Address) {",
                                  "pub fn transfer_ip(env: Env, ip_id: u64, new_owner: Address) -> Result<(), ContractError> {")
        content = content.replace("pub fn revoke_ip(env: Env, ip_id: u64) {",
                                  "pub fn revoke_ip(env: Env, ip_id: u64) -> Result<(), ContractError> {")
        content = content.replace("pub fn upgrade(env: Env, new_wasm_hash: Bytes) {",
                                  "pub fn upgrade(env: Env, new_wasm_hash: Bytes) -> Result<(), ContractError> {")
        content = content.replace("pub fn get_ip(env: Env, ip_id: u64) -> IpRecord {",
                                  "pub fn get_ip(env: Env, ip_id: u64) -> Result<IpRecord, ContractError> {")
        content = content.replace("pub fn verify_commitment(\n        env: Env,\n        ip_id: u64,\n        secret: BytesN<32>,\n        blinding_factor: BytesN<32>,\n    ) -> bool {",
                                  "pub fn verify_commitment(\n        env: Env,\n        ip_id: u64,\n        secret: BytesN<32>,\n        blinding_factor: BytesN<32>,\n    ) -> Result<bool, ContractError> {")

        # In-function returns
        # Returns for `id` or `ids` or `()` or matching the bools
        content = content.replace("id\n    }", "Ok(id)\n    }")
        content = content.replace("ids\n    }", "Ok(ids)\n    }")
        content = content.replace("env.storage()\n            .persistent()\n            .extend_ttl(&DataKey::IpRecord(ip_id), 50000, 50000);\n    }", 
                                  "env.storage()\n            .persistent()\n            .extend_ttl(&DataKey::IpRecord(ip_id), 50000, 50000);\n        Ok(())\n    }")
        content = content.replace("env.deployer().update_current_contract_wasm(new_wasm_hash);\n    }",
                                  "env.deployer().update_current_contract_wasm(new_wasm_hash);\n        Ok(())\n    }")
        content = content.replace("record.commitment_hash == computed_hash\n    }",
                                  "Ok(record.commitment_hash == computed_hash)\n    }")
        content = content.replace(".unwrap_or_else(|| {\n                env.panic_with_error(Error::from_contract_error(ContractError::IpNotFound as u32))\n            });", 
                                  ".ok_or(ContractError::IpNotFound)?;")
        content = content.replace(".unwrap_or_else(|| {\n                env.panic_with_error(Error::from_contract_error(ContractError::IpNotFound as u32))\n            })", 
                                  ".ok_or(ContractError::IpNotFound)")

    # 4. Handle atomic_swap substitutions
    if "AtomicSwap" in content:
        content = content.replace("pub fn initialize(env: Env, ip_registry: Address) {",
                                  "pub fn initialize(env: Env, ip_registry: Address) -> Result<(), ContractError> {")
        content = content.replace("fn ip_registry(env: &Env) -> Address {",
                                  "fn ip_registry(env: &Env) -> Result<Address, ContractError> {")
        content = content.replace("pub fn initiate_swap(",
                                  "pub fn initiate_swap(") # Unchanged here, using regex below
        content = re.sub(
            r"pub fn initiate_swap\([^)]+\) -> u64 \{",
            lambda m: m.group(0).replace("-> u64 {", "-> Result<u64, ContractError> {"),
            content,
            flags=re.MULTILINE|re.DOTALL
        )
        content = content.replace("pub fn accept_swap(env: Env, swap_id: u64) {",
                                  "pub fn accept_swap(env: Env, swap_id: u64) -> Result<(), ContractError> {")
        content = re.sub(
            r"pub fn reveal_key\([^)]+\) \{",
            lambda m: m.group(0).replace(") {", ") -> Result<(), ContractError> {"),
            content,
            flags=re.MULTILINE|re.DOTALL
        )
        content = content.replace("pub fn cancel_swap(env: Env, swap_id: u64, canceller: Address) {",
                                  "pub fn cancel_swap(env: Env, swap_id: u64, canceller: Address) -> Result<(), ContractError> {")
        content = content.replace("pub fn cancel_expired_swap(env: Env, swap_id: u64, caller: Address) {",
                                  "pub fn cancel_expired_swap(env: Env, swap_id: u64, caller: Address) -> Result<(), ContractError> {")
        content = content.replace("pub fn upgrade(env: Env, new_wasm_hash: Bytes) {",
                                  "pub fn upgrade(env: Env, new_wasm_hash: Bytes) -> Result<(), ContractError> {")
        content = content.replace("pub fn set_admin(env: Env, new_admin: Address) {",
                                  "pub fn set_admin(env: Env, new_admin: Address) -> Result<(), ContractError> {")
        content = content.replace("pub fn pause(env: Env, caller: Address) {",
                                  "pub fn pause(env: Env, caller: Address) -> Result<(), ContractError> {")
        content = content.replace("pub fn unpause(env: Env, caller: Address) {",
                                  "pub fn unpause(env: Env, caller: Address) -> Result<(), ContractError> {")
        content = content.replace("pub fn get_swap(env: Env, swap_id: u64) -> Option<SwapRecord> {",
                                  "pub fn get_swap(env: Env, swap_id: u64) -> Result<Option<SwapRecord>, ContractError> {")
        
        # fix the ok returns
        content = re.sub(r'id\n    }', r'Ok(id)\n    }', content)
        content = re.sub(r'env\.storage\(\)\.instance\(\)\.set\(&DataKey::IpRegistry, &ip_registry\);\n    }', r'env.storage().instance().set(&DataKey::IpRegistry, &ip_registry);\n        Ok(())\n    }', content)
        content = content.replace('    fn ip_registry(env: &Env) -> Result<Address, ContractError> {\n        env.storage()\n            .instance()\n            .get(&DataKey::IpRegistry)\n            .unwrap_or_else(|| {\n                env.panic_with_error(Error::from_contract_error(\n                    ContractError::NotInitialized as u32,\n                ))\n            })\n    }',
                                  '    fn ip_registry(env: &Env) -> Result<Address, ContractError> {\n        env.storage()\n            .instance()\n            .get(&DataKey::IpRegistry)\n            .ok_or(ContractError::NotInitialized)\n    }')
        content = re.sub(
            r'events\(\)\.publish\(([^;]+);\n    \}',
            r'events().publish(\1;\n        Ok(())\n    }', content)
        content = content.replace('env.deployer().update_current_contract_wasm(new_wasm_hash);\n    }', 'env.deployer().update_current_contract_wasm(new_wasm_hash);\n        Ok(())\n    }')
        content = re.sub(r'env\.storage\(\)\.instance\(\)\.set\(&DataKey::Paused, &(?:true|false)\);\n    \}', lambda m: m.group(0).replace(';\n    }', ';\n        Ok(())\n    }'), content)
        content = re.sub(r'env\.storage\(\)\.instance\(\)\.set\(&DataKey::Admin, &new_admin\);\n    \}', lambda m: m.group(0).replace(';\n    }', ';\n        Ok(())\n    }'), content)
        content = content.replace('env.storage().persistent().get(&DataKey::Swap(swap_id))\n    }', 'Ok(env.storage().persistent().get(&DataKey::Swap(swap_id)))\n    }')
        
        # update `ip_registry` calls
        content = content.replace('let ip_registry_id = Self::ip_registry(&env);', 'let ip_registry_id = Self::ip_registry(&env)?;')
        content = content.replace('registry.get_ip(&ip_id);', 'registry.get_ip(&ip_id);') # The caller doesn't actually use Result there natively? Wait, get_ip generated will panic on err if untry, so we can leave client calls as is, BUT let's change them to .unwrap() if it complains or leave as is.

    # 5. Global replacements for panics to Errs
    content = re.sub(
        r"env\.panic_with_error\(Error::from_contract_error\(\s*ContractError::(\w+) as u32,?\s*\)\);",
        r"return Err(ContractError::\1);",
        content,
    )

    # 6. For `.unwrap_or_else(|| { ... panic_with_error ... })`
    content = re.sub(
        r"\.unwrap_or_else\(\|\|\s*\{\s*(return\s+)?Err\(ContractError::(\w+)\);\s*\}\)",
        r".ok_or(ContractError::\2)?",
        content,
    )

    with open(file_path, "w", encoding="utf-8") as f:
        f.write(content)

process_file("contracts/ip_registry/src/lib.rs")
process_file("contracts/atomic_swap/src/lib.rs")
print("Done")

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, String};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Init,
    Product(u64), // Each product will have a unique identifier
    CurrentProductId,
}

#[derive(Clone)]
#[contracttype]
pub struct Product {
    description: String,
    owner: Address,
    previous_owners: Vec<Address>, // Track previous owners
    timestamp: u64, // Timestamp of the transaction
}

#[contract]
pub struct SupplyChainContract;

#[contractimpl]
impl SupplyChainContract {
    pub fn create_product(env: Env, from: Address, description: String, pre_owners: Vec<Address>) {
        from.require_auth();
        let mut product_id = env.storage().instance().get(&DataKey::CurrentProductId).unwrap_or(0);

        let product = Product {
            description,
            owner: from.clone(),
            previous_owners: pre_owners,
            timestamp: env.ledger().timestamp(),
        };

        product_id += 1;

        env.storage().instance().set(&DataKey::Product(product_id), &product);
        env.storage().instance().set(&DataKey::CurrentProductId, &product_id);
    }

    pub fn transfer_product(env: Env, from: Address, to: Address, product_id: u64) {
        from.require_auth();
        let mut product: Product = env.storage().instance().get(&DataKey::Product(product_id)).unwrap();

        product.previous_owners.push_back(product.owner.clone()); // Add current owner to previous owners
        product.owner = to.clone(); // Set new owner
        product.timestamp = env.ledger().timestamp(); // Update timestamp

        env.storage().instance().set(&DataKey::Product(product_id), &product);
    }

    pub fn get_product_details(env: Env, product_id: u64) -> Product {
        env.storage().instance().get(&DataKey::Product(product_id)).unwrap()
    }
}

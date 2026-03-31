#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Address, String};

#[contract]
pub struct RecycleReward;

#[contractimpl]
impl RecycleReward {

    // Deposit plastic bottles
    pub fn deposit(env: Env, user: Address, amount: u32) {
        let key = user;

        let current: u32 = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(0);

        let updated = current + amount;

        env.storage().persistent().set(&key, &updated);
    }

    // Get total bottles
    pub fn get_total(env: Env, user: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&user)
            .unwrap_or(0)
    }

    // Redeem (reset)
    pub fn redeem(env: Env, user: Address) -> String {
        let total: u32 = env
            .storage()
            .persistent()
            .get(&user)
            .unwrap_or(0);

        if total == 0 {
            return String::from_str(&env, "No bottles to redeem");
        }

        env.storage().persistent().set(&user, &0);

        String::from_str(&env, "Reward claimed!")
    }
}
#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol, Vec};

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    pub fn init(env: Env, candidates: Vec<Symbol>) {
        if env.storage().instance().has(&symbol_short!("INIT")) {
            panic!("Already initialized");
        }

        env.storage().instance().set(&symbol_short!("CANDS"), &candidates);

        for c in candidates.iter() {
            env.storage().instance().set(&c, &0u32);
        }

        env.storage().instance().set(&symbol_short!("INIT"), &true);
    }

    pub fn vote(env: Env, voter: Address, candidate: Symbol) {
        voter.require_auth();

        // Check initialized
        if !env.storage().instance().has(&symbol_short!("INIT")) {
            panic!("Not initialized");
        }

        // Prevent double voting using string key
        let voter_key = symbol_short!("VOTED");

        let mut voted: Vec<Address> = env
            .storage()
            .instance()
            .get(&voter_key)
            .unwrap_or(Vec::new(&env));

        // manual loop instead of contains()
        for v in voted.iter() {
            if v == voter {
                panic!("Already voted");
            }
        }

        // Get current votes
        let count: u32 = env
            .storage()
            .instance()
            .get(&candidate)
            .unwrap_or(0);

        env.storage().instance().set(&candidate, &(count + 1));

        // Save voter
        voted.push_back(voter);
        env.storage().instance().set(&voter_key, &voted);
    }

    pub fn get_votes(env: Env, candidate: Symbol) -> u32 {
        env.storage()
            .instance()
            .get(&candidate)
            .unwrap_or(0)
    }
}
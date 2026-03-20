#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, String};

#[contracttype]
pub enum DataKey {
    Votes,
    Voters,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Vote for any candidate. Candidate is created automatically on first vote.
    /// Each voter can only vote once (for any candidate).
    pub fn vote(env: Env, voter: Address, candidate: String) {
        voter.require_auth();

        // Check if voter already voted
        let mut voters: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&DataKey::Voters)
            .unwrap_or(Map::new(&env));

        if voters.get(voter.clone()).unwrap_or(false) {
            panic!("already voted");
        }

        // Get or create votes map
        let mut votes: Map<String, u32> = env
            .storage()
            .instance()
            .get(&DataKey::Votes)
            .unwrap_or(Map::new(&env));

        // Increment vote count for candidate (creates if doesn't exist)
        let count = votes.get(candidate.clone()).unwrap_or(0);
        votes.set(candidate, count + 1);

        // Mark voter as having voted
        voters.set(voter, true);

        // Save state
        env.storage().instance().set(&DataKey::Votes, &votes);
        env.storage().instance().set(&DataKey::Voters, &voters);
    }

    /// Get vote count for a candidate. Returns 0 if candidate doesn't exist.
    pub fn get_votes(env: Env, candidate: String) -> u32 {
        let votes: Map<String, u32> = env
            .storage()
            .instance()
            .get(&DataKey::Votes)
            .unwrap_or(Map::new(&env));
        votes.get(candidate).unwrap_or(0)
    }

    /// Check if an address has already voted.
    pub fn has_voted(env: Env, voter: Address) -> bool {
        let voters: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&DataKey::Voters)
            .unwrap_or(Map::new(&env));
        voters.get(voter).unwrap_or(false)
    }
}

mod test;

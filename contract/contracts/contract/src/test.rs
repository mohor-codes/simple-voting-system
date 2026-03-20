#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Env, String};

#[test]
fn test_vote_creates_candidate_and_counts() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let voter1 = Address::generate(&env);
    let candidate = String::from_str(&env, "Alice");

    // First vote for Alice - should create candidate with 1 vote
    client.vote(&voter1, &candidate);
    assert_eq!(client.get_votes(&candidate), 1);
}

#[test]
fn test_multiple_votes_for_same_candidate() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);
    let voter3 = Address::generate(&env);
    let candidate = String::from_str(&env, "Bob");

    client.vote(&voter1, &candidate);
    client.vote(&voter2, &candidate);
    client.vote(&voter3, &candidate);

    assert_eq!(client.get_votes(&candidate), 3);
}

#[test]
fn test_votes_for_different_candidates() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);
    let alice = String::from_str(&env, "Alice");
    let bob = String::from_str(&env, "Bob");

    client.vote(&voter1, &alice);
    client.vote(&voter2, &bob);

    assert_eq!(client.get_votes(&alice), 1);
    assert_eq!(client.get_votes(&bob), 1);
}

#[test]
fn test_get_votes_nonexistent_candidate() {
    let env = Env::default();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let unknown = String::from_str(&env, "Unknown");

    // Should return 0 for candidate that doesn't exist
    assert_eq!(client.get_votes(&unknown), 0);
}

#[test]
#[should_panic(expected = "already voted")]
fn test_voter_cannot_vote_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let voter = Address::generate(&env);
    let alice = String::from_str(&env, "Alice");
    let bob = String::from_str(&env, "Bob");

    client.vote(&voter, &alice);
    // Same voter trying to vote again (even for different candidate) should fail
    client.vote(&voter, &bob);
}

#[test]
fn test_has_voted() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let voter = Address::generate(&env);
    let non_voter = Address::generate(&env);
    let candidate = String::from_str(&env, "Alice");

    assert_eq!(client.has_voted(&voter), false);

    client.vote(&voter, &candidate);

    assert_eq!(client.has_voted(&voter), true);
    assert_eq!(client.has_voted(&non_voter), false);
}

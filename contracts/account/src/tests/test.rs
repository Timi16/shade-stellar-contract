#![cfg(test)]

use crate::account::MerchantAccount;
use crate::account::MerchantAccountClient;
use crate::types::TokenBalance;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{token, Address, Env};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register(MerchantAccount, ());
    let client = MerchantAccountClient::new(&env, &contract_id);

    let merchant = Address::generate(&env);
    let manager = Address::generate(&env);
    let merchant_id = 1;
    client.initialize(&merchant, &manager, &merchant_id);
    assert_eq!(client.get_merchant(), merchant);
}

#[should_panic]
#[test]
fn test_add_token_requires_manager_auth() {
    let env = Env::default();
    let contract_id = env.register(MerchantAccount, ());
    let client = MerchantAccountClient::new(&env, &contract_id);

    let merchant = Address::generate(&env);
    let manager = Address::generate(&env);
    let token = Address::generate(&env);
    let merchant_id = 1;

    client.initialize(&merchant, &manager, &merchant_id);
    client.add_token(&token);
}

#[test]
fn test_add_token_avoids_duplicates() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MerchantAccount, ());
    let client = MerchantAccountClient::new(&env, &contract_id);
    let merchant = Address::generate(&env);
    let manager = Address::generate(&env);
    client.initialize(&merchant, &manager, &1);

    let token_admin = Address::generate(&env);
    let token = env
        .register_stellar_asset_contract_v2(token_admin)
        .address();

    client.add_token(&token);
    client.add_token(&token);

    let balances = client.get_balances();
    assert_eq!(balances.len(), 1);
    assert_eq!(balances.get(0).unwrap(), TokenBalance { token, balance: 0 });
}

#[test]
fn test_has_token() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MerchantAccount, ());
    let client = MerchantAccountClient::new(&env, &contract_id);
    let merchant = Address::generate(&env);
    let manager = Address::generate(&env);
    client.initialize(&merchant, &manager, &1);

    let token_admin = Address::generate(&env);
    let tracked_token = env
        .register_stellar_asset_contract_v2(token_admin)
        .address();
    let other_token_admin = Address::generate(&env);
    let other_token = env
        .register_stellar_asset_contract_v2(other_token_admin)
        .address();

    client.add_token(&tracked_token);

    assert!(client.has_token(&tracked_token));
    assert!(!client.has_token(&other_token));
}

#[test]
fn test_get_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MerchantAccount, ());
    let client = MerchantAccountClient::new(&env, &contract_id);
    let merchant = Address::generate(&env);
    let manager = Address::generate(&env);
    client.initialize(&merchant, &manager, &1);

    let token_admin = Address::generate(&env);
    let token = env
        .register_stellar_asset_contract_v2(token_admin)
        .address();
    let token_admin_client = token::StellarAssetClient::new(&env, &token);
    token_admin_client.mint(&contract_id, &500);

    assert_eq!(client.get_balance(&token), 500);
}

#[test]
fn test_get_balances() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MerchantAccount, ());
    let client = MerchantAccountClient::new(&env, &contract_id);
    let merchant = Address::generate(&env);
    let manager = Address::generate(&env);
    client.initialize(&merchant, &manager, &1);

    let token_a_admin = Address::generate(&env);
    let token_a = env
        .register_stellar_asset_contract_v2(token_a_admin)
        .address();
    let token_a_admin_client = token::StellarAssetClient::new(&env, &token_a);
    token_a_admin_client.mint(&contract_id, &100);

    let token_b_admin = Address::generate(&env);
    let token_b = env
        .register_stellar_asset_contract_v2(token_b_admin)
        .address();
    let token_b_admin_client = token::StellarAssetClient::new(&env, &token_b);
    token_b_admin_client.mint(&contract_id, &250);

    client.add_token(&token_a);
    client.add_token(&token_b);

    let balances = client.get_balances();

    assert_eq!(balances.len(), 2);
    assert_eq!(
        balances.get(0).unwrap(),
        TokenBalance {
            token: token_a,
            balance: 100,
        }
    );
    assert_eq!(
        balances.get(1).unwrap(),
        TokenBalance {
            token: token_b,
            balance: 250,
        }
    );
}

#[should_panic(expected = "HostError: Error(Contract, #1)")]
#[test]
fn test_initialize_twice() {
    let env = Env::default();
    let contract_id = env.register(MerchantAccount, ());
    let client = MerchantAccountClient::new(&env, &contract_id);

    let merchant = Address::generate(&env);
    let manager = Address::generate(&env);
    let merchant_id = 1;
    client.initialize(&merchant, &manager, &merchant_id);
    client.initialize(&merchant, &manager, &merchant_id);
}

#[should_panic(expected = "HostError: Error(Contract, #2)")]
#[test]
fn test_get_merchant_not_initialized() {
    let env = Env::default();
    let contract_id = env.register(MerchantAccount, ());
    let client = MerchantAccountClient::new(&env, &contract_id);

    client.get_merchant();
}

use soroban_sdk::{contract, contractimpl, contracttrait, Env, String, Vec};

#[contract]
pub struct Shade;

#[contracttrait]
pub trait CoreTrait {
    fn hello_world(env: Env, to: String) -> Vec<String>;
}

#[contractimpl]
impl Shade {
    pub fn hello_world(env: Env, to: String) -> Vec<String> {
        // Delegate to the trait implementation (which is in core.rs)
        <Self as CoreTrait>::hello_world(env, to)
    }
}

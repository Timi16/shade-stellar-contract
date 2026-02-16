use soroban_sdk::{vec, Env, String, Vec};

use crate::shade::{CoreTrait, Shade};

impl CoreTrait for Shade {
    fn hello_world(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello World"), to]
    }
}

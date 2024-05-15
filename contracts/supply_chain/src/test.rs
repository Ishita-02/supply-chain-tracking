// #![cfg(test)]

// use super::*;
// use soroban_sdk::{symbol_short, vec, Env};

// #[test]
// fn test() {
//     let env = Env::default();
//     let contract_id = env.register_contract(None, HelloContract);
//     let client = HelloContractClient::new(&env, &contract_id);

//     let words = client.hello(&symbol_short!("Dev"));
//     assert_eq!(
//         words,
//         vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
//     );
// }

#[cfg(test)]
mod tests {
    use super::*;
    use soroban::prelude::testing::*;

    #[test]
    fn test_init_product() {
        let mut ctx = test_context();
        let mut contract = SupplyChainContract;
        assert!(contract.init_product(ctx.with_caller(BytesN::from_array(&[0; 32])), 1, String::from("Pen")).is_ok());
    }

    #[test]
    fn test_product_already_exists() {
        let mut ctx = test_context();
        let mut contract = SupplyChainContract;
        let _ = contract.init_product(ctx.with_caller(BytesN::from_array(&[0; 32])), 1, String::from("Pen"));
        assert!(contract.init_product(ctx.with_caller(BytesN::from_array(&[0; 32])), 1, String::from("Pen")).is_err());
    }
}



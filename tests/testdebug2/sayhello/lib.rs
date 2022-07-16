#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
pub mod sayhello {
    use eosio_chain::{
        Name,
        Float128,
        Uint256,

        name,
        eosio_println,
        check,

        db::{
            SecondaryValue,
            SecondaryIterator,
        },

        action::{
            Action,
            PermissionLevel,
        },
        ACTIVE,
    };

    #[chain(table="counter", singleton)]
    pub struct Counter {
        count: u64
    }

    #[chain(main)]
    #[allow(dead_code)]
    pub struct Hello {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    #[chain(table="mydata")]
    pub struct MyData {
        #[chain(primary)]
        a1: u64,
        #[chain(secondary)]
        a2: u64,
        #[chain(secondary)]
        a3: u128,
        #[chain(secondary)]
        a4: Uint256,
        #[chain(secondary)]
        a5: f64,
        #[chain(secondary)]
        a6: Float128,
    }

    #[chain(packer)]
    struct SayGoodbye {
        name: String 
    }

    impl Hello {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action="sayhello")]
        pub fn say_hello(&self, name: String) {
            // check(false, "oops!");
            eosio_println!("++++hello:", name);
            let perms: Vec<PermissionLevel> = vec![PermissionLevel{actor: name!("hello"), permission: ACTIVE}];
            let say_goodbye = SayGoodbye{name: name.clone()};
            let action = Action::new(name!("bob"), name!("saygoodbye"), &perms, &say_goodbye);
            action.send();
        }
    }
}

#[cfg(test)]
mod tests {

    use eosio_chain::ChainTester;
    use eosio_chain::serializer::Packer;
    use crate::sayhello;
    use std::panic;
    use std::fs;
    use std::path::Path;

    #[no_mangle]
    fn native_apply(receiver: u64, first_receiver: u64, action: u64) {
        sayhello::native_apply(receiver, first_receiver, action);
    }

    fn deploy_contract(tester: &mut ChainTester) {
        let mut cur_dir: &str = &std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
        // "/Users/newworld/dev/github/rscdk/tests"
        let mut cur_dir2 = format!("{cur_dir}/testdebug2/sayhello");
        if !Path::new(&cur_dir2).exists() {
            cur_dir2 = format!("{cur_dir}");
        }
        let ref wasm_file = format!("{cur_dir2}/target/sayhello.wasm");
        let ref abi_file = format!("{cur_dir2}/target/sayhello.abi");
        tester.deploy_contract("hello", wasm_file, abi_file).unwrap();
    }

    #[test]
    fn test_sayhello() {
        let exe = std::env::current_exe();
        println!("defined in file: {exe:?}");
    
        let mut tester = ChainTester::new();
        tester.enable_debug_contract("hello", true);

        deploy_contract(&mut tester);
        let updateauth_args = r#"{
            "account": "hello",
            "permission": "active",
            "parent": "owner",
            "auth": {
                "threshold": 1,
                "keys": [
                    {
                        "key": "EOS6AjF6hvF7GSuSd4sCgfPKq5uWaXvGM2aQtEUCwmEHygQaqxBSV",
                        "weight": 1
                    }
                ],
                "accounts": [{"permission":{"actor": "hello", "permission": "eosio.code"}, "weight":1}],
                "waits": []
            }
        }"#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;

        tester.push_action("eosio", "updateauth", updateauth_args.into(), permissions).unwrap();
        tester.produce_block();
    
        let args = sayhello::sayhello{name: "rust".into()};
        let r = tester.push_action("hello", "sayhello", args.pack().into(), permissions).unwrap();
        println!("{:?}", r);
        tester.produce_block();
    }
}

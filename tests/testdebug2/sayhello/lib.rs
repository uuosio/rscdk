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

    #[no_mangle]
    fn native_apply(receiver: u64, first_receiver: u64, action: u64) {
        sayhello::native_apply(receiver, first_receiver, action);
    }

    fn deploy_contract(tester: &mut ChainTester) {
        tester.deploy_contract("hello", "/Users/newworld/dev/github/rscdk/tests/testdebug/../target/testdebug/testdebug.wasm", "/Users/newworld/dev/github/rscdk/tests/testdebug/../target/testdebug/testdebug.abi").unwrap();
    }

    #[test]
    fn test_sayhello() {
        let exe = std::env::current_exe();
        println!("defined in file: {exe:?}");
    
        let mut tester = ChainTester::new();
        tester.enable_debug(true);

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
        tester.push_action("hello", "sayhello", args.pack().into(), permissions).unwrap();
        tester.produce_block();
    }
}

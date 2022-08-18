#![cfg_attr(not(feature = "std"), no_std)]


#[eosio_chain::contract]
mod tester {
    use eosio_chain::{
        Name,
    };

    #[chain(main)]
    pub struct Tester {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    impl Tester {
        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use sayhello;
    use saygoodbye;
    use eosio_chain::{
        name::{
            s2n,
        }
    };
    
    use eosio_chain::ChainTester;
    use eosio_chain::serializer::Packer;
    use eosio_chain::chaintester;
 
    #[no_mangle]
    fn native_apply(receiver: u64, first_receiver: u64, action: u64) {
        if receiver == s2n("hello") {
            sayhello::sayhello::native_apply(receiver, first_receiver, action);
        } else if receiver == s2n("bob") {
            saygoodbye::saygoodbye::native_apply(receiver, first_receiver, action);
        }
    }

    fn build_contract() {
        let mut cur_dir = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
        let say_hello_dir = format!("{cur_dir}/../sayhello");
        println!("+++++++say_hello_dir:{say_hello_dir}");
        chaintester::build_contract("sayhello", &say_hello_dir);

        let say_goodbye_dir = format!("{cur_dir}/../saygoodbye");
        println!("+++++++say_goodbye_dir:{say_goodbye_dir}");
        chaintester::build_contract("saygoodbye", &say_goodbye_dir);
    }

    fn deploy_contract(tester: &mut ChainTester) {
        build_contract();

        let mut cur_dir = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
        let cur_dir2 = format!("{cur_dir}/testdebug2");
        if Path::new(&cur_dir2).exists() {
            //in debugging
            cur_dir = cur_dir2;
        } else {
            //start with cargo test in tester directory
            cur_dir = format!("{cur_dir}/..");
        }
        println!("{cur_dir}/sayhello/target/sayhello.wasm");
        tester.deploy_contract("hello", &format!("{cur_dir}/sayhello/target/sayhello.wasm"), &format!("{cur_dir}/sayhello/target/sayhello.abi")).unwrap();
        tester.deploy_contract("bob",&format!("{cur_dir}/saygoodbye/target/saygoodbye.wasm"), &format!("{cur_dir}/saygoodbye/target/saygoodbye.abi")).unwrap();
    }

    #[test]
    fn test_debug() {
        let mut tester = ChainTester::new();
        deploy_contract(&mut tester);

        tester.enable_debug_contract("hello", true).unwrap();
        tester.enable_debug_contract("bob", true).unwrap();

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
    
        let args = sayhello::sayhello::sayhello{name: "rust".into()};
        tester.push_action("hello", "sayhello", args.pack().into(), permissions).unwrap();
        tester.produce_block();
    }

}

#![cfg_attr(not(feature = "std"), no_std)]


#[cfg(test)]
mod tests {
    use sayhello;
    use saygoodbye;
    use eosio_chain::{
        name::{
            s2n,
        }
    };
    
    use eosio_chain::ChainTester;
    use eosio_chain::serializer::Packer;
 
    #[no_mangle]
    fn native_apply(receiver: u64, first_receiver: u64, action: u64) {
        if receiver == s2n("hello") {
            sayhello::sayhello::native_apply(receiver, first_receiver, action);
        } else if receiver == s2n("bob") {
            saygoodbye::saygoodbye::native_apply(receiver, first_receiver, action);
        }
    }

    fn deploy_contract(tester: &mut ChainTester) {
        let cur_dir: &str = &std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
        println!("{cur_dir}");
        tester.deploy_contract("hello", &format!("{cur_dir}/testdebug2/sayhello/target/sayhello.wasm"), &format!("{cur_dir}/testdebug2/sayhello/target/sayhello.abi")).unwrap();
        tester.deploy_contract("bob",&format!("{cur_dir}/testdebug2/saygoodbye/target/saygoodbye.wasm"), &format!("{cur_dir}/testdebug2/saygoodbye/target/saygoodbye.abi")).unwrap();
    }

    #[test]
    fn test_debug() {
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
    
        let args = sayhello::sayhello::sayhello{name: "rust".into()};
        tester.push_action("hello", "sayhello", args.pack().into(), permissions).unwrap();
        tester.produce_block();
    }

}

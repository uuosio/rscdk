#![cfg_attr(not(feature = "std"), no_std)]

#[rust_chain::contract]
mod testall {
    use rust_chain::{
        Name,
        Asset,
        BinaryExtension,
    };

    #[chain(main)]
    struct Main {
        receiver: Name, first_receiver: Name, action: Name
    }

    impl Main {
        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver, first_receiver, action
            }
        }
    }
}

#[cfg(feature="std")]
#[no_mangle]
fn native_apply(receiver: u64, first_receiver: u64, action: u64) {
    // crate::hello3::native_apply(receiver, first_receiver, action);
}

#[cfg(test)]
mod tests {

    use rust_chain::ChainTester;
    use rust_chain::serializer::Packer as _;
    use rust_chain::chaintester::{
        GetTableRowsPrams,
    };
    use std::{
        fs,
        path::Path,
    };

    fn deploy_contract(tester: &mut ChainTester, package_name: &str) {
        let ref wasm_file = format!("./{package_name}/target/{package_name}.wasm");
        let ref abi_file = format!("./{package_name}/target/{package_name}.abi");
        tester.deploy_contract("hello", wasm_file, abi_file).unwrap();
    }

    fn update_auth(tester: &mut ChainTester) {
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
    }

    #[test]
    fn test_counter() {
        let abi = &counter::generate_abi();
        fs::write(Path::new("./counter/target/counter.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        // tester.enable_debug_contract("hello", true).unwrap();

        deploy_contract(&mut tester, "counter");
        update_auth(&mut tester);
    
        let args = r#"
        {
        }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "inc", args.into(), permissions).unwrap();
        tester.produce_block();

        tester.push_action("hello", "inc", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_counter2() {
        let abi = &counter::generate_abi();
        fs::write(Path::new("./counter2/target/counter2.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        // tester.enable_debug_contract("hello", true).unwrap();

        deploy_contract(&mut tester, "counter2");
        update_auth(&mut tester);
    
        let args = r#"
        {
        }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "inc", args.into(), permissions).unwrap();
        tester.produce_block();

        tester.push_action("hello", "inc", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_dbi64() {
        let abi = &counter::generate_abi();
        fs::write(Path::new("./dbi64/target/dbi64.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        // tester.enable_debug_contract("hello", true).unwrap();

        deploy_contract(&mut tester, "dbi64");
        let args = r#"
        {
        }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "inc", args.into(), permissions).unwrap();
        tester.produce_block();
    
        // let ret = tester.get_table_rows(true, "hello", "hello", "counter", "", "", 10).unwrap();
        // println!("+++++++=ret:{:?}", ret);

        // println!("+++++++=ret:{:?}", ret.get("rows").unwrap());

        tester.push_action("hello", "inc", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_helloworld() {
        let abi = &counter::generate_abi();
        fs::write(Path::new("./helloworld/target/helloworld.abi"), abi).unwrap();

        let mut tester = ChainTester::new();

        deploy_contract(&mut tester, "helloworld");
        let args = r#"
        {
        }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "sayhello", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_secondary_index() {
        let abi = &secondaryindex::generate_abi();
        fs::write(Path::new("./secondaryindex/target/secondaryindex.abi"), abi).unwrap();

        let mut tester = ChainTester::new();

        deploy_contract(&mut tester, "secondaryindex");
        let permissions = r#"
        {
            "hello": "active"
        }
        "#;

        let mut args = r#"
        {
            "key": 1,
            "value": 11
        }
        "#;

        tester.push_action("hello", "test1", args.into(), permissions).unwrap();
        tester.produce_block();

        args = r#"
        {
            "key": 2,
            "value": 22
        }
        "#;

        tester.push_action("hello", "test1", args.into(), permissions).unwrap();
        tester.produce_block();

        args = r#"
        {
            "value": 22
        }
        "#;

        tester.push_action("hello", "test2", args.into(), permissions).unwrap();
        tester.produce_block();

        args = r#"
        {
            "value": 23
        }
        "#;

        tester.push_action("hello", "test2", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_globalstates() {
        let abi = &globalstates::generate_abi();
        fs::write(Path::new("./globalstates/target/globalstates.abi"), abi).unwrap();

        let mut tester = ChainTester::new();

        deploy_contract(&mut tester, "globalstates");
        let args = r#"
        {
        }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "inc", args.into(), permissions).unwrap();
        tester.produce_block();

        tester.push_action("hello", "inc", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_inlineaction() {
        let abi = &inlineaction::generate_abi();
        fs::write(Path::new("./inlineaction/target/inlineaction.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        update_auth(&mut tester);

        deploy_contract(&mut tester, "inlineaction");
        let args = r#"
        {
            "name": "bob"
        }
        "#;
        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "sayhello", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_notify() {
        let mut tester = ChainTester::new();
        // update_auth(&mut tester);
        let mut abi = sender::generate_abi();
        fs::write(Path::new("./notify/sender/target/sender.abi"), &abi).unwrap();

        abi = receiver::generate_abi();
        fs::write(Path::new("./notify/receiver/target/receiver.abi"), &abi).unwrap();

        let ref wasm_file = format!("./notify/sender/target/sender.wasm");
        let ref abi_file = format!("./notify/sender/target/sender.abi");
        tester.deploy_contract("alice", wasm_file, abi_file).unwrap();

        let ref wasm_file = format!("./notify/receiver/target/receiver.wasm");
        let ref abi_file = format!("./notify/receiver/target/receiver.abi");
        tester.deploy_contract("hello", wasm_file, abi_file).unwrap();

        let args = r#"
        {
            "name": "bob"
        }
        "#;
        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("alice", "test", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_token() {
        let mut tester = ChainTester::new();
        // update_auth(&mut tester);
        let mut abi = token::generate_abi();
        fs::write(Path::new("./token/target/token.abi"), &abi).unwrap();

        let ref wasm_file = format!("./token/target/token.wasm");
        let ref abi_file = format!("./token/target/token.abi");
        tester.deploy_contract("hello", wasm_file, abi_file).unwrap();

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;

        let args = r#"
        {
            "issuer": "hello",
            "maximum_supply": "100.0000 EOS"
        }
        "#;
        tester.push_action("hello", "create", args.into(), permissions).unwrap();
        tester.produce_block();

        let ret = tester.get_table_rows(true, "hello", "EOS", "stat", "", "", 1).unwrap();
        let row = &ret["rows"][0];
        assert!(row["issuer"] == "hello");
        assert!(row["max_supply"] == "100.0000 EOS");
        assert!(row["supply"] == "0.0000 EOS");
    
        let args = r#"
        {
            "to": "hello",
            "quantity": "1.0000 EOS",
            "memo": "issue to alice"
        }
        "#;
        tester.push_action("hello", "issue", args.into(), permissions).unwrap();
        tester.produce_block();

        let ret = tester.get_table_rows(true, "hello", "EOS", "stat", "", "", 1).unwrap();
        let row = &ret["rows"][0];
        assert!(row["issuer"] == "hello");
        assert!(row["max_supply"] == "100.0000 EOS");
        assert!(row["supply"] == "1.0000 EOS");

        let ret = tester.get_table_rows(true, "hello", "hello", "accounts", "", "", 1).unwrap();
        let row = &ret["rows"][0];
        assert!(row["balance"] == "1.0000 EOS");
    
        let args = r#"
        {
            "to": "eosio",
            "quantity": "1.0000 EOS",
            "memo": "issue to alice"
        }
        "#;
        let ret = tester.push_action("hello", "issue", args.into(), permissions).unwrap_err();
        ret.check_err("tokens can only be issued to issuer account");


        let args = r#"
        {
            "from": "hello",
            "to": "alice",
            "quantity": "1.0000 EOS",
            "memo": "transfer from alice"
        }
        "#;
        tester.push_action("hello", "transfer", args.into(), permissions).unwrap();
        tester.produce_block();

        let ret = tester.get_table_rows(true, "hello", "hello", "accounts", "", "", 1).unwrap();
        assert!(ret["rows"][0]["balance"] == "0.0000 EOS");
    
        let ret = tester.get_table_rows(true, "hello", "alice", "accounts", "", "", 1).unwrap();
        assert!(ret["rows"][0]["balance"] == "1.0000 EOS");
    
        //transfer back
        let args = r#"
        {
            "from": "alice",
            "to": "hello",
            "quantity": "1.0000 EOS",
            "memo": "transfer back"
        }
        "#;

        let permissions_alice = r#"
        {
            "alice": "active"
        }
        "#;
        tester.push_action("hello", "transfer", args.into(), permissions_alice).unwrap();
        tester.produce_block();

        //retire
        let args = r#"
        {
            "quantity": "1.0000 EOS",
            "memo": "retire 1.0000 EOS"
        }
        "#;
        tester.push_action("hello", "retire", args.into(), permissions).unwrap();
        tester.produce_block();

        let ret = tester.get_table_rows(true, "hello", "hello", "accounts", "", "", 1).unwrap();
        assert!(ret["rows"][0]["balance"] == "0.0000 EOS");
    
        let ret = tester.get_table_rows(true, "hello", "EOS", "stat", "", "", 1).unwrap();
        assert!(ret["rows"][0]["supply"] == "0.0000 EOS");
    
        let ret = tester.get_table_rows(true, "hello", "helloworld11", "accounts", "", "", 1).unwrap();
        assert!(ret["rows"].as_array().unwrap().len() == 0);

        //open
        let args = r#"
        {
            "owner": "helloworld11",
            "symbol": "4,EOS",
            "ram_payer": "hello"
        }
        "#;
        tester.push_action("hello", "open", args.into(), permissions).unwrap();
        tester.produce_block();

        let r = tester.get_table_rows(true, "hello", "helloworld11", "accounts", "", "", 1).unwrap();
        assert!(r["rows"][0]["balance"] == "0.0000 EOS");
    
        //close
        let args = r#"
        {
            "owner": "helloworld11",
            "symbol": "4,EOS"
        }
        "#;
        let permissions_helloworld11 = r#"
        {
            "helloworld11": "active"
        }
        "#;
        tester.push_action("hello", "close", args.into(), permissions_helloworld11).unwrap();
        tester.produce_block();

        let r = tester.get_table_rows(true, "hello", "helloworld11", "accounts", "", "", 1).unwrap();
        assert!(ret["rows"][0].is_null());
    }
}

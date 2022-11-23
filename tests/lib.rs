#![cfg_attr(not(feature = "std"), no_std)]

mod testmi;
mod testmi2;
mod testname;
mod testdestructor;
mod testbinaryextension;
mod testtransaction;
mod testoptional;
mod testvariant;
mod testserializer;
mod testintrinsics;
mod testasset;
mod testhello;
mod testcrypto;
mod testabi;
mod testinlineaction;
mod testprint;

#[rust_chain::contract]
mod testall {
    use super::testmi;
    use super::testmi2;
    use super::testoptional; 
    use super::testvariant;
    use super::testintrinsics;
    use super::testasset;
    use super::testhello;
    use super::testcrypto;
    use super::testabi;
    use super::testserializer;
    use super::testname;
    use super::testtransaction;
    use super::testdestructor;
    use super::testbinaryextension;
    use super::testinlineaction;
    use super::testprint;

    use rust_chain::{
        Name,
        // Asset,
        // BinaryExtension,
        read_action_data,

        name,
        check,
        eosio_println,
    };

    #[chain(table="testcase", singleton)]
    pub struct TestCase {
        pub name: String
    }
    
    #[allow(dead_code)]
    #[chain(main)]
    struct Main {
        receiver: Name, first_receiver: Name, action: Name
    }

    #[allow(dead_code)]
    impl Main {
        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver, first_receiver, action
            }
        }

        #[chain(action="sayhello")]
        pub fn say_hello(&self, name: String) {
            eosio_println!("++++++hello ", name);
        }

        #[chain(action="settest")]
        pub fn set_test(&self) {
            let table = TestCase::new_table(self.receiver);
            let data = read_action_data();
            let mut testcase = TestCase{name: "".into()};
            testcase.unpack(&data);
            table.set(&testcase, self.receiver);
        }
    }

    pub fn contract_apply(receiver: u64, first_receiver: u64, action: u64) {
        let _receiver = Name{n: receiver};
        let _first_receiver = Name{n: first_receiver};
        let _action = Name{n: action};

        if action == name!("settest").n {
            Main::new(_receiver, _first_receiver, _action).set_test();
            return;
        } else if action == name!("testsendfree").n {
            eosio_println!("+++++++testsendfree");
            return;
        }

        let table = TestCase::new_table(Name{n: receiver});
        let testcase = table.get().unwrap_or_else(|| {
            check(false, "test case not set");
            TestCase{name: "".into()}
        });

        let test_case_name = &testcase.name;
        if test_case_name == "testhello" {
            testhello::testhello::contract_apply(receiver, first_receiver, action);
        } else if test_case_name == "testasset" {
            testasset::testasset::contract_apply(receiver, first_receiver, action);
        } else if test_case_name == "testoptional" {
            testoptional::testoptional::contract_apply(receiver, first_receiver, action);
        } else if test_case_name == "testvariant" {
            testvariant::testvariant::contract_apply(receiver, first_receiver, action);
        } else if test_case_name == "testserializer" {
            testserializer::testserializer::contract_apply(receiver, first_receiver, action);
        } else if test_case_name == "testintrinsics" {
            testintrinsics::testintrinsics::contract_apply(receiver, first_receiver, action);
        } else if test_case_name == "testmi" {
            testmi::testmi::contract_apply(receiver, first_receiver, action);
        } else if test_case_name == "testmi2" {
            testmi2::testmi2::contract_apply(receiver, first_receiver, action);
        } else if test_case_name == "testcrypto" {
            testcrypto::testcrypto::contract_apply(receiver, first_receiver, action);
        } else if test_case_name ==  "testname" {
            testname::testname::contract_apply(receiver, first_receiver, action);
        } else if test_case_name == "testdestructor" {
            testdestructor::testdestructor::contract_apply(receiver, first_receiver, action);
        } else if test_case_name == "testbinaryextension" {
            testbinaryextension::testbinaryextension::contract_apply(receiver, first_receiver, action);
        } else if test_case_name == "testtransaction" {
            testtransaction::testtransaction::contract_apply(receiver, first_receiver, action);
        } else if test_case_name == "testabi" {
            testabi::testabi::contract_apply(receiver, first_receiver, action);
        } else if test_case_name == "testinlineaction" {
            testinlineaction::testinlineaction::contract_apply(receiver, first_receiver, action);
        } else if test_case_name == "testprint" {
            testprint::testprint::contract_apply(receiver, first_receiver, action);
        } else {
            check(false, "Invalid test case");
        }
    }

    #[cfg(not(feature = "std"))]
    #[no_mangle]
    pub fn apply(receiver: u64, first_receiver: u64, action: u64) {
        contract_apply(receiver, first_receiver, action);
    }

    #[cfg(feature = "std")]
    pub fn native_apply(receiver: u64, first_receiver: u64, action: u64) {
        contract_apply(receiver, first_receiver, action);
    }
}

#[cfg(test)]
mod tests {
    // use super::testmi;
    // use super::testmi2;
    use super::testoptional; 
    use super::testvariant;
    use super::testintrinsics;
    use super::testasset;
    // use super::testhello;
    use super::testcrypto;
    use super::testabi;
    use super::testserializer;
    // use super::testname;
    // use super::testtransaction;
    // use super::testdestructor;
    // use super::testbinaryextension;
    use super::testinlineaction;
    use super::testprint;

    use rust_chain::ChainTester;
    use rust_chain::serializer::Packer as _;
    use rust_chain::chaintester::{
        get_globals,
        get_test_mutex
    };
    use std::{
        fs,
        path::Path,
    };

    // use std::sync::Once;
    // static INIT: Once = Once::new();

    // use rust_chain::serializer::Packer as _;

    // fn deploy_contract(tester: &mut ChainTester, package_name: &str) {
    //     let ref wasm_file = format!("./{package_name}/target/{package_name}.wasm");
    //     let ref abi_file = format!("./{package_name}/target/{package_name}.abi");
    //     tester.deploy_contract("hello", wasm_file, abi_file).unwrap();
    // }

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

    pub fn init_test(test_case: &str) -> ChainTester {
        let mut tester = ChainTester::new();
        if std::env::var("TEST_COVERAGE").is_ok() {
            get_globals().set_debug_mode(true);
        } else {
            get_globals().set_debug_mode(false);
        }

        let _ = tester.set_native_apply("hello", Some(super::testall::native_apply));

        let ref abi_file = format!("./target/{test_case}.abi");
        let ref wasm_file = format!("./target/testall.wasm");
        tester.deploy_contract("hello", wasm_file, abi_file).unwrap();

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        let args = super::testall::TestCase{name: test_case.into()}.pack();
        tester.push_action("hello", "settest", args.into(), permissions).unwrap();
        return tester;
    }

    #[test]
    fn test_sayhello() {
        let _test_lock = get_test_mutex();
        let abi = &crate::testhello::generate_abi();
        fs::write(Path::new("./target/testhello.abi"), abi).unwrap();

        let mut tester = init_test("testhello");
        
        let args = r#"
        {
            "name": "rust"
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
    fn test_asset() {
        let _test_lock = get_test_mutex();
        let abi = testasset::generate_abi();
        fs::write(Path::new("./target/testasset.abi"), abi).unwrap();

        let mut tester = init_test("testasset");
    
        let args = r#"
        {
            "a": "1.1234 EOS"
        }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "test", args.into(), permissions).unwrap();
        tester.produce_block();

        let max_amount = (1i64 << 62) - 1;

        let bad_mini_amount = -max_amount - 1;
        let bad_max_amount = max_amount  + 1;
    
        {
            // #test Asset.unpack
            let mut args = bad_max_amount.to_le_bytes().to_vec();
            args.append(&mut "\x04EOS\x00\x00\x00\x00".as_bytes().to_vec());
            let ret = tester.push_action("hello", "test2", args.into(), permissions).unwrap_err();
            ret.check_err("Asset.unpack: bad asset amount");
            tester.produce_block();
        }

        {
            let mut args = bad_mini_amount.to_le_bytes().to_vec();
            args.append(&mut "\x04EOS\x00\x00\x00\x00".as_bytes().to_vec());
            let ret = tester.push_action("hello", "test2", args.into(), permissions).unwrap_err();
            ret.check_err("Asset.unpack: bad asset amount");
            tester.produce_block();    
        }
        {
            let args = r#"
                {
                    "error_asset": "1123A.0 EOS"
                }
            "#;
            let ret = tester.push_action("hello", "test3", args.into(), permissions).unwrap_err();
            ret.check_err("Asset.from_string: bad amount");
            tester.produce_block();    
        }

        {
            let args = r#"
                {
                    "error_asset": "11234.A EOS"
                }
            "#;
            let ret = tester.push_action("hello", "test3", args.into(), permissions).unwrap_err();
            ret.check_err("Asset.from_string: bad amount");
            tester.produce_block();    
        }

        let args = r#"
        {
            "a": {
                "contract": "hello",
                "quantity": "1.1234 EOS"
            }
        }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "test5", args.into(), permissions).unwrap();
    }

    #[test]
    fn test_optional() {
        let _test_lock = get_test_mutex();
        let abi = &testoptional::generate_abi();
        fs::write(Path::new("./target/testoptional.abi"), abi).unwrap();

        let mut tester = init_test("testoptional");
    
        let args = r#"
            {
                "a1": null,
                "a2": {"a2": {"a1": 123}},
                "a3":  {"a2": {"a1": 456}},
                "a4":  {"a2": {"a1": null}}
            }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "test", args.into(), permissions).unwrap();
        tester.produce_block();
    }


    #[test]
    fn test_variant() {
        let _test_lock = get_test_mutex();
        let abi = &testvariant::generate_abi();
        fs::write(Path::new("./target/testvariant.abi"), abi).unwrap();

        let mut tester = init_test("testvariant");

        let args = r#"
            {"v": ["uint64", 10]}
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "test", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_name() {
        let _test_lock = get_test_mutex();
        let abi = &crate::testname::generate_abi();
        fs::write(Path::new("./target/testname.abi"), abi).unwrap();

        let mut tester = init_test("testname");

        let args = r#"
        {
            "a11": "hello1",
            "a12": "hello1",
            "a21": "aaaaaaaaaaaaj",
            "a22": "aaaaaaaaaaaaj"
        }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "test", args.into(), permissions).unwrap();
        tester.produce_block();

        let args = r#"
        {
            "a": "12345123451234"
        }
        "#;
        let err = tester.push_action("hello", "test2", args.into(), permissions).unwrap_err();
        err.check_err("bad name string");
        tester.produce_block();

        let args = r#"
        {
            "a": "123451234512z"
        }
        "#;
        let err = tester.push_action("hello", "test2", args.into(), permissions).unwrap_err();
        err.check_err("bad name string");
        tester.produce_block();

        let names: [&str;3] = [
            "123451234512z",
            "123451234512A",
            "12345A1234512",
        ];

        for name in names {
            let args = format!(r#"
            {{
                "a": "{name}"
            }}"#);
            let err = tester.push_action("hello", "test2", args.into(), permissions).unwrap_err();
            err.check_err("bad name string");
            tester.produce_block();    
        }

        tester.push_action("hello", "test3", "".into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_trx() {
        let _test_lock = get_test_mutex();
        let abi = &crate::testtransaction::generate_abi();
        fs::write(Path::new("./target/testtransaction.abi"), abi).unwrap();

        let mut tester = init_test("testtransaction");

        let args = r#"
        {
        }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "test", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_binext() {
        let _test_lock = get_test_mutex();
        let abi = &crate::testbinaryextension::generate_abi();
        fs::write(Path::new("./target/testbinaryextension.abi"), abi).unwrap();

        let mut tester = init_test("testbinaryextension");

        let args = r#"
            {
                "a": 111,
                "b": 123
            }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "test", args.into(), permissions).unwrap();
        tester.produce_block();

        let args = r#"
            {
                "a": 111
            }
        "#;
        tester.push_action("hello", "test2", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_notify() {
        let _test_lock = get_test_mutex();
        get_globals().set_debug_mode(false);
        let mut tester = ChainTester::new();
        let ref wasm_file = "./testnotify/sender/target/sender.wasm";
        let ref abi_file = "./testnotify/sender/target/sender.abi";
        tester.deploy_contract("alice", wasm_file, abi_file).unwrap();

        let ref wasm_file = "./testnotify/receiver/target/receiver.wasm";
        let ref abi_file = "./testnotify/receiver/target/receiver.abi";
        tester.deploy_contract("hello", wasm_file, abi_file).unwrap();

        let args = r#"
            {
                "name": "rust"
            }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "test", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_destructor() {
        let _test_lock = get_test_mutex();
        let abi = &crate::testdestructor::generate_abi();
        fs::write(Path::new("./target/testdestructor.abi"), abi).unwrap();

        let mut tester = init_test("testdestructor");

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
    }

    #[test]
    fn test_abi() {
        let _test_lock = get_test_mutex();
        let abi = &testabi::generate_abi();
        fs::write(Path::new("./target/testabi.abi"), abi).unwrap();

        let mut tester = init_test("testabi");

        let args = r#"
            {
                "a1": true,
                "a2": -1,
                "a3": 255,
                "a4": -1,
                "a5": 65535,
                "a6": -1,
                "a7": 4294967295,
                "a8": -1,
                "a9": 18446744073709551615,
                "a10": -1,
                "a11": "0xffffffffffffffffffffffffffffffff",
                "a13": 4294967295,
                "a14": 1.1,
                "a15": 2.2,
                "a16": "0xffffffffffffffffffffffffffffffff",
                "a17": "2021-09-03T04:13:21",
                "a18": "2021-09-03T04:13:21",
                "a19": {
                    "slot": 193723200
                },
                "a20": "eosio",
                "a21": "68656c6c6f",
                "a22": "hello",
                "a23": "bbaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "a24": "bbaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "a25": "bbaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "a26": "EOS5HoPaVaPivnVHsCvpoKZMmB6gcWGV5b3vF7S6pfsgFACzufMDy",
                "a27": "SIG_K1_KbSF8BCNVA95KzR1qLmdn4VnxRoLVFQ1fZ8VV5gVdW1hLfGBdcwEc93hF7FBkWZip1tq2Ps27UZxceaR3hYwAjKL7j59q8",
                "a28": "4,EOS",
                "a29": "EOS",
                "a30": "1.0000 EOS",
                "a31": [
                    "1.0000 EOS",
                    "eosio.token"
                ]
            }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "test", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_mi() {
        let _test_lock = get_test_mutex();
        let abi = &crate::testmi::generate_abi();
        fs::write(Path::new("./target/testmi.abi"), abi).unwrap();

        let mut tester = init_test("testmi");

        let args = r#"
            {
            }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "test1", args.into(), permissions).unwrap();
        tester.produce_block();

        tester.push_action("hello", "test2", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_2mi() {
        let _test_lock = get_test_mutex();
        let abi = &crate::testmi2::generate_abi();
        fs::write(Path::new("./target/testmi2.abi"), abi).unwrap();

        let mut tester = init_test("testmi2");

        let args = r#"
            {
            }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;

        tester.push_action("hello", "test", args.into(), permissions).unwrap();
        tester.produce_block();

        tester.push_action("hello", "test2", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_crypto() {
        let _test_lock = get_test_mutex();
        let abi = &testcrypto::generate_abi();
        fs::write(Path::new("./target/testcrypto.abi"), abi).unwrap();

        let mut tester = init_test("testcrypto");

        let args = r#"
        {
            "msg": "hello,world",
            "digest": "77df263f49123356d28a4a8715d25bf5b980beeeb503cab46ea61ac9f3320eda",
            "sig": "SIG_K1_KXdabr1z4G6e2o2xmi7jPhzxH3Lj5igjR5v3q9LY7KbLWyXBZyES748bPzfM2MhQQVsLrouJzXT9YFfw1CywzMVCcNVMGH",
            "k1": "EOS87J9kj21dvniKhqd7A7QPXRz498ek3H3doXoQVPf4VnHHNtt1M",
            "r1": "PUB_R1_6FPFZqw5ahYrR9jD96yDbbDNTdKtNqRbze6oTDLntrsANgQKZu",
            "web_auth_n": "PUB_WA_8PPYTWYNkRqrveNAoX7PJWDtSqDUp3c29QGBfr6MD9EaLocaPBmsk5QAHWq4vEQt2"
        }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "test", args.into(), permissions).unwrap();
        tester.produce_block();

        tester.push_action("hello", "test2", "".into(), permissions).unwrap();
        tester.produce_block();

        let err = tester.push_action("hello", "test3", "".into(), permissions).unwrap_err();
        err.check_err("bad hex charactors");
        tester.produce_block();
    }

    #[test]
    fn test_serializer() {
        let _test_lock = get_test_mutex();
        let abi = &testserializer::generate_abi();
        fs::write(Path::new("./target/testserializer.abi"), abi).unwrap();

        let mut tester = init_test("testserializer");

        let args = r#"
        {
        }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "test", args.into(), permissions).unwrap();
        tester.produce_block();

        let err = tester.push_action("hello", "test2", "".into(), permissions).unwrap_err();
        err.check_err("invalid utf8 string");
        tester.produce_block();
    }

    #[test]
    fn test_inlineaction() {
        let _test_lock = get_test_mutex();
        let abi = &testinlineaction::generate_abi();
        fs::write(Path::new("./target/testinlineaction.abi"), abi).unwrap();

        let mut tester = init_test("testinlineaction");
        update_auth(&mut tester);

        let args = r#"
        {
            "name": "alice"
        }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "test", args.into(), permissions).unwrap();
        tester.produce_block();
    }
    
    #[test]
    fn test_intrinsics() {
        let _test_lock = get_test_mutex();
        let abi = &testintrinsics::generate_abi();
        fs::write(Path::new("./target/testintrinsics.abi"), abi).unwrap();

        let mut tester = init_test("testintrinsics");

        let args = r#"
        {
            "account": "hello",
            "is_priv": true
        }
        "#;

        let permissions = r#"
        {
            "eosio": "active"
        }
        "#;
        tester.push_action("eosio", "setpriv", args.into(), permissions).unwrap();
        tester.produce_block();

        let args = r#"
        {
            "msg": "hello,world",
            "digest": "77df263f49123356d28a4a8715d25bf5b980beeeb503cab46ea61ac9f3320eda",
            "sig": "SIG_K1_KXdabr1z4G6e2o2xmi7jPhzxH3Lj5igjR5v3q9LY7KbLWyXBZyES748bPzfM2MhQQVsLrouJzXT9YFfw1CywzMVCcNVMGH",
            "pubkey": "EOS87J9kj21dvniKhqd7A7QPXRz498ek3H3doXoQVPf4VnHHNtt1M"
        }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "test", args.into(), permissions).unwrap();
        tester.produce_block();

        let ret = tester.push_action("hello", "test2", "".into(), permissions).unwrap();
        if ret["action_traces"][0]["return_value"] != "68656c6c6f776f726c64" {//helloworld
            panic!("invalid return value");
        }
        tester.produce_block();

        let info = tester.get_info().unwrap();
        let args = testintrinsics::testintrinsics::test3{num: info["head_block_num"].as_u64().unwrap() as u32 + 1}.pack();
        tester.push_action("hello", "test3", args.into(), permissions).unwrap();

        tester.push_action("hello", "testctxfree", "".into(), permissions).unwrap();
        tester.produce_block();

        tester.push_action("hello", "testtime", "{}".into(), permissions).unwrap();
        tester.produce_block_ex(10);

        tester.push_action("hello", "testtime", "{}".into(), permissions).unwrap();
    }

    #[test]
    fn test_print() {
        let _test_lock = get_test_mutex();
        let abi = &testprint::generate_abi();
        fs::write(Path::new("./target/testprint.abi"), abi).unwrap();

        let mut tester = init_test("testprint");

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        let ret = tester.push_action("hello", "test", "".into(), permissions).unwrap();
        if ret["action_traces"][0]["console"] != "1111111111\n1.000000e+00\n1.000000000000000e+00\n6.000000000000000000e+00" {
            panic!("console output mismatch!");
        }
        // 1111111111
        // 1.000000e+00
        // 1.000000000000000e+00
        // 6.000000000000000000e+00
        tester.produce_block();
    }

    #[test]
    fn test_chain() {
        let _test_lock = get_test_mutex();
        let mut tester = ChainTester::new();
        let ret = tester.get_info().unwrap();
        println!("+++:{}", ret);
        println!("+++:{}", ret["chain_id"]);

        let ret = tester.get_account("hello").unwrap();
        println!("+++:{}", ret);
        println!("++++:{}", ret["head_block_time"]);
        tester.import_key("EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV", "5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3");
        let amount = tester.get_balance("hello");
        println!("+++++++++amount: {}", amount);

        let result = std::panic::catch_unwind(|| {
            rust_chain::check(false, "oops!");
        });
        let err = result.unwrap_err();
        println!("+++++++err:{:?}", err);        
    }
}

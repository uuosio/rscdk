#![cfg_attr(not(feature = "std"), no_std)]

mod testmi;
mod testmi2;
mod testname;
mod testdestructor;
mod testbinaryextension;
mod testtransaction;

#[eosio_chain::contract]
mod testall {

    use super::testmi::testmi::TestMI;
    use super::testmi2::testmi2::TestMI2;
    use super::testname::testname::NameTest;
    use super::testdestructor::testdestructor::TestDestructor;
    use super::testbinaryextension::testbinaryextension::TestBinaryExtension;
    use super::testtransaction::testtransaction::TestTransaction;

    use eosio_chain::{
        Name,
        Asset,
        BinaryExtension,
    };

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
        #[chain(action="mitest1")]
        pub fn mitest1(&self) {
            let test = TestMI::new(self.receiver, self.first_receiver, self.action);
            test.test1();    
        }

        #[chain(action="mitest2")]
        pub fn mitest2(&self) {
            let test = TestMI::new(self.receiver, self.first_receiver, self.action);
            test.test2();    
        }

        #[chain(action="mi2test")]
        pub fn mi2test(&self) {
            let test = TestMI2::new(self.receiver, self.first_receiver, self.action);
            test.test();    
        }

        /// name test
        #[chain(action="nametest")]
        pub fn nametest(&self, a11: String, a12: Name, a21: String, a22: Name) {
            let test = NameTest::new(self.receiver, self.first_receiver, self.action);
            test.test(a11, a12, a21, a22);
        }

        #[chain(action="destructtest")]
        pub fn destructor_test(&self) {
            let mut test = TestDestructor::new(self.receiver, self.first_receiver, self.action);
            test.inc_count();
        }

        #[chain(action="binexttest")]
        pub fn binary_extension_test(&self, a: BinaryExtension<u64>) {
            let test = TestBinaryExtension::new(self.receiver, self.first_receiver, self.action);
            test.test(a);
        }

        #[chain(action="trxtest")]
        pub fn transaction_test(&self) {
            let test = TestTransaction::new(self.receiver, self.first_receiver, self.action);
            test.test();
        }
    }
}

#[cfg(feature="std")]
#[no_mangle]
fn native_apply(_receiver: u64, _first_receiver: u64, _action: u64) {
    testintrinsics::testintrinsics::native_apply(_receiver, _first_receiver, _action);
    // testserializer::test::native_apply(_receiver, _first_receiver, _action);
}

#[cfg(test)]
mod tests {

    use eosio_chain::ChainTester;
    use eosio_chain::serializer::Packer as _;
    use eosio_chain::eosio_chaintester;
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
    fn test_sayhello() {
        let abi = &hello::generate_abi();
        fs::write(Path::new("./hello/target/hello.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        // tester.enable_debug_contract("hello", true).unwrap();

        deploy_contract(&mut tester, "hello");
        update_auth(&mut tester);
    
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
        let abi = &testasset::generate_abi();
        fs::write(Path::new("./testasset/target/testasset.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        // tester.enable_debug_contract("hello", true).unwrap();

        deploy_contract(&mut tester, "testasset");
        update_auth(&mut tester);
    
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
            let mut args = r#"
                {
                    "error_asset": "1123A.0 EOS"
                }
            "#;
            let ret = tester.push_action("hello", "test3", args.into(), permissions).unwrap_err();
            ret.check_err("Asset.from_string: bad amount");
            tester.produce_block();    
        }

        {
            let mut args = r#"
                {
                    "error_asset": "11234.A EOS"
                }
            "#;
            let ret = tester.push_action("hello", "test3", args.into(), permissions).unwrap_err();
            ret.check_err("Asset.from_string: bad amount");
            tester.produce_block();    
        }    
    }

    #[test]
    fn test_optional() {
        let abi = &testoptional::generate_abi();
        fs::write(Path::new("./testoptional/target/testoptional.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        // tester.enable_debug_contract("hello", true).unwrap();

        deploy_contract(&mut tester, "testoptional");
    
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
        let abi = &testvariant::generate_abi();
        fs::write(Path::new("./testvariant/target/testvariant.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        // tester.enable_debug_contract("hello", true).unwrap();

        deploy_contract(&mut tester, "testvariant");
    
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
        let abi = &crate::testall::generate_abi();
        fs::write(Path::new("./target/testall.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        let ref wasm_file = format!("./target/testall.wasm");
        let ref abi_file = format!("./target/testall.abi");
        tester.deploy_contract("hello", wasm_file, abi_file).unwrap();

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
        tester.push_action("hello", "nametest", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_trx() {
        let abi = &crate::testall::generate_abi();
        fs::write(Path::new("./target/testall.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        let ref wasm_file = format!("./target/testall.wasm");
        let ref abi_file = format!("./target/testall.abi");
        tester.deploy_contract("hello", wasm_file, abi_file).unwrap();

        let args = r#"
        {
        }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "trxtest", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_binext() {
        let abi = &crate::testall::generate_abi();
        fs::write(Path::new("./target/testall.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        let ref wasm_file = format!("./target/testall.wasm");
        let ref abi_file = format!("./target/testall.abi");
        tester.deploy_contract("hello", wasm_file, abi_file).unwrap();

        let args = r#"
            {
                "a": 123
            }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "binexttest", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_notify() {
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
        let abi = &crate::testall::generate_abi();
        fs::write(Path::new("./target/testall.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        let ref wasm_file = format!("./target/testall.wasm");
        let ref abi_file = format!("./target/testall.abi");
        tester.deploy_contract("hello", wasm_file, abi_file).unwrap();

        let args = r#"
            {
            }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "destructtest", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_abi() {
        let abi = &testabi::generate_abi();
        fs::write(Path::new("./testabi/target/testabi.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        // tester.enable_debug_contract("hello", true).unwrap();

        deploy_contract(&mut tester, "testabi");
    
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
        let abi = &crate::testall::generate_abi();
        fs::write(Path::new("./target/testall.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        let ref wasm_file = format!("./target/testall.wasm");
        let ref abi_file = format!("./target/testall.abi");
        tester.deploy_contract("hello", wasm_file, abi_file).unwrap();

        let args = r#"
            {
            }
        "#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "mitest1", args.into(), permissions).unwrap();
        tester.produce_block();

        tester.push_action("hello", "mitest2", args.into(), permissions).unwrap();
        tester.produce_block();

        tester = ChainTester::new();
        tester.push_action("hello", "mi2test", args.into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_crypto() {
        let abi = &testcrypto::generate_abi();
        fs::write(Path::new("./testcrypto/target/testcrypto.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        // tester.enable_debug_contract("hello", true).unwrap();

        deploy_contract(&mut tester, "testcrypto");

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
    }

    #[test]
    fn test_serializer() {
        let abi = &testintrinsics::generate_abi();
        fs::write(Path::new("./testserializer/target/testserializer.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        tester.enable_debug_contract("hello", true).unwrap();

        deploy_contract(&mut tester, "testserializer");

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
    fn test_intrinsics() {
        let abi = &testintrinsics::generate_abi();
        fs::write(Path::new("./testintrinsics/target/testintrinsics.abi"), abi).unwrap();

        let mut tester = ChainTester::new();
        tester.enable_debug_contract("hello", true).unwrap();

        deploy_contract(&mut tester, "testintrinsics");

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
    }
}

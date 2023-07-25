#![cfg_attr(not(feature = "std"), no_std)]

extern "C" {
    pub fn prints(cstr: *const u8);
}

#[cfg(not(feature = "std"))]
#[no_mangle]
pub fn apply(receiver: u64, first_receiver: u64, action: u64) {
    unsafe {
        prints("hello, world".as_ptr() as *const u8);
    }
}

#[cfg(feature="std")]
#[no_mangle]
fn native_apply(receiver: u64, first_receiver: u64, action: u64) {
}

#[cfg(all(not(feature = "std"), target_arch = "wasm32"))]
#[allow(unused_variables)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable();
}

#[cfg(feature = "std")]
pub fn generate_abi() -> String {
    return r#"
    {
        "version": "eosio::abi/1.1",
        "types": [],
        "structs": [],
        "actions": [],
        "tables": [],
        "variants": [],
        "abi_extensions": [],
        "error_messages": [],
        "ricardian_clauses": [],
        "action_results": []
      }
    "#.into();
}

#[cfg(test)]
mod tests {

    use chaintester::ChainTester;

    fn deploy_contract(tester: &mut ChainTester) {
        let ref wasm_file = format!("./target/tests.wasm");
        let ref abi_file = format!("./target/tests.abi");
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
    fn test_inc() {
        let mut tester = ChainTester::new();
        //uncomment the following line to enable contract debugging.
        // tester.enable_debug_contract("hello", true).unwrap();
        let key = tester.create_key().unwrap();
        println!("+++++++++++private key:{}", key["private"]);
        let pub_key = key["public"].as_str().unwrap();
        tester.create_account("hello", "helloworld33", pub_key, pub_key, 10*1024*1024, 100000, 100000).unwrap();
        let ret = tester.get_account("helloworld33").unwrap();
        println!("+++++++++++++{}", ret["head_block_time"]);

        deploy_contract(&mut tester);
        update_auth(&mut tester);
    
        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "inc", "".into(), permissions).unwrap();
        tester.produce_block();

    }
}

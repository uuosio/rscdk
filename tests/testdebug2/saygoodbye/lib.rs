#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
pub mod saygoodbye {
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

    impl Hello {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action="saygoodbye")]
        pub fn say_goodbye(&self, name: String) {
            eosio_println!("++++hello:", name);
        }
    }
}

#[cfg(test)]
mod tests {
    use eosio_chain::ChainTester;
    use eosio_chain::serializer::Packer;
    use crate::saygoodbye;

    #[no_mangle]
    fn native_apply(receiver: u64, first_receiver: u64, action: u64) {
        saygoodbye::native_apply(receiver, first_receiver, action);
    }

    fn deploy_contract(tester: &mut ChainTester) {
        let cur_dir: &str = &std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
        println!("{cur_dir}");
        tester.deploy_contract("hello", &format!("{cur_dir}/testdebug2/saygoodbye/target/saygoodbye.wasm"), &format!("{cur_dir}/testdebug2/saygoodbye/target/saygoodbye.abi")).unwrap();
    }

    #[test]
    fn test_saygoodbye() {
        let exe = std::env::current_exe();
        println!("defined in file: {exe:?}");
    
        let mut tester = ChainTester::new();
        tester.enable_debug_contract("hello", true).unwrap();

        deploy_contract(&mut tester);

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;

        let args = saygoodbye::saygoodbye{name: "rust".into()};
        tester.push_action("hello", "saygoodbye", args.pack().into(), permissions).unwrap();
        tester.produce_block();
    }

}

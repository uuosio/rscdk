#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
mod hello {
    use eosio_chain::{
        Name,
        name,
        eosio_println,
        print::{
            prints,
        },
        action::{
            Action,
            PermissionLevel,
        },
        ACTIVE,
    };

    #[chain(main)]
    #[allow(dead_code)]
    pub struct Hello {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    #[chain(packer)]
    struct SayGoodbye {
        name: String,
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
            for i in 0..1 {
                eosio_println!("++++hello:", name);
            }
            // return;
            let perms: Vec<PermissionLevel> = vec![PermissionLevel{actor: name!("hello"), permission: ACTIVE}];
            let say_goodbye = SayGoodbye{name: name};
            let action = Action::new(name!("hello"), name!("saygoodbye"), &perms, &say_goodbye);
            action.send();
        }

        #[chain(action="saygoodbye")]
        pub fn say_goodbye(&self, name: String) {
            eosio_println!("++++hello:", name);
        }

    }

    // #[no_mangle]
    // fn apply(receiver: u64, first_receiver: u64, action: u64) {
    //     prints("hello, debugger!!!");
    //     return;
    //     use eosio_chain::eosio_chaintester;
    //     use eosio_chain::eosio_chaintester::chaintester::TApplySyncClient;
    //     let mut client = eosio_chaintester::new_vm_api_client("127.0.0.1", 9092).unwrap();
    //     client.prints(String::from("hello, debugger!")).unwrap();            
    // }

    // #[no_mangle]
    // fn native_apply(receiver: u64, first_receiver: u64, action: u64) {
    //     apply(receiver, first_receiver, action);
    // }
}

#[cfg(test)]
mod tests {
    use eosio_chain::ChainTester;
    #[test]
    fn test_prints() {
        let mut tester = ChainTester::new();
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
        tester.push_action("hello", "sayhello", args, permissions)
    }
}

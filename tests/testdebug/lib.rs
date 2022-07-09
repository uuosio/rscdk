#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
mod hello {
    use eosio_chain::{
        Name,
        eosio_println,
        print::{
            prints,
        }
    };

    #[chain(main)]
    #[allow(dead_code)]
    pub struct Hello {
        receiver: Name,
        first_receiver: Name,
        action: Name,
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
            prints("hello, debugger!!!!!!!!!");
            // eosio_println!("++++hello", name);
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
        tester.push_action("hello");
    }
}

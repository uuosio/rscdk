#![cfg_attr(not(feature = "std"), no_std)]

#[rust_chain::contract]
pub mod testhello {
    use rust_chain::{
        Name,
        eosio_println,
    };

    #[chain(sub)]
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
            eosio_println!("++++hello", name);
        }
    }
}

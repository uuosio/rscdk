#![cfg_attr(not(feature = "std"), no_std)]

#[rust_chain::contract]
mod helloworld {
    use rust_chain::{
        Name,
        eosio_println,
    };

    #[chain(main)]
    #[allow(dead_code)]
    pub struct Contract {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    impl Contract {
        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action = "sayhello")]
        pub fn say_hello(&self) {
            eosio_println!("hello,world!");
        }
    }
}

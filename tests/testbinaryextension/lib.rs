#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
mod testbinaryextension {
    use eosio_chain::{
        Name,
        BinaryExtension,
        
        check,
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

        #[chain(action = "test")]
        pub fn test(&self, a: BinaryExtension<u64>) {
            check(a.value().is_some(), "bad value");
            check(*a.value().unwrap() == 123, "bad value");
        }
    }
}

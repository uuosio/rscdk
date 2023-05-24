#![cfg_attr(not(feature = "std"), no_std)]

use rust_chain as chain;

#[chain::contract]
mod hello {
    use ::rust_chain::{
        Name,
        require_recipient,
        chain_println,
    };

    #[chain(packer)]
    pub struct SayHello {
        pub name: String
    }

    #[chain(main)]
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

        #[chain(action="test", notify)]
        pub fn test(&self, name: String) {
            chain_println!("++++++++receiver:", name);
        }
    }
}

#![cfg_attr(not(feature = "std"), no_std)]

use rust_chain as chain;

#[chain::contract]
mod hello {
    use ::rust_chain::{
        Name,
        chain_println,
        require_recipient,
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

        #[chain(action="test")]
        pub fn test(&self, name: String) {
            require_recipient(Name::new("hello"));
            chain_println!("++++++++sender:", name);
        }
    }
}

#![cfg_attr(not(feature = "std"), no_std)]

use eosio_chain as chain;

#[chain::contract]
mod hello {

    #[chain(table="mydata")]
    pub struct MyData {
        #[chain(primary)]
        count: u64
    }

    #[chain(main)]
    pub struct Hello {
        receiver: u64,
        first_receiver: u64,
        action: u64,
    }

    impl Hello {

        pub fn new(receiver: u64, first_receiver: u64, action: u64) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action="sayhello")]
        pub fn say_hello(&self) {
            let mut v = vec![1, 2, 3, 4];
            eosio_println!("hello", v);
        }
    }
}

#![cfg_attr(not(feature = "std"), no_std)]

use eosio_chain as chain;

#[chain::contract]
mod hello {
    pub struct MyData3 {
        count: u64
    }

    pub struct MyData2 {
        count: u64,
        mydata: MyData3,
    }

    #[chain(table="mydata")]
    pub struct MyData {
        #[chain(primary)]
        a1: u64,
        #[chain(Idx64)]
        a2: u64,
        mydata: MyData2,
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

        #[chain(action="sayhello")]
        pub fn say_hello(&self) {
            let mut v = vec![1, 2, 3, 4];
            eosio_println!("hello", v);
        }

        #[chain(action="test")]
        pub fn test(&self, a1: String, a2: Option<u8>) {
        }
    }
}

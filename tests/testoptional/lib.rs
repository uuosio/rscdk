#![cfg_attr(not(feature = "std"), no_std)]

use rust_chain as chain;

#[chain::contract]
mod test {
    use rust_chain::{
        Name,
        check,
        eosio_println,
    };

    #[chain(main)]
    #[allow(dead_code)]
    pub struct TestSerialzier {
        receiver: Name,
        first_receiver: Name,
        action: Name,
        value: u32,
    }

    #[chain(packer)]
    pub struct A1 {
        a1: Option<u64>
    }

    pub struct A2 {
        a2: Option<A1>
    }

    impl TestSerialzier {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
                value: 0,
            }
        }

        #[chain(action="test")]
        pub fn test(&self, a1: Option<u64>, a2: A2, a3: Option<A2>, a4: A2) {
            check(a1 == None, "bad value a1");
            if let Some(v1) = a2.a2 {
                if let Some(v2) = v1.a1 {
                    check(v2 == 123, "bad value a2");
                } else {
                    check(false, "bad value a2")
                }
            } else {
                check(false, "bad value a2")
            }

            if let Some(v1) = a3 {
                if let Some(v2) = v1.a2 {
                    if let Some(v3) = v2.a1 {
                        check(v3 == 456, "bad value a3");
                    } else {
                        check(false, "bad value a3");
                    }
                } else {
                    check(false, "bad value a3");
                }
            } else {
                check(false, "bad value a3");
            }

            if let Some(v1) = a4.a2 {
                check(v1.a1 == None, "bad value a4");
            } else {
                check(false, "bad value a4");
            }
            eosio_println!("+++++optional test done!");
        }
    }
}

#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
pub mod testname {
    use eosio_chain::{
        Name,
        name,
        check,
        eosio_println,
    };

    pub struct NameTest {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    impl NameTest {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action="test")]
        pub fn test(&self, a11: String, a12: Name, a21: String, a22: Name) {
            check(a12 == Name::from_str(&a11), "bad value 1");
            check(a22 == Name::from_str(&a21), "bad value 2");

            check(a12 == name!("hello1"), "bad value 1");
            check(a22 == name!("aaaaaaaaaaaaj"), "bad value 2");

            let n1 = name!("hello1");
            let n2 = name!("hello2");
            let n3 = name!("hello3");
            let n4 = name!("hello4");
            let n5 = name!("hello5");
            let n6 = Name::new("hello11");
            let n7 = Name::new("hello12");
            let n8 = Name::new("hello13");
            let n9 = name!("hello14");

            eosio_println!("hello", n1, n2, n3, n4, n5, n6, n7, n8, n9);
        }
    }
}


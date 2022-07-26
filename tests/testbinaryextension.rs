#[eosio_chain::contract]
pub mod testbinaryextension {
    use eosio_chain::{
        Name,
        BinaryExtension,
        
        check,
        eosio_println,
    };

    #[allow(dead_code)]
    pub struct TestBinaryExtension {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    impl TestBinaryExtension {
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
            eosio_println!("test BinaryExtension done!");
        }
    }
}

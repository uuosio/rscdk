#[rust_chain::contract]
pub mod testbinaryextension {
    use rust_chain::{
        Name,
        BinaryExtension,
        
        check,
        eosio_println,
    };

    #[chain(sub)]
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
        pub fn test(&self, a: u64, b: BinaryExtension<u64>) {
            check(a == 111, "a == 111");
            check(b.value().is_some(), "bad value");
            check(*b.value().unwrap() == 123, "bad value");
            eosio_println!("test BinaryExtension done!");

            let mut _b = BinaryExtension::<u64>::new(Some(1));
            check(_b.size() == 8, "b.size() == 8");
            _b.unpack(&b.pack());
            check(*_b.value().unwrap() == 123, "*_b.value().unwrap() == 123");
        }

        #[chain(action = "test2")]
        pub fn test2(&self, a: u64, b: BinaryExtension<u64>) {
            check(a == 111, "a == 111");
            check(b.value().is_none(), "b.value().is_none()");

            let mut _b = BinaryExtension::<u64>::new(Some(1));
            check(_b.size() == 8, "b.size() == 8");
            _b.unpack(&b.pack());
            check(_b.value().is_none(), "_b.value().is_none()");
            eosio_println!("test2 BinaryExtension done!");
        }
    }
}

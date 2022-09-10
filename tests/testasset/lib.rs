#![cfg_attr(not(feature = "std"), no_std)]

#[rust_chain::contract]
pub mod test {
    use rust_chain::{
        Asset,
        Name,
        Symbol,

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
        pub fn test(&self, a: Asset) {
            check(a == Asset::from_string("1.1234 EOS"), "1: bad asset!");
            check(a.symbol() == Symbol::new("EOS", 4u8), "2: bad symbol!");
            check(a.symbol() == Asset::new(11234, Symbol::new("EOS", 4u8)).symbol(), "3: bad symbol!");
            check(a == Asset::new(11234, Symbol::new("EOS", 4u8)), "4: bad asset string!");
            check(a.to_string() == "1.1234 EOS", "5: bad asset string!");
            eosio_println!("Done!");
        }

        #[chain(action="test2")]
        pub fn test2(&self, invalid_asset: Asset) {
            //check(!invalid_asset.is_valid(), "test failed");
        }

        #[chain(action="test3")]
        pub fn test3(&self, error_asset: String) {
            Asset::from_string(&error_asset);
        }
    }
}

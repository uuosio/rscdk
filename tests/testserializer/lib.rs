#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
pub mod test {
    use eosio_chain::{
        Asset,
        Name,
        Symbol,

        Encoder,
        Decoder,

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
        pub fn test(&self) {
            let mut enc = Encoder::new(10);
            enc.pack_number(10u8);

            let v = enc.get_bytes();
            let mut dec = Decoder::new(&v);
            let mut n = 0u8;
            dec.unpack(&mut n);
            check(n == 10, "bad value!");
        }
    }
}

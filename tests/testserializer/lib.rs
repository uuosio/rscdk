#![cfg_attr(not(feature = "std"), no_std)]

#[rust_chain::contract]
pub mod test {
    use rust_chain::{
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
            enc.pack_number(10i8);
            enc.pack_number(10u16);
            enc.pack_number(10i16);
            enc.pack_number(10u32);
            enc.pack_number(10i32);

            let v1 = vec![1u8;256];
            enc.pack(&v1);

            let raw = enc.get_bytes();
            let mut dec = Decoder::new(&raw);

            let n = dec.unpack_number::<u8>();
            check(n == 10, "bad value");

            let n = dec.unpack_number::<i8>();
            check(n == 10, "bad value");

            let n = dec.unpack_number::<u16>();
            check(n == 10, "bad value");

            let n = dec.unpack_number::<i16>();
            check(n == 10, "bad value");

            let n = dec.unpack_number::<u32>();
            check(n == 10, "bad value");

            let n = dec.unpack_number::<i32>();
            check(n == 10, "bad value");
            
            let mut v2: Vec<u8> = Vec::new();
            dec.unpack(&mut v2);
            check(v1 == v2, "v.len() == v2.len()");
        }
    }
}

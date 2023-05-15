#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(warnings))]

#[rust_chain::contract]
pub mod testprint {
    use rust_chain::{
        Float128,
        Name,
        print::{
            // Printable as _,
            prints,
        },
        // eosio_println,
    };

    #[chain(sub)]
    #[allow(dead_code)]
    pub struct TestPrint {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    impl TestPrint {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action="test")]
        pub fn test(&self) {
            1i8.print();
            1u8.print();
            1i16.print();
            1u16.print();
            1i32.print();
            1u32.print();
            1i64.print();
            1u64.print();
            1i128.print();
            1u128.print();

            prints("\n");
            1.0f32.print();

            prints("\n");
            1.0f64.print();

            prints("\n");
            Float128::new([0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x80,0x01,0x40]).print();
        }
    }
}

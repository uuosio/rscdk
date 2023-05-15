#[rust_chain::contract]
pub mod testvariant {
    use rust_chain::{
        Name,
        eosio_println,
    };

    #[chain(variant)]
    pub enum MyVariant {
        A(u32),
        B(u64),
    }

    #[chain(sub)]
    #[allow(dead_code)]
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

        #[chain(action="test")]
        pub fn test(&self, v: MyVariant) {
            if let MyVariant::B(b) = v {
                eosio_println!("hello", b);
            }
        }
    }
}

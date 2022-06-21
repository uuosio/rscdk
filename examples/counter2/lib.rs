#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
mod token {
    use eosio_chain::{
        Name,
        eosio_println,
    };

    #[chain(table="counter", singleton)]
    pub struct Counter {
        count: u64
    }

    #[chain(main)]
    pub struct Contract {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    impl Contract {
        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action = "inc")]
        pub fn inc_count(&self) {
            let db = Counter::new_mi(self.receiver, self.receiver);
            let mut value = db.get().unwrap_or(Counter{count: 1});
            eosio_println!("+++++count2:", value.count);
            value.count += 1;
            db.set(&value, self.receiver);
        }
    }
}

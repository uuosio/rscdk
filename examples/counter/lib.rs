#![cfg_attr(not(feature = "std"), no_std)]

#[rust_chain::contract]
mod token {
    use rust_chain::{
        Name,
        eosio_println,
    };

    #[chain(table="counter")]
    pub struct Counter {
        #[chain(primary)]
        key: u64,
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
            let db = Counter::new_table(self.receiver);
            let it = db.find(1u64);
            if let Some(mut value) = db.get(&it) {
                value.count += 1;
                db.update(&it, &value, self.receiver);
                eosio_println!("count is", value.count);
            } else {
                db.store(&Counter{key: 1, count: 1}, self.receiver);
                eosio_println!("count is", 1);
            }
        }
    }
}

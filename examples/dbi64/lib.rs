#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
mod dbi64 {
    use eosio_chain::{
        name,
        Name,
        db::{
            DBI64,
        },
        eosio_println,
    };

    #[chain(table="counter")]
    pub struct Counter {
        #[chain(primary)]
        key: u64,
        count: u64,
    }

    #[chain(main)]
    #[allow(dead_code)]
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
            let db = DBI64::<Counter>::new(self.receiver, self.receiver, name!("counter"));
            let it = db.find(1);
            let payer = self.receiver;
            if let Some(mut value) = it.get_value() {
                value.count += 1;
                db.update(&it, &value, payer);
                eosio_println!("+++count:", value.count);
            } else {
                let value = Counter{key: 1, count: 1};
                db.store(1, &value, payer);
                eosio_println!("+++count:", value.count);
            }
        }
    }
}

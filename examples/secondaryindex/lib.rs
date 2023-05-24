#![cfg_attr(not(feature = "std"), no_std)]

#[rust_chain::contract]
mod secondaryindex {
    use rust_chain::{
        Name,
        chain_println,
    };

    #[chain(table="counter")]
    pub struct MyData {
        #[chain(primary)]
        key: u64,
        #[chain(secondary)]
        value: u64,
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

        #[chain(action = "test1")]
        pub fn test1(&self, key: u64, value: u64) {
            let db = MyData::new_table(self.receiver);
            let it = db.find(key);
            if let Some(mut data) = it.get_value() {
                data.value = value;
                db.update(&it, &data, self.receiver);
                chain_println!("key is:", data.key, "value is:", data.value);
            } else {
                let data = &MyData{key: key, value: value};
                db.store(&data, self.receiver);
                chain_println!("key is:", data.key, "value is:", data.value);
            }
        }

        #[chain(action = "test2")]
        pub fn test2(&self, value: u64) {
            chain_println!("+++value:", value);
            let db = MyData::new_table(self.receiver);
            let idx = db.get_idx_by_value();
            let (it_secondary, mut secondary_value) = idx.lower_bound(value);
            if it_secondary.is_ok() {
                chain_println!("++++primary value", it_secondary.primary, "secondary value:", secondary_value);
                // update secondary value
                let payer = self.receiver;
                secondary_value += 1;
                db.idx_update(&it_secondary, secondary_value.into(), payer);
            }
        }
    }
}

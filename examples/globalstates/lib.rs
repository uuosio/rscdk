#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
mod test {
    use eosio_chain::{
        Name,
        eosio_println,
    };

    #[chain(table="states", singleton)]
    pub struct States {
        count: u64
    }

    #[chain(main)]
    #[allow(dead_code)]
    pub struct Contract {
        receiver: Name,
        first_receiver: Name,
        action: Name,
        states: States,
        states_db: Box<StatesMultiIndex>,
    }

    impl Contract {
        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            let states_db = States::new_table(receiver);
            let states = states_db.get().unwrap_or(States{count: 1});
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
                states,
                states_db,
            }
        }

        #[chain(action = "inc")]
        pub fn inc_count(&mut self) {
            self.states.count += 1;
            eosio_println!("++++count:", self.states.count);
        }
    }

    impl Drop for Contract {
        fn drop(&mut self) {
            self.states_db.set(&self.states, self.receiver);
        }
    }
}

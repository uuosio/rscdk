#[rust_chain::contract]
pub mod testdestructor {
    use rust_chain::{
        Name,
        chain_println,
    };

    #[chain(table="states", singleton)]
    pub struct States {
        count: u64
    }

    #[chain(sub)]
    #[allow(dead_code)]
    pub struct TestDestructor {
        receiver: Name,
        first_receiver: Name,
        action: Name,
        states: States,
        states_db: Box<StatesMultiIndex>,
    }

    impl TestDestructor {
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
            chain_println!("++++count:", self.states.count);
        }
    }

    impl Drop for TestDestructor {
        fn drop(&mut self) {
            self.states_db.set(&self.states, self.receiver);
            chain_println!("++++drop");
        }
    }
}

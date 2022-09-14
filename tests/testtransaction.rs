#[rust_chain::contract]
pub mod testtransaction {
    use rust_chain::{
        Name,
        name,
        check,
        eosio_println,
        read_transaction,
        Transaction,
    };

    #[chain(sub)]
    #[allow(dead_code)]
    pub struct TestTransaction {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    impl TestTransaction {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action="test")]
        pub fn test(&self) {
            let tx = read_transaction();
            eosio_println!("++++test:", tx.expiration().utc_seconds());
            let actions = tx.actions();
            check(actions.len() == 1, "bad actions");
            check(actions[0].account == name!("hello"), "bad action account");
            check(actions[0].name == name!("test"), "bad action name");
        }
    }
}


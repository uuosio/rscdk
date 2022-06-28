#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
mod hello {
    use eosio_chain::{
        Name,
        name,
        check,
        eosio_println,
        read_transaction,
        Transaction,
    };

    #[chain(main)]
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
        pub fn test(&self) {
            let raw_tx = read_transaction();
            let mut tx = Transaction::default();
            tx.unpack(&raw_tx);
            eosio_println!("++++test:", tx.expiration().utc_seconds());
            let actions = tx.actions();
            check(actions.len() == 1, "bad actions");
            check(actions[0].account == name!("hello"), "bad action account");
            check(actions[0].name == name!("test"), "bad action name");
        }
    }
}


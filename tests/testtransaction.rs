#[rust_chain::contract]
pub mod testtransaction {
    use rust_chain::{
        Name,
        name,
        check,
        eosio_println,
        read_transaction,

        Asset,
        Symbol,
        Action,
        PermissionLevel,
        Transaction,
    };

    #[chain(sub)]
    #[allow(dead_code)]
    pub struct TestTransaction {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    #[chain(packer)]
    pub struct Transfer {
        from: Name,
        to: Name,
        quantity: Asset,
        memo: String
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
            eosio_println!("++++test:", tx.expiration().seconds());
            let actions = tx.actions();
            check(actions.len() == 1, "bad actions");
            check(actions[0].account == name!("hello"), "bad action account");
            check(actions[0].name == name!("test"), "bad action name");
        }

        #[chain(action="test2")]
        pub fn test2(&self) {
            let mut tx = Transaction::new(1, 0);
            // pub fn new(account: Name, name: Name, authorization: &Vec<PermissionLevel>, data: &dyn Packer) -> Self {
            let perm = vec![PermissionLevel{actor: name!("hello"), permission: name!{"active"}}];
            let transfer = Transfer {
                from: self.receiver,
                to: name!("eosio"),
                quantity: Asset::new(10000, Symbol::new("EOS", 4)),
                memo: "hello".into()
            };
            let action = Action::new(name!("eosio.token"), name!("transfer"), perm, &transfer);
            tx.add_action(action);
            tx.send(self.receiver, 1, true);
        }
    }
}


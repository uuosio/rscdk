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
        TransactionExtension,
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
            let mut tx2 = Transaction::default();
            tx2.unpack(&Encoder::pack(&tx));
            check(tx == tx2, "tx == tx2");

            eosio_println!("++++test:", tx.expiration().seconds());

            eosio_println!(tx.ref_block_num(), tx.ref_block_prefix(), tx.max_net_usage_words(), tx.max_cpu_usage_ms(), tx.delay_sec());

            let actions = tx.actions();
            check(actions.len() == 1, "bad actions");
            check(actions[0].account == name!("hello"), "bad action account");
            check(actions[0].name == name!("test"), "bad action name");

            let mut ext = TransactionExtension::default();
            let mut ext2 = TransactionExtension::default();
            ext.ty = 1;
            ext.data = vec![1, 2, 3, 4, 5];
            ext2.unpack(&Encoder::pack(&ext));
            check(ext == ext2, "ext == ext2");
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


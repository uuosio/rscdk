#![cfg_attr(not(feature = "std"), no_std)]

#[rust_chain::contract]
pub mod testinlineaction {
    use rust_chain::action::{
        PermissionLevel,
        Action,
    };

    use rust_chain::{
        Name,
        name,
        eosio_println,
    };

    #[chain(packer)]
    pub struct SayHello {
        pub name: String
    }

    #[allow(dead_code)]
    #[chain(sub)]
    pub struct TestInlineAction {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    impl TestInlineAction {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action="test")]
        pub fn test(&self, name: String) {
            eosio_println!("send sayhello action", &name);
            let say_hello = SayHello{name: name};
            let perms: Vec<PermissionLevel> = vec![PermissionLevel::new(name!("hello"), name!("active"))];
            let action = Action::new(name!("hello"), name!("sayhello"), perms, &say_hello);
            action.send();
        }

        #[chain(action="sayhello")]
        pub fn sayhello(&self, name: String) {
            eosio_println!("hello", name);
        }

        #[chain(action="test2")]
        pub fn test2(&self, _perm: PermissionLevel, _a: Action) {

        }
    }
}

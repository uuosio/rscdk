#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
mod inline_action_example {
    use eosio_chain::{
        Name,
        action::{
            Action,
            PermissionLevel,    
        },
        name,
        ACTIVE,
        eosio_println,
    };

    #[chain(packer)]
    struct SayGoodbye {
        name: String
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

        #[chain(action = "sayhello")]
        pub fn say_hello(&self, name: String) {
            eosio_println!("hello", name);
            let perms: Vec<PermissionLevel> = vec![PermissionLevel{actor: name!("hello"), permission: ACTIVE}];
            let say_goodbye = SayGoodbye{name: name};
            let action = Action::new(name!("hello"), name!("saygoodbye"), &perms, &say_goodbye);
            action.send();
        }

        #[chain(action = "saygoodbye")]
        pub fn say_goodbye(&self, name: String) {
            eosio_println!("goodbye", name);
        }
    }
}

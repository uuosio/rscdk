#![cfg_attr(not(feature = "std"), no_std)]

#[rust_chain::contract]
mod inline_action_example {
    use rust_chain::{
        Name,
        action::{
            Action,
            PermissionLevel,    
        },
        name,
        ACTIVE,
        chain_println,
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
            chain_println!("hello", name);
            let perm = PermissionLevel{actor: name!("hello"), permission: ACTIVE};
            let say_goodbye = SayGoodbye{name: name};
            let action = Action::new(name!("hello"), name!("saygoodbye"), perm, &say_goodbye);
            action.send();
        }

        #[chain(action = "saygoodbye")]
        pub fn say_goodbye(&self, name: String) {
            chain_println!("goodbye", name);
        }
    }
}

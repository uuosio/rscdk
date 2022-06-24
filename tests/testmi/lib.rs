#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
mod hello {
    use eosio_chain::{
        Name,
        eosio_println,
    };

    #[chain(table="mydata2")]
    pub struct MyData2 {
        #[chain(primary)]
        a2: u64,
    }

    #[chain(table="mydata1")]
    pub struct MyData1 {
        #[chain(primary)]
        a1: u64,
    }

    #[chain(main)]
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
            let it1: eosio_chain::db::Iterator<MyData1>;
            {
                let db1 = MyData1::new_mi(self.receiver, self.receiver);
                let db2 = MyData2::new_mi(self.receiver, self.receiver);
    
                it1 = db1.find(1u64);
                let it2 = db2.find(1u64);
                if it1.is_ok() {
                    eosio_println!("+++it1:", db1.end().get_i());
                } else {
                    db1.store(&MyData1{a1:1}, self.receiver);
                }
    
                if it2.is_ok() {
                    eosio_println!("+++it2:", db2.end().get_i());
                } else {
                    db2.store(&MyData2{a2:1}, self.receiver);
                }                
                it1.get_value();
            }
        }
    }
}

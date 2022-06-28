#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
mod test {
    use eosio_chain::{
        Name,
        Asset,
        Float128,
        Uint256,
        db::{
            SecondaryType,
            SecondaryValue,
            SecondaryIterator,
            MultiIndexValue,
            PrimaryValueInterface,
            SecondaryValueInterface,
            AsAny,
        },

        mi_not_generic::{
            DBI64,
            MultiIndex,
        },

        name,
        check,
        eosio_println,
    };

    #[chain(main)]
    #[allow(dead_code)]
    pub struct TestSerialzier {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    #[chain(packer)]
    #[derive(Clone)]
    pub struct MyStruct {
        amount: i64,
        symbol: u64,
    }

    impl AsAny for MyStruct {
        fn as_any(&self) -> &dyn core::any::Any {
            self
        }
    }

    impl PrimaryValueInterface for MyStruct {
        ///
        fn get_primary(&self) -> u64 {
            return self.amount as u64;
        }
    }

    impl SecondaryValueInterface for MyStruct {
        ///
        fn get_secondary_value(&self, i: usize) -> SecondaryValue {
            return SecondaryValue::Idx64(1);
        }
        ///
        fn set_secondary_value(&mut self, i: usize, value: SecondaryValue) {
            if let SecondaryValue::Idx64(x) = value {
                self.amount = x as i64;
            }
        }
    }

    impl MultiIndexValue for MyStruct {}
    

    impl TestSerialzier {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action="test")]
        pub fn test(&self) {
            // 
            fn unpacker(raw: &[u8]) -> Box<dyn MultiIndexValue>  {
                let mut value = MyStruct::default();
                value.unpack(raw);
                Box::new(value)
            };

            let code = self.receiver;
            let scope = self.receiver;
            let table = name!("hello");
            let indexes: Vec<SecondaryType> = Vec::new();
            let mi = MultiIndex::new(code, scope, table, &indexes, unpacker);
            let it = mi.find(1);
            if let Some(value) = it.get_value() {
                if let Some(x) = value.as_any().downcast_ref::<MyStruct>() {
                    let mut mystruct = x.clone();
                    mystruct.amount += 1;
                    mi.update(&it, &mystruct, self.receiver);
                    eosio_println!("++++amount:", mystruct.amount);
                }
            } else {
                let mystruct = MyStruct{amount: 1, symbol: 1};
                mi.store(&mystruct, self.receiver);
                eosio_println!("++++amount:", mystruct.amount);
            }
            eosio_println!("test done!");
        }
    }

}

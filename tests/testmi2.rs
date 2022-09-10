#[rust_chain::contract]
pub mod testmi2 {
    use rust_chain::{
        Name,
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
            MultiIndex,
        },

        name,
        check,
        eosio_println,
    };

    #[chain(sub)]
    #[allow(dead_code)]
    pub struct TestMI2 {
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

        fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
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
    

    impl TestMI2 {

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
            }
            let code = self.receiver;
            let scope = self.receiver;
            let table = name!("hello");
            let indices: Vec<SecondaryType> = Vec::new();
            let mi = MultiIndex::new(code, scope, table, &indices, unpacker);
            let it = mi.find(1);
            if let Some(mut value) = it.get_value() {
                if let Some(x) = value.as_any_mut().downcast_mut::<MyStruct>() {
                    x.amount += 1;
                    mi.update(&it, x, self.receiver);
                    eosio_println!("++++amount:", x.amount);
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

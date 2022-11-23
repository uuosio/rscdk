#[rust_chain::contract]
pub mod testmi2 {
    use rust_chain::{
        Name,
        db::{
            SecondaryType,
            //SecondaryValue,
            // SecondaryIterator,
            MultiIndexValue,
            // PrimaryValueInterface,
            //SecondaryValueInterface,
            AsAny,
        },

        mi_not_generic::{
            MultiIndex,
            cast_value,
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

    #[chain(table="mystruct")]
    #[derive(Clone)]
    pub struct MyStruct {
        #[chain(primary)]
        amount: u64,
        #[chain(secondary)]
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

    // impl PrimaryValueInterface for MyStruct {
    //     ///
    //     fn get_primary(&self) -> u64 {
    //         return self.amount as u64;
    //     }
    // }

    // impl SecondaryValueInterface for MyStruct {
    //     ///
    //     fn get_secondary_value(&self, _i: usize) -> SecondaryValue {
    //         return SecondaryValue::Idx64(1);
    //     }
    //     ///
    //     fn set_secondary_value(&mut self, _i: usize, value: SecondaryValue) {
    //         if let SecondaryValue::Idx64(x) = value {
    //             self.amount = x as i64;
    //         }
    //     }
    // }

    impl MultiIndexValue for MyStruct {}
    
    impl TestMI2 {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        pub fn init_test(&self) -> MultiIndex {
            // 
            fn unpacker(raw: &[u8]) -> Box<dyn MultiIndexValue>  {
                let mut value = MyStruct::default();
                value.unpack(raw);
                Box::new(value)
            }
            let code = self.receiver;
            let scope = self.receiver;
            let table = name!("hello");
            let indices: Vec<SecondaryType> = vec![SecondaryType::Idx64];
            return MultiIndex::new(code, scope, table, &indices, unpacker);
        }

        #[chain(action="test")]
        pub fn test(&self) {
            let payer = self.receiver;
            let mi = self.init_test();
            mi.set(&MyStruct{amount: 1, symbol: 2}, payer);
            mi.set(&MyStruct{amount: 11, symbol: 22}, payer);
            mi.set(&MyStruct{amount: 111, symbol: 222}, payer);
            eosio_println!("test done!");
        }

        #[chain(action="test2")]
        pub fn test2(&self) {
            let payer = self.receiver;
            let mi = self.init_test();
            let it = mi.find(11);
            check(it.get_primary().unwrap() == 11, "it.get_primary() == 11");
            check(it.get_value_ex::<MyStruct>().unwrap().amount == 11, "bad value");

            let it = mi.next(&it);
            check(it.get_value_ex::<MyStruct>().unwrap().amount == 111, "bad value");

            let it = mi.previous(&it);
            check(it.get_value_ex::<MyStruct>().unwrap().amount == 11, "bad value");

            let it = mi.lower_bound(11);
            check(it.get_value_ex::<MyStruct>().unwrap().amount == 11, "bad value");

            let it = mi.upper_bound(11);
            check(it.get_value_ex::<MyStruct>().unwrap().amount == 111, "bad value");

            let data = it.get_value_ex::<MyStruct>().unwrap();
            mi.set(&data, payer);

            {
                let _data = it.get_value();
                let data = cast_value::<MyStruct>(&_data);
                check(data.unwrap().amount == 111, "data.unwrap().amount == 111");
            }
            let mut data = it.get_value_ex::<MyStruct>().unwrap();
            data.symbol += 1;
            mi.set(&data, payer);

            let data2 = mi.get(&it).unwrap();
            check(data2.get_primary() == 111, "data2.get_primary() == 111");

            mi.remove(&it);
            check(!mi.find(111).is_ok(), "!mi.find(111).is_ok()");

            // check(value.get_value().unwrap().amount == 1, "");
            // if let Some(mut value) = it.get_value() {
            //     if let Some(x) = value.as_any_mut().downcast_mut::<MyStruct>() {
            //         x.amount += 1;
            //         mi.update(&it, x, self.receiver);
            //         eosio_println!("++++amount:", x.amount);
            //     }
            // } else {
            //     let mystruct = MyStruct{amount: 1, symbol: 1};
            //     mi.store(&mystruct, self.receiver);
            //     eosio_println!("++++amount:", mystruct.amount);
            // }
            eosio_println!("test done!");
        }
    }

}

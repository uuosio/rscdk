#![cfg_attr(not(feature = "std"), no_std)]

#[eosio_chain::contract]
mod hello {
    use eosio_chain::{
        Name,
        Float128,
        Uint256,

        name,
        eosio_println,
        check,

        db::{
            SecondaryValue,
            SecondaryIterator,
        },

        action::{
            Action,
            PermissionLevel,
        },
        ACTIVE,
    };

    #[chain(table="counter", singleton)]
    pub struct Counter {
        count: u64
    }

    #[chain(main)]
    #[allow(dead_code)]
    pub struct Hello {
        receiver: Name,
        first_receiver: Name,
        action: Name,
    }

    #[chain(table="mydata")]
    pub struct MyData {
        #[chain(primary)]
        a1: u64,
        #[chain(secondary)]
        a2: u64,
        #[chain(secondary)]
        a3: u128,
        #[chain(secondary)]
        a4: Uint256,
        #[chain(secondary)]
        a5: f64,
        #[chain(secondary)]
        a6: Float128,
    }

    impl Hello {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
            }
        }

        #[chain(action="sayhello")]
        pub fn say_hello(&self, name: String) {
            for i in 0..=1 {
                eosio_println!("++++hello:", name);
                // return;
                let perms: Vec<PermissionLevel> = vec![PermissionLevel{actor: name!("hello"), permission: ACTIVE}];
                let say_goodbye = saygoodbye{name: name.clone()};
                let action = Action::new(name!("hello"), name!("saygoodbye"), &perms, &say_goodbye);
                action.send();
            }
        }

        #[chain(action="saygoodbye")]
        pub fn say_goodbye(&self, name: String) {
            eosio_println!("++++hello:", name);
        }

        #[chain(action = "inc")]
        pub fn inc_count(&self) {
            for _ in 0..1 {
                let db = Counter::new_table(self.receiver);
                let mut value = db.get().unwrap_or(Counter{count: 1});
                eosio_println!("+++++count2:", value.count);
                value.count += 1;
                db.set(&value, self.receiver);    
            }
        }

        #[chain(action="testmi")]
        pub fn testmi(&self) {
            eosio_println!("+++++test2");
            let receiver = self.receiver;

            let mydb = MyData::new_table(receiver);

            let it = mydb.find(1);
            // 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x80,0x01,0x40,
            // 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x08,0x05,0x40,
            // 0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x4d,0x08,0x40,
            //6.0
            let a6_6: Float128 = Float128::new([0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x80,0x01,0x40]);
            //66.0
            let a6_66: Float128 = Float128::new([0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x08,0x05,0x40]);
            //666.0
            let a6_666: Float128 = Float128::new([0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x4d,0x08,0x40]);

            if !it.is_ok() {
                //6.0
                let mydata = MyData{a1: 1, a2: 2, a3: 3, a4: Uint256::new(4, 0), a5: 5.0, a6: a6_6};
                mydb.store(&mydata, receiver);  
            }

            let it = mydb.find(11);
            if !it.is_ok() {
                let mydata = MyData{a1: 11, a2: 22, a3: 33, a4: Uint256::new(44, 0), a5: 55.0, a6: a6_66};
                mydb.store(&mydata, receiver);    
            }

            let it = mydb.find(111);
            if !it.is_ok() {
                let mydata = MyData{a1: 111, a2: 222, a3: 333, a4: Uint256::new(444, 0), a5: 555.0, a6: a6_666};
                mydb.store(&mydata, receiver);
            }

            let check_fn = |it: SecondaryIterator, checker: fn(data: &MyData) -> bool | -> bool {

                let it_primary = mydb.find(it.primary);
                if !it_primary.is_ok() {
                    return false;
                }

                if let Some(x) = mydb.get(&it_primary) {
                    if !checker(&x) {
                        return false;
                    }
                    return true;
                } else {
                    return false;
                }
            };

            //test for Idx64Table.previous
            {
                let idx = mydb.get_idx_by_a2();

                //test for Idx64Table.previous
                {
                    let it_secondary = idx.find(22 as u64);
                    let it_secondary_previous = idx.previous(&it_secondary);
                    let ret = check_fn(it_secondary_previous, |data: &MyData| {
                        data.a1 == 1 && data.a2 == 2
                    });
                    check(ret, "Idx64Table: bad secondary previous value");
                }

                //test for Idx64Table.next
                {
                    let it_secondary = idx.find(22 as u64);
                    let it_secondary_next = idx.next(&it_secondary);
                    let ret = check_fn(it_secondary_next, |data: &MyData| {
                        data.a1 == 111 && data.a2 == 222
                    });
                    check(ret, "bad secondary next value");
                }

                //test for Idx64Table.lower_bound
                {
                    let (it_secondary, secondary) = idx.lower_bound(22);
                    check(it_secondary.primary == 11, "bad primary value!");
                    check(secondary == 22, "Idx64Table.lower_bound: bad secondary value!");

                    let ret = check_fn(it_secondary, |data: &MyData| {
                        data.a1 == 11 && data.a2 == 22
                    });
                    check(ret, "bad secondary next value");
                }

                //test for Idx64Table.upper_bound
                {
                    let (it_secondary, secondary) = idx.upper_bound(22);
                    check(it_secondary.primary == 111, "upper_bound: bad primary value!");
                    eosio_println!("+++++++secondary:", secondary);
                    check(secondary == 222, "Id64Table.upper_boundd: bad secondary value!");

                    let ret = check_fn(it_secondary, |data: &MyData| {
                        data.a1 == 111 && data.a2 == 222
                    });
                    check(ret, "bad secondary next value");
                }
            }

            //test for Idx128Table.previous
            {
                let idx = mydb.get_idx_by_a3();
                //test for Idx128Table.previous
                {
                    let it_secondary = idx.find(33u128);
                    let it_secondary_previous = idx.previous(&it_secondary);
                    let ret = check_fn(it_secondary_previous, |data: &MyData| {
                        data.a1 == 1 && data.a2 == 2 && data.a3 == 3
                    });
                    check(ret, "Idx128Table: bad secondary previous value");
                }

                //test for Idx128Table.next
                {
                    let it_secondary = idx.find(33u128);
                    let it_secondary_next = idx.next(&it_secondary);
                    let ret = check_fn(it_secondary_next, |data: &MyData| {
                        data.a1 == 111 && data.a2 == 222 && data.a3 == 333
                    });
                    check(ret, "bad secondary next value");
                }

                //test for Idx128Table.lower_bound
                {
                    let (it_secondary, secondary) = idx.lower_bound(33);
                    check(it_secondary.primary == 11, "bad primary value!");
                    eosio_println!(it_secondary.primary, secondary);
                    check(secondary == 33, "Idx128Table.lower_bound: bad secondary value!");

                    let ret = check_fn(it_secondary, |data: &MyData| {
                        data.a1 == 11 && data.a2 == 22 && data.a3 == 33
                    });
                    check(ret, "bad secondary next value");
                }

                //test for Idx128Table.upper_boundddd
                {
                    let (it_secondary, secondary) = idx.upper_bound(33);
                    check(it_secondary.primary == 111, "upper_bound: bad primary value!");
                    eosio_println!("+++++++secondary:", secondary);
                    check(secondary == 333, "Idx128Table.upper_bound: bad secondary value!");

                    let ret = check_fn(it_secondary, |data: &MyData| {
                        data.a1 == 111 && data.a2 == 222 && data.a3 == 333
                    });
                    check(ret, "bad secondary next value");
                }
            }

            //test for Idx256Table.previous
            {
                let idx = mydb.get_idx_by_a4();
                //test for Idx256Table.previous
                {
                    let it_secondary = idx.find(Uint256::new(44, 0));
                    let it_secondary_previous = idx.previous(&it_secondary);
                    let ret = check_fn(it_secondary_previous, |data: &MyData| {
                        data.a1 == 1 && data.a2 == 2 && data.a3 == 3
                    });
                    check(ret, "Idx256Table: bad secondary previous value");
                }

                //test for Idx256Table.next
                {
                    let it_secondary = idx.find(Uint256::new(44, 0));
                    let it_secondary_next = idx.next(&it_secondary);
                    let ret = check_fn(it_secondary_next, |data: &MyData| {
                        data.a1 == 111 && data.a2 == 222 && data.a3 == 333 && data.a4 == Uint256::new(444, 0)
                    });
                    check(ret, "bad secondary next value");
                }

                //test for Idx256Table.lower_bound
                {
                    let (it_secondary, secondary) = idx.lower_bound(Uint256::new(44, 0));
                    check(it_secondary.primary == 11, "bad primary value!");
                    check(secondary == Uint256::new(44, 0), "Idx256Table.lower_bound: bad secondary value!");

                    let ret = check_fn(it_secondary, |data: &MyData| {
                        data.a1 == 11 && data.a2 == 22 && data.a3 == 33 && data.a4 == Uint256::new(44, 0)
                    });
                    check(ret, "bad secondary next value");
                }

                //test for Idx256Table.upper_bound
                {
                    let (it_secondary, secondary) = idx.upper_bound(Uint256::new(44, 0));
                    check(it_secondary.primary == 111, "upper_bound: bad primary value!");
                    eosio_println!("+++++++secondary:", secondary);
                    check(secondary == Uint256::new(444, 0), "Idx256Table.upper_bound: bad secondary value!");

                    let ret = check_fn(it_secondary, |data: &MyData| {
                        data.a1 == 111 && data.a2 == 222 && data.a3 == 333 && data.a4 == Uint256::new(444, 0)
                    });
                    check(ret, "bad secondary next value");
                }
            }

            //test for IdxF64Table.previous
            {
                let idx = mydb.get_idx_by_a5();
                //test for IdxF64Table.previous
                {
                    let it_secondary = idx.find(55.0);
                    let it_secondary_previous = idx.previous(&it_secondary);
                    let ret = check_fn(it_secondary_previous, |data: &MyData| {
                        data.a1 == 1 && data.a2 == 2 && data.a3 == 3 && data.a4 == Uint256::new(4, 0) && data.a5 == 5.0
                    });
                    check(ret, "IdxF64Table: bad secondary previous value");
                }

                //test for IdxF64Table.next
                {
                    let it_secondary = idx.find(55.0);
                    let it_secondary_next = idx.next(&it_secondary);
                    let ret = check_fn(it_secondary_next, |data: &MyData| {
                        data.a1 == 111 && data.a2 == 222 && data.a3 == 333 && data.a4 == Uint256::new(444, 0) && data.a5 == 555.0
                    });
                    check(ret, "bad secondary next value");
                }

                //test for IdxF64Table.lower_bound
                {
                    let (it_secondary, secondary) = idx.lower_bound(55.0);
                    check(it_secondary.primary == 11, "bad primary value!");
                    check(secondary == 55.0, "IdxF64Table.lower_bound: bad secondary value!");

                    let ret = check_fn(it_secondary, |data: &MyData| {
                        data.a1 == 11 && data.a2 == 22 && data.a3 == 33 && data.a4 == Uint256::new(44, 0) && data.a5 == 55.0
                    });
                    check(ret, "bad secondary next value");
                }

                //test for IdxF64Table.upper_bound
                {
                    let (it_secondary, secondary) = idx.upper_bound(55.0);
                    check(it_secondary.primary == 111, "upper_bound: bad primary value!");
                    eosio_println!("+++++++secondary:", secondary);
                    check(secondary == 555.0, "IdxF64Table.upper_bound: bad secondary value!");

                    let ret = check_fn(it_secondary, |data: &MyData| {
                        data.a1 == 111 && data.a2 == 222 && data.a3 == 333 && data.a4 == Uint256::new(444, 0) && data.a5 == 555.0
                    });
                    check(ret, "bad secondary next value");
                }
            }

            //test for IdxF128Table.previous
            {
                let idx = mydb.get_idx_by_a6();
                //test for IdxF128Table.previous
                {
                    let it_secondary = idx.find(a6_66);
                    let it_secondary_previous = idx.previous(&it_secondary);
                    let ret = check_fn(it_secondary_previous, |data: &MyData| {
                        let a6_6: Float128 = Float128::new([0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x80,0x01,0x40]);
                        eosio_println!("a6_6", a6_6);
                        data.a1 == 1 && data.a2 == 2 && data.a3 == 3 && data.a4 == Uint256::new(4, 0) && data.a5 == 5.0 && data.a6 == a6_6
                    });

                    check(ret, "IdxF128Table: bad secondary previous value");
                }

                //test for IdxF128Table.next
                {
                    let it_secondary = idx.find(a6_66);
                    let it_secondary_next = idx.next(&it_secondary);
                    let ret = check_fn(it_secondary_next, |data: &MyData| -> bool {
                        let a6_666: Float128 = Float128::new([0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x4d,0x08,0x40]);
                        return data.a1 == 111 && data.a2 == 222 && data.a3 == 333 && data.a4 == Uint256::new(444, 0) && data.a5 == 555.0 && data.a6 == a6_666
                    });
                    check(ret, "IdxF128Table: bad secondary next value");
                }

                //test for IdxF128Table.lower_bound
                {
                    let (it_secondary, secondary) = idx.lower_bound(a6_66);
                    check(it_secondary.primary == 11, "bad primary value!");
                    check(secondary == a6_66, "IdxF128Db.lower_bound: bad secondary value!");

                    let ret = check_fn(it_secondary, |data: &MyData| {
                        let a6_66: Float128 = Float128::new([0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x08,0x05,0x40]);
                        data.a1 == 11 && data.a2 == 22 && data.a3 == 33 && data.a4 == Uint256::new(44, 0) && data.a5 == 55.0 && data.a6 == a6_66
                    });
                    check(ret, "IdxF128Table.lower_bound: bad secondary value");
                }

                //test for IdxF128Table.upper_bound
                {
                    let (it_secondary, secondary) = idx.upper_bound(a6_66);
                    check(it_secondary.primary == 111, "upper_bound: bad primary value!");
                    eosio_println!("+++++++secondary:", secondary);
                    check(secondary == a6_666, "IdxF128Table.upper_bound: bad secondary value!");

                    let ret = check_fn(it_secondary, |data: &MyData| {
                        let a6_666: Float128 = Float128::new([0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x4d,0x08,0x40]);
                        data.a1 == 111 && data.a2 == 222 && data.a3 == 333 && data.a4 == Uint256::new(444, 0) && data.a5 == 555.0 && data.a6 == a6_666
                    });
                    check(ret, "IdxF128Db.upper_bound: bad secondary value");
                }
            }
            // eosio_println!("+++++MyDataIdx::a2:", MyDataIdx::a2 as usize);
            let idx_db = mydb.get_idx_db(0 as usize);
            let mut it_secondary = idx_db.find(SecondaryValue::Idx64(2));
            eosio_println!("+++++++2 it_secondary.is_ok():", it_secondary.is_ok(), it_secondary.i);
            it_secondary = idx_db.find(SecondaryValue::Idx64(3));
            eosio_println!("+++++++3 it_secondary.is_ok():", it_secondary.is_ok(), it_secondary.i);

            if it_secondary.is_ok() {
                eosio_println!("++++it_secondary:", it_secondary.i, it_secondary.primary);
                mydb.idx_update(&it_secondary, SecondaryValue::Idx64(3), receiver);    
            }

            {
                if let Some(value) = mydb.get_by_primary(1) {
                    eosio_println!("+++value:", value.a1, value.a2);
                }
            }
            eosio_println!("test2 done!");
        }
    }

    // #[no_mangle]
    // fn apply(receiver: u64, first_receiver: u64, action: u64) {
    //     prints("hello, debugger!!!");
    //     return;
    //     use eosio_chain::eosio_chaintester;
    //     use eosio_chain::eosio_chaintester::chaintester::TApplySyncClient;
    //     let mut client = eosio_chaintester::new_vm_api_client("127.0.0.1", 9092).unwrap();
    //     client.prints(String::from("hello, debugger!")).unwrap();            
    // }

    // #[no_mangle]
    // fn native_apply(receiver: u64, first_receiver: u64, action: u64) {
    //     apply(receiver, first_receiver, action);
    // }
}

#[cfg(test)]
mod tests {

    use eosio_chain::ChainTester;
    use eosio_chain::serializer::Packer;
    use crate::hello::sayhello;

    fn deploy_contract(tester: &mut ChainTester) {
        tester.deploy_contract("hello", "/Users/newworld/dev/github/rscdk/tests/testdebug/../target/testdebug/testdebug.wasm", "/Users/newworld/dev/github/rscdk/tests/testdebug/../target/testdebug/testdebug.abi").unwrap();
    }

    #[test]
    fn test_prints() {
        let exe = std::env::current_exe();
        println!("defined in file: {exe:?}");
    
        let mut tester = ChainTester::new();
        tester.enable_debug(true);

        deploy_contract(&mut tester);
        let updateauth_args = r#"{
            "account": "hello",
            "permission": "active",
            "parent": "owner",
            "auth": {
                "threshold": 1,
                "keys": [
                    {
                        "key": "EOS6AjF6hvF7GSuSd4sCgfPKq5uWaXvGM2aQtEUCwmEHygQaqxBSV",
                        "weight": 1
                    }
                ],
                "accounts": [{"permission":{"actor": "hello", "permission": "eosio.code"}, "weight":1}],
                "waits": []
            }
        }"#;

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;

        tester.push_action("eosio", "updateauth", updateauth_args.into(), permissions).unwrap();
        tester.produce_block();
    
        let args = sayhello{name: "rust".into()};
        tester.push_action("hello", "sayhello", args.pack().into(), permissions).unwrap();
        tester.produce_block();
    }

    #[test]
    fn test_counter() {
        let args = "{}";
        let permissions = r#"
        {
            "hello": "active"
        }
        "#;

        {
            let mut tester = ChainTester::new();
            deploy_contract(&mut tester);

            let r = tester.push_action("hello", "inc", args.into(), permissions).unwrap();
            tester.produce_block();

            tester.push_action("hello", "inc", args.into(), permissions).unwrap();
            tester.produce_block();
        }
        {
            let mut tester = ChainTester::new();
            deploy_contract(&mut tester);

            tester.push_action("hello", "inc", args.into(), permissions).unwrap();
            tester.produce_block();

            tester.push_action("hello", "inc", args.into(), permissions).unwrap();
            tester.produce_block();

        }
    }

    #[test]
    fn test_mi() {
        let mut tester = ChainTester::new();

        let args = "";

        let permissions = r#"
        {
            "hello": "active"
        }
        "#;
        tester.push_action("hello", "testmi", args.into(), permissions).unwrap();
    }
}

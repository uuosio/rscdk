use rust_chain as chain;

#[chain::contract]
pub mod testintrinsics {
    use rust_chain::{
        Action,
        PermissionLevel,

        crypto,
        Checksum256,
        PublicKey,
        Signature,
        Name,
        action::{
            get_code_hash,
        },
        get_active_producers,

        assert_sha256,
        assert_sha1,
        assert_sha512,
        assert_ripemd160,
    
        sha256,
        sha1,
        sha512,
        ripemd160,

        read_transaction,
        check_transaction_authorization,
        check_permission_authorization,
        get_permission_last_used,
        get_account_creation_time,

        //action.h
        read_action_data,
        action_data_size,
        require_recipient,
        require_auth,
        has_auth,
        require_auth2,
        is_account,
        send_inline,
        send_context_free_inline,
        publication_time,
        current_receiver,
//        set_action_return_value

        //privileged.h
        get_resource_limits,
        set_resource_limits,
        //set_proposed_producers,
        //set_proposed_producers_ex,
        is_privileged,
        set_privileged,
        set_blockchain_parameters,
        get_blockchain_parameters,
        //preactivate_feature,

        set_action_return_value,
        get_block_num,

        current_time,

        check,
        chain_println,
        name,
    };

    #[chain(table="mydata")]
    pub struct MyData {
        #[chain(primary)]
        pub a1: u64,
        #[chain(secondary)]
        pub a2: u64,
    }

    #[chain(sub)]
    #[allow(dead_code)]
    pub struct TestSerialzier {
        receiver: Name,
        first_receiver: Name,
        action: Name,
        value: u32,
    }

    impl TestSerialzier {

        pub fn new(receiver: Name, first_receiver: Name, action: Name) -> Self {
            Self {
                receiver: receiver,
                first_receiver: first_receiver,
                action: action,
                value: 0,
            }
        }

        #[chain(action="test")]
        pub fn test(&self, msg: String, digest: Checksum256, sig: Signature, pubkey: PublicKey) {
            chain_println!("+++++++msg:", msg);
            if "goodbye" == msg {
                return;
            }

            let _pubkey = crypto::recover_key(&digest, &sig);
            check(_pubkey == pubkey, "bad value");
            crypto::assert_recover_key(&digest, &sig, &pubkey);

            let data: Vec<u8> =  vec![1, 2, 3, 4, 5, 6, 7];
            let ret = sha256(&data);
            assert_sha256(&data, &ret);

            let ret = sha1(&data);
            assert_sha1(&data, &ret);

            let ret = sha512(&data);
            assert_sha512(&data, &ret);

            let ret = ripemd160(&data);
            assert_ripemd160(&data, &ret);
            check(is_privileged(name!("hello")), "not prviledged!");
            set_privileged(name!("hello"), true);

            // permission.h
            // int32_t check_transaction_authorization( const char* trx_data,     uint32_t trx_size,
            //     const char* pubkeys_data, uint32_t pubkeys_size,
            //     const char* perms_data,   uint32_t perms_size
            // );
            let trx = read_transaction();
            check_transaction_authorization(&trx, &vec![PermissionLevel::new(name!("hello"), name!("active"))], &Vec::new());

            // int32_t check_permission_authorization( capi_name account,
            //                 capi_name permission,
            //                 const char* pubkeys_data, uint32_t pubkeys_size,
            //                 const char* perms_data,   uint32_t perms_size,
            //                 uint64_t delay_us
            //             );
            check_permission_authorization(name!("hello"), name!("active"), &vec![PermissionLevel::new(name!("hello"), name!("active"))], &Vec::new(), 0);

            // int64_t get_permission_last_used( capi_name account, capi_name permission );
            let time = get_permission_last_used(name!("hello"), name!("active"));
            chain_println!("+++time.elapsed:", time.elapsed);

            // int64_t get_account_creation_time( capi_name account );
            let time = get_account_creation_time(name!("hello"));
            chain_println!("+++time.elapsed:", time.elapsed);

            // uint32_t read_action_data( void* msg, uint32_t len );
            let data = read_action_data();
            chain_println!("+++data.len:", data.len());
            
            // test is a generated struct
            let mut test = test::default();
            test.unpack(&data);
            check(test.msg == "hello,world", "bad value");

            // uint32_t action_data_size();
            check(action_data_size() == data.len(), "bad value");

            // void require_recipient( capi_name name );
            require_recipient(name!("hello"));

            // void require_auth( capi_name name );
            require_auth(name!("hello"));
            // bool has_auth( capi_name name );
            check(has_auth(name!("hello")), "bad value");
            check(!has_auth(name!("eosio")), "bad value");
            // void require_auth2( capi_name name, capi_name permission );
            require_auth2(name!("hello"), name!("active"));
            // bool is_account( capi_name name );
            check(is_account(name!("hello")), "bad value");
            check(!is_account(name!("helloooo")), "bad value");
            
            // void send_inline(char *serialized_action, uint32_t size);
            test.msg = "goodbye".into();
            let a = Action::new(name!("hello"), name!("test"), PermissionLevel::new(name!("hello"), name!("active")), &test);
            send_inline(&Encoder::pack(&a));

            // void send_context_free_inline(char *serialized_action, uint32_t size);
            // a.authorization = vec![];
            // chain_println!("+++a.pack():", a.pack());
            // send_context_free_inline(&a.pack());

            // uint64_t  publication_time();
            let time = publication_time();
            chain_println!("++++++publication time:", time.elapsed);

            // capi_name current_receiver();
            check(current_receiver() == name!("hello"), "bad receiver");
            // void set_action_return_value(const char *data, uint32_t data_size);

            //privileged.h
            // void get_resource_limits( capi_name account, int64_t* ram_bytes, int64_t* net_weight, int64_t* cpu_weight );
            let (ram_bytes, net_weight, cpu_weight) = get_resource_limits(name!("hello"));
            chain_println!(ram_bytes, net_weight, cpu_weight);

            // void set_resource_limits( capi_name account, int64_t ram_bytes, int64_t net_weight, int64_t cpu_weight );
            set_resource_limits(name!("hello"), 10000000, 10000000, 10000000);
            let (ram_bytes, net_weight, cpu_weight) = get_resource_limits(name!("hello"));
            chain_println!(ram_bytes, net_weight, cpu_weight);

            // int64_t set_proposed_producers( const char *producer_data, uint32_t producer_data_size );
            // int64_t set_proposed_producers_ex( uint64_t producer_data_format, const char *producer_data, uint32_t producer_data_size );
            // bool is_privileged( capi_name account );
            check(is_privileged(name!("eosio")), "is_privileged(eosio)");
            check(is_privileged(name!("hello")), "is_privileged(hello)");

            // void set_privileged( capi_name account, bool is_priv );
            set_privileged(name!("hello"), true);

            // void set_blockchain_parameters_packed( const char* data, uint32_t datalen );
            // uint32_t get_blockchain_parameters_packed( char* data, uint32_t datalen );
            let params = get_blockchain_parameters();
            set_blockchain_parameters(&params);
            // void preactivate_feature( const capi_checksum256* feature_digest );

            //chain.h
            let prods = get_active_producers();
            chain_println!("++++++prods.len():", prods.len());
            check(prods.len() == 1, "prods.len() == 1");
            check(prods[0] == name!("eosio"), "bad value");
            chain_println!("intrinsics tests done!");
        }

        #[chain(action="test2")]
        pub fn test_set_action_return_value(&self) {
            set_action_return_value(String::from("helloworld").into_bytes());
        }

        #[chain(action="test3")]
        pub fn test_block_num(&self, num: u32) {
            check(get_block_num() == num, "get_block_num() == num");
        }

        #[chain(action="testctxfree")]
        pub fn test_context_free_action(&self) {
            let mut a = Action::new_ex(name!("hello"), name!("testsendfree"), vec![], &MyData{a1:1, a2: 2});
            a.authorization = vec![];
            chain_println!("+++a.pack():", Encoder::pack(&a));
            send_context_free_inline(&Encoder::pack(&a));
        }

        #[chain(action="testtime")]
        pub fn test_block_time(&self) {
            chain_println!("+++++++current_time:", current_time().elapsed);
        }

        #[chain(action="testcodehash")]
        pub fn test_code_hash(&self, hash: Checksum256) {
            let ret = get_code_hash(self.first_receiver);
            check(ret == hash, "bad code hash!");
        }
    }
}

#![cfg_attr(not(feature = "std"), no_std)]

use eosio_chain as chain;

#[chain::contract]
pub mod testcrypto {
    use eosio_chain::{
        crypto,
        Checksum256,
        PublicKey,
        Signature,
        Name,

        assert_sha256,
        assert_sha1,
        assert_sha512,
        assert_ripemd160,
    
        sha256,
        sha1,
        sha512,
        ripemd160,

        check,
        eosio_println,
    };

    #[chain(table="mydata")]
    pub struct MyData {
        #[chain(primary)]
        pub a1: u64,
        #[chain(secondary)]
        pub a2: u64,
    }

    #[chain(main)]
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
            eosio_println!("done!");
        }
    }
}

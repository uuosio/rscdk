#![cfg_attr(not(feature = "std"), no_std)]

use rust_chain as chain;

#[chain::contract]
pub mod testcrypto {
    use rust_chain::{
        crypto,

        Checksum160,
        Checksum256,
        Checksum512,
        ECCPublicKey,
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

        // "k1": "EOS87J9kj21dvniKhqd7A7QPXRz498ek3H3doXoQVPf4VnHHNtt1M",
        // "r1": "PUB_R1_6FPFZqw5ahYrR9jD96yDbbDNTdKtNqRbze6oTDLntrsANgQKZu",
        // "webAuthN": "PUB_WA_8PPYTWYNkRqrveNAoX7PJWDtSqDUp3c29QGBfr6MD9EaLocaPBmsk5QAHWq4vEQt2"

        #[chain(action="test")]
        pub fn test(&self, msg: String, digest: Checksum256, sig: Signature, k1: PublicKey, r1: PublicKey, web_auth_n: PublicKey) {
            eosio_println!("++++++msg:", msg);
            let _pubkey = crypto::recover_key(&digest, &sig);
            check(_pubkey == k1, "_pubkey == k1");
            crypto::assert_recover_key(&digest, &sig, &k1);
            let raw_r1: Vec<u8> = vec![0x01,0x02,0xb3,0x23,0xea,0x27,0xd1,0x91,0x14,0x3e,0xb9,0xad,0x27,0xc9,0x6d,0xb1,0x5d,0x8b,0x12,0x9d,0x30,0x96,0xa0,0xcb,0x17,0xae,0x11,0xae,0x26,0xab,0xce,0x80,0x33,0x40];
            let raw_webauthn: Vec<u8> = vec![0x02,0x03,0x78,0xb7,0x61,0x07,0xe4,0x50,0x33,0x28,0xbd,0xd1,0x09,0x93,0x4d,0x63,0xab,0xc4,0x45,0x7c,0x7c,0x8a,0x0f,0x59,0x12,0x6d,0x28,0x8f,0xa5,0x11,0x89,0x75,0x2e,0x03,0x01,0x09,0x6c,0x6f,0x63,0x61,0x6c,0x68,0x6f,0x73,0x74];
            
            let mut web_auth = PublicKey::default();
            web_auth.unpack(&raw_webauthn);
            check(r1.pack() == raw_r1, "r1.pack() == raw_r1");
            check(web_auth_n.pack() == raw_webauthn, "web_auth_n.pack() == raw_webauthn");
    
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

        #[chain(action="test2")]
        pub fn test2(&self) {
            let _ = rust_chain::utils::decode_hex("0000000000000000000000000000000000000000000000000000000000000000");
            //let hash = Checksum256::from_hex("0000000000000000000000000000000000000000000000000000000000000000");
            Checksum160::from_hex("0000000000000000000000000000000000000000");
            Checksum512::from_hex("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
            ECCPublicKey::from_hex("000000000000000000000000000000000000000000000000000000000000000000");
            Signature::from_hex("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
        }

        #[chain(action="test3")]
        pub fn test3(&self) {
            let _ = rust_chain::utils::decode_hex("000000000000000000000000000000000000000000000000000000000000000Z");
        }
    }
}

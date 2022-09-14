#![cfg_attr(not(feature = "std"), no_std)]

use rust_chain as chain;

#[chain::contract]
pub mod testabi {
    use rust_chain::{
        VarUint32,
        Float128, //TODO:
        TimePoint,
        TimePointSec,
        BlockTimeStampType,
        Name,
        Checksum160,
        Checksum256,
        Checksum512,
        PublicKey,
        Signature,
        Symbol,
        SymbolCode,
        Asset,
        ExtendedAsset,

        ECCPublicKey,

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

        // "Varint32" => "varint32",
        // "VarUint32" => "varuint32",
        // "f32" => "float32",
        // "f64" => "float64",

        // "Float128" => "float128",


        // "Checksum160" => "checksum160",
        // "Checksum256" => "checksum256",
        // "Checksum512" => "checksum512",
        // "PublicKey" => "public_key",
        // "Signature" => "signature",
        // "Symbol" => "symbol",
        // "SymbolCode" => "symbol_code",
        // "Asset" => "asset",
        // "ExtendedAsset" => "extended_asset",

        #[chain(action="test")]
        pub fn test(&self,
            a1: bool,
            a2: i8,
            a3: u8,
            a4: i16,
            a5: u16,
            a6: i32,
            a7: u32,
            a8: i64,
            a9: u64,
            a10: i128,
            a11: u128,
            //a12: VarInt32,
            a13: VarUint32,
            a14: f32,
            a15: f64,
            // a16: Float128, //TODO:
            a17: TimePoint,
            a18: TimePointSec,
            // a19: BlockTimeStampType,
            a20: Name,
            a21: Vec<u8>,
            a22: String,
            a23: Checksum160,
            a24: Checksum256,
            a25: Checksum512,
            a26: PublicKey,
            a27: Signature,
            a28: Symbol,
            a29: SymbolCode,
            a30: Asset,
            a31: ExtendedAsset,
        ) {
            check(a1 == true, "bad a1 value!");
            check(a2 == -1, "bad a2 value!");
            
            check(a3 == 0xff, "bad a3 value!");

            check(a4 == -1, "bad a4 value!");
            check(a5 == 0xffff, "bad a5 value!");

            check(a6 == -1, "bad a6 value!");
            check(a7 == 0xffffffff, "bad a7 value!");

            check(a8 == -1, "bad a8 value!");
            check(a9 == 0xffffffffffffffff, "bad a9 value!");

            check(a10 == -1, "bad a10 value!");
            check(a11 == 0xffffffffffffffffffffffffffffffff, "bad a11 value!");

            check(a13.n == 0xffffffff, "bad a13 value!");
            check(a14 == 1.1, "bad a14 value!");
            check(a15 == 2.2, "bad a15 value!");

            check(a20 == Name::new("eosio"), "bad value a20");
            check(a21 == String::from("hello").as_bytes().to_vec(), "bad value a21");
            check(a22 == String::from("hello"), "bad value a22");

            check(a23 == Checksum160::from_hex("bbaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), "bad value 23");
            check(a24 == Checksum256::from_hex("bbaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), "bad value 24");
            check(a25 == Checksum512::from_hex("bbaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), "bad value 25");

            check(a26 == PublicKey::K1(ECCPublicKey::from_hex("0234ee2fc290bfad20635b8a79212b86ff13f8b866274a9fff9de79786a2eaafc6")), "bad value a26");
            check(a27 == Signature::from_hex("20331f956b5b344e5d225c857cceac8183a90dd883201510e34b5cd60aac0d7da015f7da1a6a53ef5c6572050826b1d8a70e56abbe815a513d7c796314c59774a7"), "bad value a27");

            check(a30 == Asset::from_string("1.0000 EOS"), "bad value a30");
            check(a31 == ExtendedAsset::new(Asset::from_string("1.0000 EOS"), Name::new("eosio.token")), "bad value a31");
            eosio_println!("test serializer done!");
            // eosio_println!(
            //     a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a13,
            //     a14, a15,
            //     a11, a13,
            //     a30
            // );
        }
    }
}

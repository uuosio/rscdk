#[rust_chain::contract]
pub mod testserializer {
    use rust_chain::{
        VarUint32,
        // Float128, //TODO:
        TimePoint,
        TimePointSec,
        // BlockTimeStampType,
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
        check,
        name,
    };
    
    #[chain(packer)]
    #[derive(PartialEq)]
    struct TestData {
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
        a31: ExtendedAsset
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
        pub fn test(&self) {
            let data = TestData {
                a1: false,
                a2: -1,
                a3: 0xff,
                a4: -1i16,
                a5: 0xffu16,
                a6: -1i32,
                a7: 0xffffffffu32,
                a8: -1i64,
                a9: 0xffffffffffffffffu64,
                a10: -1i128,
                a11: 0xffffffffffffffffffffffffffffffffu128,
                //a12: VarInt32,
                a13: VarUint32::new(0xffffffff),
                a14: 1.1f32,
                a15: 1.1f64,
                // a16: Float128, //TODO:
                a17: TimePoint{elapsed: 1},
                a18: TimePointSec{seconds: 1},
                // a19: BlockTimeStampType,
                a20: name!("hello"),
                a21: vec![1u8, 2u8, 3u8],
                a22: String::from("hello"),
                a23: Checksum160::default(),
                a24: Checksum256::default(),
                a25: Checksum512::default(),
                a26: PublicKey::default(),
                a27: Signature::default(),
                a28: Symbol::new("EOS", 4),
                a29: SymbolCode::new("EOS"),
                a30: Asset::new(1000, Symbol::new("EOS", 4)),
                a31: ExtendedAsset::new(Asset::new(1000, Symbol::new("EOS", 4)), name!("hello"))
            };
            
            let mut _data = TestData::default();
            _data.unpack(&Encoder::pack(&data));
            check(data == _data, "data == _data");
        }

        #[chain(action="test2")]
        pub fn test2(&self) {
            let mut s = String::default();
            s.unpack(&vec![2, 0xff, 0xff]);    
        }
    }
}

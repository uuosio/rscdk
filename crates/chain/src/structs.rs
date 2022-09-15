use crate::utils::{
    decode_hex,
};

use crate::vmapi::eosio::{
    eosio_memcpy,
    check,
};

use crate::name::{ Name };

use crate::print::{ 
    Printable,
    printui128,
    // printhex,
};

use crate::serializer::{
    Packer,
    Encoder,
    Decoder,
};

use crate::varint::{
    VarUint32,
};

use crate::{
    vec,
    vec::Vec,
    string::String,
};

///
#[repr(C)]
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct Float128 {
    ///
    pub data: [u8; 16],
}

impl Float128 {
    ///
    pub fn new(data: [u8;16]) -> Self {
        Self {
            data: data
        }
    }

    ///
    pub fn data(&self) -> &[u8; 16] {
        return &self.data;
    }
}

impl Packer for Float128 {
    fn size(&self) -> usize {
        return 16;
    }

    fn pack(&self) -> Vec<u8> {
        return self.data.to_vec();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= 16, "Float128.unpack: buffer overflow!");
        eosio_memcpy(self.data.as_mut_ptr(), raw.as_ptr(), 16);
        return 16;
    }
}

///
#[repr(C)]
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct Checksum160 {
    pub data: [u8; 20],
}

impl Checksum160 {
    ///
    pub fn from_hex(s: &str) -> Self {
        check(s.len() == 40, "Checksum160: bad hex string length");
        let data = decode_hex(s);
        let mut ret = Self::default();
        eosio_memcpy(ret.data.as_mut_ptr(), data.as_ptr(), 40);
        return ret;
    }
}

impl Packer for Checksum160 {
    fn size(&self) -> usize {
        return 20;
    }

    fn pack(&self) -> Vec<u8> {
        return self.data.to_vec();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= 20, "Checksum160.unpack: buffer overflow!");
        eosio_memcpy(self.data.as_mut_ptr(), raw.as_ptr(), 20);
        return 20;
    }
}

///
#[repr(C)]
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct Checksum256 {
    ///
    pub data: [u8; 32],
}

impl Checksum256 {
    ///
    pub fn from_hex(s: &str) -> Self {
        check(s.len() == 64, "Checksum256: bad hex string length");
        let data = decode_hex(s);
        let mut ret = Self::default();
        eosio_memcpy(ret.data.as_mut_ptr(), data.as_ptr(), 32);
        return ret;
    }
}

impl Packer for Checksum256 {
    fn size(&self) -> usize {
        return 32;
    }

    fn pack(&self) -> Vec<u8> {
        return self.data.to_vec();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= 32, "Checksum256.unpack: buffer overflow!");
        eosio_memcpy(self.data.as_mut_ptr(), raw.as_ptr(), 32);
        return 32;
    }
}

///
#[repr(C)]
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Checksum512 {
    pub data: [u8; 64],
}

impl Checksum512 {
    ///
    pub fn from_hex(s: &str) -> Self {
        check(s.len() == 128, "Checksum512: bad hex string length");
        let data = decode_hex(s);
        let mut ret = Self::default();
        eosio_memcpy(ret.data.as_mut_ptr(), data.as_ptr(), 64);
        return ret;
    }
}

impl Default for Checksum512 {
    ///
    #[inline]
    fn default() -> Self {
        Checksum512{data: [0; 64]}
    }
}

impl Packer for Checksum512 {
    fn size(&self) -> usize {
        return 64;
    }

    fn pack(&self) -> Vec<u8> {
        return self.data.to_vec();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= 64, "Checksum512.unpack: buffer overflow!");
        eosio_memcpy(self.data.as_mut_ptr(), raw.as_ptr(), 64);
        return 64;
    }
}


///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ECCPublicKey {
    ///
    pub data: [u8; 33],
}

impl ECCPublicKey {
    ///
    pub fn from_hex(s: &str) -> Self {
        let mut ret = Self::default();
        check(s.len() == 33*2, "ECCPublicKey: bad hex string length");
        let data = decode_hex(s);
        eosio_memcpy(ret.data.as_mut_ptr(), data.as_ptr(), 33);
        return ret;
    }
}

impl Default for ECCPublicKey {
    ///
    #[inline]
    fn default() -> Self {
        ECCPublicKey{data: [0; 33]}
    }
}

impl Packer for ECCPublicKey {
    fn size(&self) -> usize {
        return 33;
    }

    fn pack(&self) -> Vec<u8> {
        return self.data.to_vec();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= 33, "EccPublicKey.unpack: buffer overflow!");
        eosio_memcpy(self.data.as_mut_ptr(), raw.as_ptr(), 33);
        return 33;
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum UserPresence {
    ///
    None = 0,
    ///
    Present = 1,
    ///
    Verified = 2,
}

impl Default for UserPresence {
    fn default() -> Self {
        UserPresence::None
    }
}

impl Packer for UserPresence {
    ///
    fn size(&self) -> usize {
        return 1;
    }

    ///
    fn pack(&self) -> Vec<u8> {
        vec![*self as u8]
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= 1, "UserPresence.unpack: buffer overflow");
        match data[0] {
            0 => {
                *self = UserPresence::None;
            },
            1 => {
                *self = UserPresence::Present;
            },
            2 => {
                *self = UserPresence::Verified;
            }
            _ => {
                check(false, "not a UserPresence type");
            }
        }
        return 1;
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Eq, PartialEq, Default)]
pub struct WebAuthNPublicKey {
    ///
    pub key: ECCPublicKey,
    ///
    pub user_presence: UserPresence,
    ///
    pub rpid: String,
}

impl WebAuthNPublicKey {
    ///
    pub fn new(key: ECCPublicKey, user_presence: UserPresence, rpid: String) -> Self {
        Self { key, user_presence, rpid }
    }
}

impl Packer for WebAuthNPublicKey {
    ///
    fn size(&self) -> usize {
        self.key.size() + self.user_presence.size() + self.rpid.size()
    }

    ///
    fn pack(&self) -> Vec<u8> {
        let mut enc = Encoder::new(self.size());
        enc.pack(&self.key);
        enc.pack(&self.user_presence);
        enc.pack(&self.rpid);
        return enc.get_bytes();
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= 33, "WebAuthNPublicKey.unpack: buffer overflow!");
        let mut dec = Decoder::new(data);
        dec.unpack(&mut self.key);
        dec.unpack(&mut self.user_presence);
        dec.unpack(&mut self.rpid);
        return dec.get_pos();
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Eq, PartialEq)]
pub enum PublicKey {
    ///
    K1(ECCPublicKey),
    ///
    R1(ECCPublicKey),
    ///
    WebAuth(WebAuthNPublicKey),
}

impl Default for PublicKey {
    ///
    #[inline]
    fn default() -> Self {
        PublicKey::K1(ECCPublicKey::default())
    }
}

impl Packer for PublicKey {
    fn size(&self) -> usize {
        match self {
            PublicKey::K1(x) => x.size() + 1,
            PublicKey::R1(x) => x.size() + 1,
            PublicKey::WebAuth(x) => x.size() + 1,
        }
    }

    fn pack(&self) -> Vec<u8> {
        let mut enc = Encoder::new(self.size());
        match self {
            PublicKey::K1(x) => {
                enc.pack_number(0u8);
                enc.pack(x);
            }
            PublicKey::R1(x) => {
                enc.pack_number(1u8);
                enc.pack(x);
            }
            PublicKey::WebAuth(x) => {
                enc.pack_number(2u8);
                enc.pack(x);
            }
        }
        return enc.get_bytes();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= 34, "PublicKey.unpack: buffer overflow!");

        let mut dec = Decoder::new(raw);
        let mut ty: u8 = 0;
        dec.unpack(&mut ty);
        match ty {
            0 => {
                let mut pub_key = ECCPublicKey::default();
                dec.unpack(&mut pub_key);
                *self = PublicKey::K1(pub_key);
            },
            1 => {
                let mut pub_key = ECCPublicKey::default();
                dec.unpack(&mut pub_key);
                *self = PublicKey::R1(pub_key);
            },
            2 => {
                let mut pub_key = WebAuthNPublicKey::default();
                dec.unpack(&mut pub_key);
                *self = PublicKey::WebAuth(pub_key);
            }
            _ => {
                check(false, "invalid public key type");
            }
        }
        return dec.get_pos();
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Eq, PartialEq)]
pub struct Signature {
    /// Signature type
	ty: u8,
    ///
	data: [u8; 65],
}

impl Signature {
    ///
    pub fn from_hex(s: &str) -> Self {
        let mut ret = Self::default();
        check(s.len() == 65*2, "Signature: bad hex string length");
        let data = decode_hex(s);
        ret.ty = 0;
        eosio_memcpy(ret.data.as_mut_ptr(), data.as_ptr(), 65);
        return ret;
    }
}

impl Default for Signature {
    fn default() -> Self {
        Self { ty: 0, data: [0; 65] }
    }
}

impl Packer for Signature {
    ///
    fn size(&self) -> usize {
        return 66;
    }

    ///
    fn pack(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(66);
        v.push(self.ty);
        v.append(&mut self.data.to_vec());
        return v;
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= 66, "Signature::unpack: buffer overflow");
        self.ty = data[0];
        check(self.ty == 0, "bad signature type");
        eosio_memcpy(self.data.as_mut_ptr(), data[1..66].as_ptr(), 66);
        return 66;
    }
}

///
#[repr(C)]
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uint128 {
    ///
    pub lo: u64,
    ///
    pub hi: u64,
}

impl Default for Uint128 {
    ///
    #[inline]
    fn default() -> Self {
        Self {
            lo: 0,
            hi: 0,
        }
    }
}

///
#[repr(C)]
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Default)]
pub struct Int128 {
    ///
    pub lo: u64,
    ///
    pub hi: u64,
}

///
#[repr(C)]
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uint256 {
    ///
    pub data: [u128; 2],
}

impl Uint256 {
    ///
    pub fn new(lo: u128, hi: u128) -> Self {
        Self { data: [lo, hi] }
    }
}

impl Packer for Uint256 {
    ///
    fn size(&self) -> usize {
        return 32;
    }

    ///
    fn pack(&self) -> Vec<u8> {
        let mut enc = Encoder::new(32);
        enc.pack(&self.data[0]);
        enc.pack(&self.data[1]);
        return enc.get_bytes();
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        let mut dec = Decoder::new(data);
        dec.unpack(&mut self.data[0]);
        dec.unpack(&mut self.data[1]);
        return dec.get_pos();
    }
}

impl Printable for Uint256 {
    fn print(&self) {
        if self.data[1] == 0 {
            printui128(self.data[0]);
        } else {
            crate::vmapi::print::printhex(self.data.as_ptr() as *mut u8, 32);
        }
    }
}

impl Default for Uint256 {
    ///
    #[inline]
    fn default() -> Self {
        Self {
            data: [Default::default(); 2],
        }
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Default)]
pub struct TimePoint {
    /// elapsed in microseconds
    pub elapsed: u64,
}

impl Packer for TimePoint {
    fn size(&self) -> usize {
        return 8;
    }

    fn pack(&self) -> Vec<u8> {
        return self.elapsed.pack();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= self.size(), "TimePoint.unpack: buffer overflow!");
        return self.elapsed.unpack(raw);
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct TimePointSec {
    ///
    pub utc_seconds: u32,
}

impl TimePointSec {
    pub fn utc_seconds(&self) -> u32 {
        return self.utc_seconds;
    }
}

impl Packer for TimePointSec {
    fn size(&self) -> usize {
        return 4;
    }

    fn pack(&self) -> Vec<u8> {
        return self.utc_seconds.pack();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= self.size(), "TimePointSec.unpack: buffer overflow!");
        return self.utc_seconds.unpack(raw);
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Default)]
pub struct BlockTimeStampType {
    ///
    pub slot: u32,
}

impl Packer for BlockTimeStampType {
    fn size(&self) -> usize {
        return 4;
    }

    fn pack(&self) -> Vec<u8> {
        return self.slot.pack();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= self.size(), "BlockTimeStampType.unpack: buffer overflow!");
        return self.slot.unpack(raw);
    }
}

///
#[derive(Clone, Eq, PartialEq)]
pub struct ProducerKey {
    ///
    pub producer_name: Name,
    ///
    pub block_signing_key: PublicKey,
}

impl ProducerKey {
    ///
    pub fn new(producer_name: Name, block_signing_key: PublicKey) -> Self {
        ProducerKey {
            producer_name,
            block_signing_key,
        }
    }
}

impl Packer for ProducerKey {
    fn size(&self) -> usize {
        return 8 + self.block_signing_key.size();
    }

    fn pack(&self) -> Vec<u8> {
        let mut enc = Encoder::new(self.size());
        enc.pack(&self.producer_name);
        enc.pack(&self.block_signing_key);
        return enc.get_bytes();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let mut dec = Decoder::new(raw);
        dec.unpack(&mut self.producer_name);
        dec.unpack(&mut self.block_signing_key);
        return dec.get_pos();
    }
}

impl Default for ProducerKey {
    ///
    #[inline]
    fn default() -> Self {
        ProducerKey { producer_name: Default::default(), block_signing_key: Default::default() }
    }
}

#[derive(Default)]
pub struct KeyWeight {
    pub key: PublicKey,
    pub weight: u16,
}

impl Packer for KeyWeight {
    fn size(&self) -> usize {
        return 2 + self.key.size();
    }

    fn pack(&self) -> Vec<u8> {
        let mut enc = Encoder::new(self.size());
        enc.pack(&self.key);
        enc.pack(&self.weight);
        return enc.get_bytes();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let mut dec = Decoder::new(raw);
        dec.unpack(&mut self.key);
        dec.unpack(&mut self.weight);
        return dec.get_pos();
    }
}

#[derive(Default)]
pub struct BlockSigningAuthorityV0 {
    /**
     * minimum threshold of accumulated weights from component keys that satisfies this authority
     *
     * @brief minimum threshold of accumulated weights from component keys that satisfies this authority
     */
    pub threshold: u32,

    /**
     * component keys and their associated weights
     *
     * @brief component keys and their associated weights
     */
    pub keys: Vec<KeyWeight>,
}

impl Packer for BlockSigningAuthorityV0 {
    fn size(&self) -> usize {
        let mut size = 2 + VarUint32::new(self.keys.len() as u32).size();
        for key in &self.keys {
            size += key.size();
        }
        return size;
    }

    fn pack(&self) -> Vec<u8> {
        let mut enc = Encoder::new(self.size());
        enc.pack(&self.threshold);
        enc.pack(&self.keys);
        return enc.get_bytes();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let mut dec = Decoder::new(raw);
        dec.unpack(&mut self.threshold);
        dec.unpack(&mut self.keys);
        return dec.get_pos();
    }
}

pub enum BlockSigningAuthority {
    V0(BlockSigningAuthorityV0)
}

impl Default for BlockSigningAuthority {
    fn default() -> Self {
        BlockSigningAuthority::V0(Default::default())
    }
}

impl Packer for BlockSigningAuthority {
    fn size(&self) -> usize {
        return 1 + match self {
            BlockSigningAuthority::V0(x) => x.size(),
        };
    }

    fn pack(&self) -> Vec<u8> {
        let mut enc = Encoder::new(self.size());
        enc.pack(&0u8);
        match self {
            BlockSigningAuthority::V0(x) => {
                enc.pack(x);
            }
        }
        return enc.get_bytes();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let mut dec = Decoder::new(raw);
        let mut ty = 0u8;
        dec.unpack(&mut ty);
        if ty == 0 {
            let mut v0 = BlockSigningAuthorityV0::default();
            dec.unpack(&mut v0);
            *self = BlockSigningAuthority::V0(v0);
        } else {
            check(false, "bad BlockSigningAuthority type");
        }
        return dec.get_pos();
    }
}


#[derive(Default)]
pub struct ProducerAuthority {

    /**
     * Name of the producer
     *
     * @brief Name of the producer
     */
    pub producer_name: Name,

    /**
     * The block signing authority used by this producer
     */
    pub authority: BlockSigningAuthority,
}

impl Packer for ProducerAuthority {
    fn size(&self) -> usize {
        return self.producer_name.size() + self.authority.size();
    }

    fn pack(&self) -> Vec<u8> {
        let mut enc = Encoder::new(self.size());
        enc.pack(&self.producer_name);
        enc.pack(&self.authority);
        return enc.get_bytes();
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let mut dec = Decoder::new(raw);
        dec.unpack(&mut self.producer_name);
        dec.unpack(&mut self.authority);
        return dec.get_pos();
    }
}

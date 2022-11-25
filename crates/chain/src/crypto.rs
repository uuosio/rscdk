use crate::vmapi::crypto;

use crate::structs::*;

use crate::serializer::{
    Packer as _,
    Encoder
};

use crate::vec;

///
pub fn assert_sha256(data: &[u8], hash: &Checksum256) {
    crypto::assert_sha256(data.as_ptr(), data.len() as u32, hash);
}

///
pub fn assert_sha1(data: &[u8], hash: &Checksum160) {
    crypto::assert_sha1(data.as_ptr(), data.len() as u32, hash);
}

///
pub fn assert_sha512(data: &[u8], hash: &Checksum512) {
    crypto::assert_sha512(data.as_ptr(), data.len() as u32, hash);
}

pub fn assert_ripemd160(data: &[u8], hash: &Checksum160) {
    crypto::assert_ripemd160(data.as_ptr(), data.len() as u32, hash)
}

///
pub fn sha256(data: &[u8]) -> Checksum256 {
    let mut hash: Checksum256 = Default::default();
    crypto::sha256(data.as_ptr(), data.len() as u32, &mut hash);
    return hash;
}

///
pub fn sha1( data: &[u8]) -> Checksum160 {
    let mut hash: Checksum160 = Checksum160::default();
    crypto::sha1(data.as_ptr(), data.len() as u32, &mut hash);
    return hash;
}

///
pub fn sha512( data: &[u8]) -> Checksum512 {
    let mut hash: Checksum512 = Checksum512::default();
    crypto::sha512(data.as_ptr(), data.len() as u32, & mut hash);
    return hash;
}

///
pub fn ripemd160(data: &[u8]) -> Checksum160 {
    let mut hash: Checksum160 = Checksum160::default();
    crypto::ripemd160(data.as_ptr(), data.len() as u32, &mut hash);
    return hash;
}

///
pub fn recover_key( digest: &Checksum256 , sig: &Signature) -> PublicKey {
    let mut pubkey = vec![0u8; 34];
    let _sig = Encoder::pack(sig);

    crypto::recover_key(digest, _sig.as_ptr(), _sig.len(), pubkey.as_mut_ptr(), pubkey.len());
    let mut ret = PublicKey::default();
    ret.unpack(&pubkey);
    return ret;
}

///
pub fn assert_recover_key(digest: &Checksum256, sig: &Signature, pubkey: &PublicKey) {
    let _sig = Encoder::pack(sig);
    let _pubkey = Encoder::pack(pubkey);
    crypto::assert_recover_key(digest, _sig.as_ptr(), _sig.len(), _pubkey.as_ptr(), _pubkey.len());
}

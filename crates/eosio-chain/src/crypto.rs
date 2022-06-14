use crate::vmapi::crypto;

use crate::structs::*;

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

// pub fn assert_ripemd160( data: *const u8, length: u32, hash: *const Checksum160);

///
pub fn sha256(data: &[u8]) -> Checksum256 {
    let mut hash: Checksum256 = Default::default();
    crypto::sha256(data.as_ptr(), data.len() as u32, &mut hash);
    return hash;
}

///
pub fn sha1( data: &[u8]) -> Checksum160 {
    let mut hash: Checksum160 = [0; 20];
    crypto::sha1(data.as_ptr(), data.len() as u32, &mut hash);
    return hash;
}

///
pub fn sha512( data: &[u8]) -> Checksum512 {
    let mut hash: Checksum512 = [0; 64];
    crypto::sha512(data.as_ptr(), data.len() as u32, & mut hash);
    return hash;
}

///
pub fn ripemd160(data: &[u8]) -> Checksum160 {
    let mut hash: Checksum160 = [0; 20];
    crypto::ripemd160(data.as_ptr(), data.len() as u32, &mut hash);
    return hash;
}

///
pub fn recover_key( digest: &Checksum256 , sig: &[u8]) -> i32 {
    let mut pubkey: [u8; 65] = [0; 65];
    let ret = crypto::recover_key(digest, sig.as_ptr(), sig.len(), pubkey.as_mut_ptr(), pubkey.len());
    return ret;
}

///
pub fn assert_recover_key(digest: &Checksum256, sig: &[u8], _pub: &[u8]) {
    crypto::assert_recover_key(digest, sig.as_ptr(), sig.len(), _pub.as_ptr(), _pub.len());
}

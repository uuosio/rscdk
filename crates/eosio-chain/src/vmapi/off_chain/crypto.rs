use crate::structs::*;

///
pub fn assert_sha256( _data: *const u8, _length: u32, _hash: *const Checksum256) {

}

///
pub fn assert_sha1( _data: *const u8, _length: u32, _hash: *const Checksum160) {
    
}

///
pub fn assert_sha512( _data: *const u8, _length: u32, _hash: *const Checksum512) {
    
}

///
pub fn assert_ripemd160( _data: *const u8, _length: u32, _hash: *const Checksum160) {
    
}

///
pub fn sha256( _data: *const u8, _length: u32, _hash: *mut Checksum256 ) {
    
}

///
pub fn sha1( _data: *const u8, _length: u32, _hash: *mut Checksum160 ) {
    
}

///
pub fn sha512( _data: *const u8, _length: u32, _hash: *mut Checksum512 ) {
    
}

///
pub fn ripemd160( _data: *const u8, _length: u32, _hash: *mut Checksum160 ) {
    
}

///
pub fn recover_key( _digest: *const Checksum256 , _sig: *const u8, _siglen: usize, _pub: *mut u8, _publen: usize ) -> i32 {
    return 0;
}

///
pub fn assert_recover_key(_digest: *const Checksum256, _sig: *const u8, _siglen: usize, _pub: *const u8, _publen: usize) {
    
}

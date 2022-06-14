use crate::structs::*;

mod intrinsics {
    use crate::structs::*;
    extern "C" {
        ///
        pub fn assert_sha256( data: *const u8, length: u32, hash: *const Checksum256);
    
        ///
        pub fn assert_sha1( data: *const u8, length: u32, hash: *const Checksum160);
    
        ///
        pub fn assert_sha512( data: *const u8, length: u32, hash: *const Checksum512);
    
        ///
        pub fn assert_ripemd160( data: *const u8, length: u32, hash: *const Checksum160);
    
        ///
        pub fn sha256( data: *const u8, length: u32, hash: *mut Checksum256 );
    
        ///
        pub fn sha1( data: *const u8, length: u32, hash: *mut Checksum160 );
    
        ///
        pub fn sha512( data: *const u8, length: u32, hash: *mut Checksum512 );
    
        ///
        pub fn ripemd160( data: *const u8, length: u32, hash: *mut Checksum160 );
    
        ///
        pub fn recover_key( digest: *const Checksum256 , sig: *const u8, siglen: usize, _pub: *mut u8, publen: usize ) -> i32;
    
        ///
        pub fn assert_recover_key(digest: *const Checksum256, sig: *const u8, siglen: usize, _pub: *const u8, publen: usize);
    }    
}

///
pub fn assert_sha256( data: *const u8, length: u32, hash: *const Checksum256){
    unsafe {
        return intrinsics::assert_sha256( data, length, hash);
    }
}

///
pub fn assert_sha1( data: *const u8, length: u32, hash: *const Checksum160){
    unsafe {
        return intrinsics::assert_sha1( data, length, hash);
    }
}

///
pub fn assert_sha512( data: *const u8, length: u32, hash: *const Checksum512){
    unsafe {
        return intrinsics::assert_sha512( data, length, hash);
    }
}

///
pub fn assert_ripemd160( data: *const u8, length: u32, hash: *const Checksum160){
    unsafe {
        return intrinsics::assert_ripemd160( data, length, hash);
    }
}

///
pub fn sha256( data: *const u8, length: u32, hash: *mut Checksum256 ){
    unsafe {
        return intrinsics::sha256( data, length, hash);
    }
}

///
pub fn sha1( data: *const u8, length: u32, hash: *mut Checksum160 ){
    unsafe {
        return intrinsics::sha1( data, length, hash);
    }
}

///
pub fn sha512( data: *const u8, length: u32, hash: *mut Checksum512 ){
    unsafe {
        return intrinsics::sha512( data, length, hash);
    }
}

///
pub fn ripemd160( data: *const u8, length: u32, hash: *mut Checksum160 ){
    unsafe {
        return intrinsics::ripemd160( data, length, hash);
    }
}

///
pub fn recover_key( digest: *const Checksum256 , sig: *const u8, siglen: usize, _pub: *mut u8, publen: usize ) -> i32{
    unsafe {
        return intrinsics::recover_key( digest, sig, siglen, _pub, publen);
    }
}

///
pub fn assert_recover_key(digest: *const Checksum256, sig: *const u8, siglen: usize, _pub: *const u8, publen: usize){
    unsafe {
        return intrinsics::assert_recover_key(digest, sig, siglen, _pub, publen);
    }
}

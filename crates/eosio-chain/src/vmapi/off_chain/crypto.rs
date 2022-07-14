use crate::structs::*;

use eosio_chaintester::{
    get_vm_api_client,
	interfaces::{
		TApplySyncClient,
	}
};

use core::slice;

///
pub fn assert_sha256( data: *const u8, length: u32, hash: *const Checksum256) {
    unsafe {
        let _data = slice::from_raw_parts(data, length as usize);
        get_vm_api_client().assert_sha256(_data.into(), (*hash).data.into()).unwrap()
    }
}

///
pub fn assert_sha1( data: *const u8, length: u32, hash: *const Checksum160) {
    unsafe {
        let _data = slice::from_raw_parts(data, length as usize);
        get_vm_api_client().assert_sha1(_data.into(), (*hash).data.into()).unwrap()
    }
}

///
pub fn assert_sha512(data: *const u8, length: u32, hash: *const Checksum512) {
    unsafe {
        let _data = slice::from_raw_parts(data, length as usize);
        get_vm_api_client().assert_sha512(_data.into(), (*hash).data.into()).unwrap()
    }
}

///
pub fn assert_ripemd160(data: *const u8, length: u32, hash: *const Checksum160) {
    unsafe {
        let _data = slice::from_raw_parts(data, length as usize);
        get_vm_api_client().assert_ripemd160(_data.into(), (*hash).data.into()).unwrap()
    }    
}

///
pub fn sha256( data: *const u8, length: u32, hash: *mut Checksum256 ) {
    unsafe {
        let _data = slice::from_raw_parts(data, length as usize);
        let _hash = get_vm_api_client().sha256(_data.into()).unwrap();
        crate::vmapi::eosio::memcpy((*hash).data.as_ptr() as *mut u8, _hash.as_ptr(), 32);
    }
}

///
pub fn sha1(data: *const u8, length: u32, hash: *mut Checksum160) {
    unsafe {
        let _data = slice::from_raw_parts(data, length as usize);
        let _hash = get_vm_api_client().sha1(_data.into()).unwrap();
        crate::vmapi::eosio::memcpy((*hash).data.as_ptr() as *mut u8, _hash.as_ptr(), 20);
    }
}

///
pub fn sha512(data: *const u8, length: u32, hash: *mut Checksum512 ) {
    unsafe {
        let _data = slice::from_raw_parts(data, length as usize);
        let _hash = get_vm_api_client().sha1(_data.into()).unwrap();
        crate::vmapi::eosio::memcpy((*hash).data.as_ptr() as *mut u8, _hash.as_ptr(), 64);
    }
}

///
pub fn ripemd160(data: *const u8, length: u32, hash: *mut Checksum160 ) {
    unsafe {
        let _data = slice::from_raw_parts(data, length as usize);
        let _hash = get_vm_api_client().ripemd160(_data.into()).unwrap();
        crate::vmapi::eosio::memcpy((*hash).data.as_ptr() as *mut u8, _hash.as_ptr(), 20);
    }
}

///
pub fn recover_key( digest: *const Checksum256 , sig: *const u8, siglen: usize, pubkey: *mut u8, publen: usize ) -> i32 {
    if publen != 34 {
        panic!("invalid pub key length");
    }

    unsafe {
        let _sig = slice::from_raw_parts(sig, siglen as usize);
        let _pubkey = get_vm_api_client().recover_key((*digest).data.into(), _sig.into()).unwrap();
        crate::vmapi::eosio::memcpy(pubkey, _pubkey.as_ptr(), _pubkey.len());
        _pubkey.len() as i32
    }
}

///
pub fn assert_recover_key(digest: *const Checksum256, sig: *const u8, siglen: usize, pubkey: *const u8, pubkey_len: usize) {
    if pubkey_len != 34 {
        panic!("invalid pub key length");
    }

    unsafe {
        let _sig = slice::from_raw_parts(sig, siglen as usize);
        let _pubkey = slice::from_raw_parts(pubkey, pubkey_len as usize);
        get_vm_api_client().assert_recover_key((*digest).data.into(), _sig.into(), _pubkey.into()).unwrap();
    }
}

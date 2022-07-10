use std::ffi::CStr;
use eosio_chaintester::chaintester::TApplySyncClient;

use eosio_chaintester::{
    get_vm_api_client,
    new_vm_api_client
};

use crate::structs::*;
use core::slice;

///
pub fn prints(cstr: *const u8) {
    let s = unsafe { CStr::from_ptr(cstr as *const i8).to_str().unwrap() };
    let mut client = get_vm_api_client();
    client.prints(s.to_owned()).unwrap();
}

///
pub fn prints_l(_cstr: *const u8, _len: u32) {
    let s = unsafe {
        slice::from_raw_parts(_cstr, _len as usize)
    };

    let _s = std::str::from_utf8(s).unwrap();
    get_vm_api_client().prints(_s.to_owned()).unwrap();
}

///
pub fn printi(_value: i64) {
}

///
pub fn printui(_value: u64) {
}

///
pub fn printi128(_value: i128) {
}

///
pub fn printui128(_value: u128) {
}

///
pub fn printsf(_value: f32) {
}

///
pub fn printdf(_value: f64) {
}

///
pub fn printqf(_value: *const Float128) {
}

///
pub fn printn(_name: u64) {
}

///
pub fn printhex(_data: *const u8, _datalen: u32) {
}


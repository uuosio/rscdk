use std::ffi::CStr;
use eosio_chaintester::interfaces::TApplySyncClient;

use eosio_chaintester::{
    get_vm_api_client,
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

    get_vm_api_client().prints_l(s.to_vec()).unwrap();
}

///
pub fn printi(value: i64) {
    get_vm_api_client().printi(value).unwrap()
}

///
pub fn printui(value: u64) {
    get_vm_api_client().printui(value.into()).unwrap()
}

///
pub fn printi128(value: i128) {
    get_vm_api_client().printi128(value.to_le_bytes().to_vec()).unwrap()
}

///
pub fn printui128(value: u128) {
    get_vm_api_client().printui128(value.to_le_bytes().to_vec()).unwrap()
}

///
pub fn printsf(value: f32) {
    get_vm_api_client().printsf(value.to_le_bytes().to_vec()).unwrap()
}

///
pub fn printdf(value: f64) {
    get_vm_api_client().printdf(value.to_le_bytes().to_vec()).unwrap()
}

///
pub fn printqf(value: *const Float128) {
    unsafe {
        get_vm_api_client().printqf((*value).data.to_vec()).unwrap()
    }
}

///
pub fn printn(name: u64) {
    get_vm_api_client().printn(name.into()).unwrap()
}

///
pub fn printhex(data: *const u8, datalen: u32) {
    let s = unsafe {
        slice::from_raw_parts(data, datalen as usize)
    };

    get_vm_api_client().printhex(s.to_vec()).unwrap()
}


use crate::structs::*;
use crate::vmapi::system;
use crate::name::{
	Name,
};

use crate::{
	// vec,
	vec::Vec,
};

use core::slice;

use eosio_chaintester::{
    get_vm_api_client,
	interfaces::{
		TApplySyncClient,
	}
};

///
pub fn memcpy( dst: *mut u8, src: *const u8, length: usize) -> *mut u8 {
    let mut _dst = unsafe {
        slice::from_raw_parts_mut(dst, length)
    };

    let _src = unsafe {
        slice::from_raw_parts(src, length)
    };
	_dst.copy_from_slice(_src);
	dst
}

///
pub fn eosio_memcpy( dst: *mut u8, src: *const u8, length: usize) -> *mut u8 {
	return memcpy(dst, src, length);
}

///
pub fn get_active_producers() -> Vec<Name> {
	return Vec::new();
}

///
pub fn get_permission_last_used( _account: Name, _permission: Name ) -> TimePoint {
	return TimePoint{elapsed: 0};
}

///
pub fn get_account_creation_time( _account: Name ) -> TimePoint {
	return TimePoint{elapsed: 0};
}

///
pub fn read_action_data() -> Vec<u8> {
	let size = get_vm_api_client().action_data_size().unwrap();
	let ret = get_vm_api_client().read_action_data(size).unwrap();
	return ret.buffer.unwrap()
}

///
pub fn action_data_size() -> usize {
	return 0;
}

///
pub fn require_recipient(_name: Name) {
}

///
pub fn require_auth(_name: Name) {
}

///
pub fn has_auth( _name: Name ) -> bool {
	return false;
}

///
pub fn require_auth2( _name: Name, _permission: Name ) {
}

///
pub fn is_account( _name: Name ) -> bool {
	return false;
}

///
pub fn send_inline(_serialized_action: &[u8]) {
	get_vm_api_client().send_inline(_serialized_action.to_vec()).unwrap();
}

///
pub fn send_context_free_inline(_serialized_action: &[u8]) {
}

///
pub fn publication_time() -> TimePoint {
	return TimePoint{elapsed: 0};
}

///
pub fn current_receiver() -> Name {
	return Name{n: 0};
}

///
pub fn eosio_assert(_test: bool, _msg: &str) {
}

///
pub fn check(test: bool, msg: &str) {
	system::eosio_assert_message(test as u32, msg.as_ptr(), msg.len() as u32);
}

///
pub fn eosio_assert_code(_test: u32, _code: u64) {
}

///
pub fn eosio_exit(_code: i32) {
}

///
pub fn current_time() -> TimePoint {
	return TimePoint{elapsed: 0};
}

///
pub fn is_feature_activated(_feature_digest: &Checksum256) -> bool {
	return false;
}

///
pub fn get_sender() -> Name {
	return Name{n: 0};
}

///
pub fn get_resource_limits( _account: Name) -> (i64, i64, i64) {
	return (0, 0, 0);
}

///
pub fn set_resource_limits( _account: Name, _ram_bytes: i64, _net_weight: i64, _cpu_weight: i64) {
}

//TODO:
///
pub fn set_proposed_producers(_producer_keys: &[ProducerKey]) -> i64 {
	return -1;
}

//TODO:
///
pub fn set_proposed_producers_ex(_producer_data_format: u64, _producer_keys: &[ProducerKey]) -> i64 {
	return -1;
}

///
pub fn is_privileged(_account: Name) -> bool {
	return false;
}

///
pub fn set_privileged( _account: Name, _is_priv: bool) {
}

//TODO
///
pub fn set_blockchain_parameters_packed(_data: &[u8]) {
}

///
pub fn get_blockchain_parameters_packed() -> Vec<u8> {
	return Vec::new();
}

///
pub fn preactivate_feature(_feature_digest:  &Checksum256) {
}

//TODO:
///
pub fn send_deferred(_sender_id: &Uint128, _payer: Name, _serialized_transaction: &[u8], _replace_existing: u32) {
}

///
pub fn cancel_deferred(_sender_id: &Uint128) -> i32 {
	return 0;
}

//TODO:
///
pub fn read_transaction() -> Vec<u8> {
	return Vec::new();
}

///
pub fn transaction_size() -> usize {
	return 0;
}

///
pub fn tapos_block_num() -> i32 {
	return 0;
}

///
pub fn tapos_block_prefix() -> i32 {
	return 0;
}

///
pub fn expiration() -> u32 {
	return 0;
}

///
pub fn get_action(_type: u32, _index: u32) -> Vec<u8> {
	return Vec::new();
}

///
pub fn get_context_free_data(_index: u32) -> Vec<u8> {
	return Vec::new();
}

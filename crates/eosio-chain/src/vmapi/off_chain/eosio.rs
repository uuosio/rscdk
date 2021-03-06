use crate::structs::*;
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
	return get_vm_api_client().read_action_data().unwrap();
}

///
pub fn action_data_size() -> usize {
	get_vm_api_client().action_data_size().unwrap() as usize
}

///
pub fn require_recipient(name: Name) {
	get_vm_api_client().require_recipient(name.n.into()).unwrap()
}

///
pub fn require_auth(name: Name) {
	get_vm_api_client().require_auth(name.n.into()).unwrap()
}

///
pub fn has_auth(name: Name) -> bool {
	get_vm_api_client().has_auth(name.n.into()).unwrap()
}

///
pub fn require_auth2(name: Name, permission: Name) {
	get_vm_api_client().require_auth2(name.n.into(), permission.n.into()).unwrap()
}

///
pub fn is_account(name: Name) -> bool {
	get_vm_api_client().is_account(name.n.into()).unwrap()
}

///
pub fn send_inline(_serialized_action: &[u8]) {
	get_vm_api_client().send_inline(_serialized_action.to_vec()).unwrap();
}

///
pub fn send_context_free_inline(_serialized_action: &[u8]) {
	get_vm_api_client().send_context_free_inline(_serialized_action.to_vec()).unwrap();
}

///
pub fn publication_time() -> TimePoint {
	let elapsed = get_vm_api_client().publication_time().unwrap();
	return TimePoint{elapsed: elapsed.into()};
}

///
pub fn current_receiver() -> Name {
	let n = get_vm_api_client().current_receiver().unwrap();
	return Name{n: n.into()};
}

///
pub fn eosio_assert(test: bool, msg: &str) {
	get_vm_api_client().eosio_assert(test, msg.into()).unwrap()
}

///
pub fn eosio_assert_message(test: u32, msg: *const u8, msg_len: u32) {
	if test >= 1 {
		return;
	}

	let dst = unsafe {
        slice::from_raw_parts(msg, msg_len as usize)
    };

	match get_vm_api_client().eosio_assert_message(false, dst.into()) {
		Ok(()) => {

		},
		Err(err) => {
			panic!("{:?}", err);
		}
	}
}

///
pub fn eosio_assert_code(test: u32, code: u64) {
	let _test = match test {
		0 => false,
		_ => true,
	};
	get_vm_api_client().eosio_assert_code(_test, code.into()).unwrap()
}


///
pub fn check(test: bool, msg: &str) {
	if test {
		return
	}
	eosio_assert_message(0, msg.as_ptr(), msg.len() as u32);
}

///
pub fn eosio_exit(code: i32) {
	get_vm_api_client().eosio_exit(code).unwrap()
}

///
pub fn current_time() -> TimePoint {
	let elapsed = get_vm_api_client().current_time().unwrap().into();
	return TimePoint{elapsed: elapsed};
}

///
pub fn is_feature_activated(feature_digest: &Checksum256) -> bool {
	get_vm_api_client().is_feature_activated(feature_digest.data.into()).unwrap()
}

///
pub fn get_sender() -> Name {
	let n = get_vm_api_client().get_sender().unwrap().into();
	return Name{n: n};
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

///
pub fn send_deferred(sender_id: &Uint128, payer: Name, serialized_transaction: &[u8], replace_existing: u32) {
    let _sender_id = unsafe {
        slice::from_raw_parts(sender_id as *const Uint128 as *const u8, 16)
    };

	get_vm_api_client().send_deferred(_sender_id.into(), payer.n.into(), serialized_transaction.into(), replace_existing as i32).unwrap()
}

///
pub fn cancel_deferred(sender_id: &Uint128) -> i32 {
    let _sender_id = unsafe {
        slice::from_raw_parts(sender_id as *const Uint128 as *const u8, 16)
    };
	get_vm_api_client().cancel_deferred(_sender_id.into()).unwrap()
}

///
pub fn read_transaction() -> Vec<u8> {
	get_vm_api_client().read_transaction().unwrap()
}

///
pub fn transaction_size() -> usize {
	get_vm_api_client().transaction_size().unwrap() as usize
}

///
pub fn tapos_block_num() -> i32 {
	get_vm_api_client().tapos_block_num().unwrap()
}

///
pub fn tapos_block_prefix() -> i32 {
	get_vm_api_client().tapos_block_prefix().unwrap()
}

///
pub fn expiration() -> u32 {
	get_vm_api_client().expiration().unwrap() as u32
}

///
pub fn get_action(tp: u32, index: u32) -> Vec<u8> {
	get_vm_api_client().get_action(tp as i32, index as i32).unwrap()
}

///
pub fn get_context_free_data(index: u32) -> Vec<u8> {
	get_vm_api_client().get_context_free_data(index as i32).unwrap()
}

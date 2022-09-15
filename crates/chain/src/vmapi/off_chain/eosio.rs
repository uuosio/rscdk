use crate::structs::*;

use crate::transaction::{
	Transaction,
};

use crate::action::{
	PermissionLevel,
};

use crate::privileged::{
	BlockchainParameters,
};

use crate::name::{
	Name,
};

use crate::serializer::{
	Packer,
};

use crate::{
	// vec,
	vec::Vec,
};

use core::slice;

use chaintester::{
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
	let ret = get_vm_api_client().get_active_producers();
	let data = ret.unwrap();
	let mut ret: Vec<Name> = vec![Name::default();data.len()/8];
	let mut i = 0usize;
	for v in &mut ret {
		v.unpack(data[i..].into());
		i += 8;
	}
	ret
}

//permissions.h
/// Checks if a transaction is authorized by a provided set of keys and permissions
pub fn check_transaction_authorization(
	trx: &Transaction,
	perms: &Vec<PermissionLevel>,
	pubkeys: &Vec<PublicKey>
) -> i32 {
	let ret = get_vm_api_client().check_transaction_authorization(trx.pack(), pubkeys.pack(), perms.pack());
    ret.unwrap()
}

/// Checks if a permission is authorized by a provided delay and a provided set of keys and permissions
pub fn check_permission_authorization(
	account: Name,
	permission: Name,
	perms: &Vec<PermissionLevel>,
	pubkeys: &Vec<PublicKey>,
	delay_us: u64
) -> i32 {
	let perms_data = perms.pack();
	let pubkeys_data = pubkeys.pack();
	let ret = get_vm_api_client().check_permission_authorization(account.n.into(), permission.n.into(), pubkeys_data, perms_data, delay_us.into());
    ret.unwrap()
}

///
pub fn get_permission_last_used(account: Name, permission: Name ) -> TimePoint {
	let ret = get_vm_api_client().get_permission_last_used(account.n.into(), permission.n.into());
    let elapsed = ret.unwrap();
	return TimePoint{elapsed: elapsed as u64};
}

///
pub fn get_account_creation_time(account: Name) -> TimePoint {
	let ret = get_vm_api_client().get_account_creation_time(account.n.into());
    let elapsed = ret.unwrap();
	return TimePoint{elapsed: elapsed as u64};
}

///
pub fn read_action_data() -> Vec<u8> {
	let ret = get_vm_api_client().read_action_data();
    ret.unwrap()
}

///
pub fn action_data_size() -> usize {
	let ret = get_vm_api_client().action_data_size();
    ret.unwrap() as usize
}

///
pub fn require_recipient(name: Name) {
	let ret = get_vm_api_client().require_recipient(name.n.into());
    ret.unwrap()
}

///
pub fn require_auth(name: Name) {
	let ret = get_vm_api_client().require_auth(name.n.into());
    ret.unwrap()
}

///
pub fn has_auth(name: Name) -> bool {
	let ret = get_vm_api_client().has_auth(name.n.into());
    ret.unwrap()
}

///
pub fn require_auth2(name: Name, permission: Name) {
	let ret = get_vm_api_client().require_auth2(name.n.into(), permission.n.into());
    ret.unwrap()
}

///
pub fn is_account(name: Name) -> bool {
	let ret = get_vm_api_client().is_account(name.n.into());
    ret.unwrap()
}

///
pub fn send_inline(_serialized_action: &[u8]) {
	let ret = get_vm_api_client().send_inline(_serialized_action.to_vec());
    ret.unwrap();
}

///
pub fn send_context_free_inline(_serialized_action: &[u8]) {
	let ret = get_vm_api_client().send_context_free_inline(_serialized_action.to_vec());
    ret.unwrap();
}

///
pub fn publication_time() -> TimePoint {
	let ret = get_vm_api_client().publication_time();
    let elapsed = ret.unwrap();
	return TimePoint{elapsed: elapsed.into()};
}

///
pub fn current_receiver() -> Name {
	let ret = get_vm_api_client().current_receiver();
    let n = ret.unwrap();
	return Name{n: n.into()};
}

///
pub fn eosio_assert(test: bool, msg: &str) {
	let ret = get_vm_api_client().eosio_assert(test, msg.into());
    ret.unwrap()
}

///
pub fn eosio_assert_message(test: u32, msg: *const u8, msg_len: u32) {
	if test >= 1 {
		return;
	}

	let dst = unsafe {
        slice::from_raw_parts(msg, msg_len as usize)
    };

	let ret = get_vm_api_client().eosio_assert_message(false, dst.into());
    ret.unwrap();
}

///
pub fn eosio_assert_code(test: u32, code: u64) {
	let _test = match test {
		0 => false,
		_ => true,
	};
	let ret = get_vm_api_client().eosio_assert_code(_test, code.into());
    ret.unwrap()
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
	let ret = get_vm_api_client().eosio_exit(code);
    ret.unwrap()
}

///
pub fn current_time() -> TimePoint {
	let ret = get_vm_api_client().current_time();
    let elapsed = ret.unwrap().into();
	return TimePoint{elapsed: elapsed};
}

///
pub fn is_feature_activated(feature_digest: &Checksum256) -> bool {
	let ret = get_vm_api_client().is_feature_activated(feature_digest.data.into());
    ret.unwrap()
}

///
pub fn get_sender() -> Name {
	let ret = get_vm_api_client().get_sender();
    let n = ret.unwrap().into();
	return Name{n: n};
}

/// return resource limits of ram, net, and cpu.
pub fn get_resource_limits(account: Name) -> (i64, i64, i64) {
	let _ret = get_vm_api_client().get_resource_limits(account.n.into());
    let ret = _ret.unwrap();
	(
		ret.ram_bytes.unwrap(),
	    ret.net_weight.unwrap(),
    	ret.cpu_weight.unwrap(),
    )
}

///
pub fn set_resource_limits(account: Name, ram_bytes: i64, net_weight: i64, cpu_weight: i64) {
	let ret = get_vm_api_client().set_resource_limits(account.n.into(), ram_bytes, net_weight, cpu_weight);
    ret.unwrap()
}

//TODO:
///
pub fn set_proposed_producers(producer_keys: &Vec<ProducerKey>) -> i64 {
	let packed = producer_keys.pack();
	let ret = get_vm_api_client().set_proposed_producers(packed);
    ret.unwrap()
}

//TODO:
///
pub fn set_proposed_producers_ex(producer_keys: &Vec<ProducerAuthority>) -> i64 {
	let packed = producer_keys.pack();
	let ret = get_vm_api_client().set_proposed_producers_ex(1u64.into(), packed);
    ret.unwrap()
}

///
pub fn is_privileged(account: Name) -> bool {
	let ret = get_vm_api_client().is_privileged(account.n.into());
    ret.unwrap()
}

///
pub fn set_privileged(account: Name, is_priv: bool) {
	let ret = get_vm_api_client().set_privileged(account.n.into(), is_priv);
    ret.unwrap();
}

///
pub fn set_blockchain_parameters(params: &BlockchainParameters) {
	let data = params.pack();
	let ret = get_vm_api_client().set_blockchain_parameters_packed(data);
    ret.unwrap();
}

///
pub fn get_blockchain_parameters() -> BlockchainParameters {
	let mut params = BlockchainParameters::default();
	let ret = get_vm_api_client().get_blockchain_parameters_packed();
    let data = ret.unwrap();
	params.unpack(&data);
	return params;
}

///
pub fn preactivate_feature(_feature_digest:  &Checksum256) {
}

///
pub fn send_deferred(sender_id: &Uint128, payer: Name, serialized_transaction: &[u8], replace_existing: u32) {
    let _sender_id = unsafe {
        slice::from_raw_parts(sender_id as *const Uint128 as *const u8, 16)
    };

	let ret = get_vm_api_client().send_deferred(_sender_id.into(), payer.n.into(), serialized_transaction.into(), replace_existing as i32);
    ret.unwrap()
}

///
pub fn cancel_deferred(sender_id: &Uint128) -> i32 {
    let _sender_id = unsafe {
        slice::from_raw_parts(sender_id as *const Uint128 as *const u8, 16)
    };
	let ret = get_vm_api_client().cancel_deferred(_sender_id.into());
    ret.unwrap()
}

///
pub fn read_transaction() -> Transaction {
	let ret = get_vm_api_client().read_transaction();
    let data = ret.unwrap();
	let mut ret = Transaction::default();
	ret.unpack(&data);
	return ret;
}

///
pub fn transaction_size() -> usize {
	let ret = get_vm_api_client().transaction_size();
    ret.unwrap() as usize
}

///
pub fn tapos_block_num() -> i32 {
	let ret = get_vm_api_client().tapos_block_num();
    ret.unwrap()
}

///
pub fn tapos_block_prefix() -> i32 {
	let ret = get_vm_api_client().tapos_block_prefix();
    ret.unwrap()
}

///
pub fn expiration() -> u32 {
	let ret = get_vm_api_client().expiration();
    ret.unwrap() as u32
}

///
pub fn get_action(tp: u32, index: u32) -> Vec<u8> {
	let ret = get_vm_api_client().get_action(tp as i32, index as i32);
    ret.unwrap()
}

///
pub fn get_context_free_data(index: u32) -> Vec<u8> {
	let ret = get_vm_api_client().get_context_free_data(index as i32);
    ret.unwrap()
}

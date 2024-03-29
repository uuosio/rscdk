use crate::structs::*;
use crate::vmapi::system;

use crate::transaction::{
	Transaction,
};

use crate::action::{
	PermissionLevel,
};

use crate::privileged::{
	BlockchainParameters,
};

use crate::name::{ Name };

use crate::serializer::{
	Packer,
	Encoder,
};

use crate::{
	vec,
	vec::Vec,
};

mod intrinsics {
    use crate::structs::*;
    extern "C" {
		pub fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
		// pub fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
		// pub fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8;
		// pub fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;

		pub fn get_active_producers(producers: *const u8, datalen: u32) -> u32;
	

		// permission.h
        pub fn check_transaction_authorization(
			trx_data: *const u8, trx_size: u32,
			pubkeys_data: *const u8, pubkeys_size: u32,
            perms_data: *const u8, perms_size: u32,
        ) -> i32;

		pub fn check_permission_authorization(
			account: u64, permission: u64,
			pubkeys_data: *const u8, pubkeys_size: u32,
			perms_data: *const u8, perms_size: u32,
			delay_us: u64
		) -> i32;

		pub fn get_permission_last_used( account: u64, permission: u64 ) -> u64;	
		pub fn get_account_creation_time( account: u64 ) -> u64;
	
		pub fn read_action_data( msg: *mut u8, len: u32 ) -> u32;
	
		pub fn action_data_size() -> u32;
	
		pub fn require_recipient( name: u64 );
	
		pub fn require_auth( name: u64 );
	
		pub fn has_auth( name: u64 ) -> bool;
	
		pub fn require_auth2( name: u64, permission: u64 );
	
		pub fn is_account( name: u64 ) -> bool;
	
		pub fn send_inline(serialized_action: *const u8, size: usize);
	
		pub fn send_context_free_inline(serialized_action: *const u8, size: usize);
	
		pub fn publication_time() -> u64;
	
		pub fn current_receiver() -> u64;
	
		// #[link_name = "eosio_assert"]
		// fn eosio_assert(test: u32, msg: *const u8);
	
		pub fn eosio_exit(code: i32);
	
		pub fn current_time() -> u64;
	
		pub fn is_feature_activated(feature_digest: *const Checksum256) -> bool;
	
		pub fn get_sender() -> u64;
	
		pub fn get_resource_limits( account: u64, ram_bytes: *mut i64, net_weight: *mut i64, cpu_weight: *mut i64 );
	
		pub fn set_resource_limits( account: u64, ram_bytes: i64, net_weight: i64, cpu_weight: i64);
	
		pub fn set_proposed_producers(producer_data: *const u8, producer_data_size: usize ) -> i64;
	
		pub fn set_proposed_producers_ex(producer_data_format: u64, producer_data: *const u8, producer_data_size: u32) -> i64;
	
		pub fn is_privileged( account: u64 ) -> bool;
	
		pub fn set_privileged( account: u64, is_priv: bool);
	
		pub fn set_blockchain_parameters_packed(data: *const u8, datalen: u32);
	
		pub fn get_blockchain_parameters_packed( data: *mut u8, datalen: u32) -> u32;
	
		pub fn preactivate_feature(feature_digest: *const Checksum256);
	
		pub fn send_deferred(sender_id: *const Uint128, payer: u64, serialized_transaction: *const u8, size: usize, replace_existing: u32);
	
		pub fn cancel_deferred(sender_id: *const Uint128) -> i32;
	
		pub fn read_transaction(buffer: *mut u8, size: usize) -> usize;
	
		pub fn transaction_size() -> usize;	
	
		pub fn tapos_block_num() -> i32;
	
		pub fn tapos_block_prefix() -> i32;
	
		pub fn expiration() -> u32;
	
		pub fn get_action(_type: u32, index: u32, buff: *mut u8, size: usize) -> i32;
	
		pub fn get_context_free_data(index: u32, buff: *mut u8, size: usize) -> i32;
	}
}


///
pub fn eosio_memcpy( dst: *mut u8, src: *const u8, length: usize) {
	unsafe {
		intrinsics::memcpy(dst, src, length);
	}
}

///
pub fn slice_copy( dst: &mut [u8], src: &[u8]) {
	check(dst.len() == src.len(), "slice_copy: length not the same!");
	eosio_memcpy(dst.as_mut_ptr(), src.as_ptr(), dst.len());
}

///
pub fn get_active_producers() -> Vec<Name> {
	unsafe {
		let datalen = intrinsics::get_active_producers(0 as *const u8, 0);
		let data: Vec<Name> = vec![Name{n: 0}; (datalen/8) as usize];
		intrinsics::get_active_producers(data.as_ptr() as *const u8 as *mut u8, datalen);
		return data;
	}
}

//permissions.h
/// Checks if a transaction is authorized by a provided set of keys and permissions
pub fn check_transaction_authorization(
	trx: &Transaction,
	perms: &Vec<PermissionLevel>,
	pubkeys: &Vec<PublicKey>
) -> i32 {
	let trx_data = Encoder::pack(trx);
	let perms_data = Encoder::pack(perms);
	let pubkeys_data = Encoder::pack(pubkeys);
	unsafe {
		return intrinsics::check_transaction_authorization(
			trx_data.as_ptr(), trx_data.len() as u32,
			pubkeys_data.as_ptr(), pubkeys_data.len() as u32,
			perms_data.as_ptr(), perms_data.len() as u32
		);	
	}
}

/// Checks if a permission is authorized by a provided delay and a provided set of keys and permissions
pub fn check_permission_authorization(
	account: Name,
	permission: Name,
	perms: &Vec<PermissionLevel>,
	pubkeys: &Vec<PublicKey>,
	delay_us: u64
) -> i32 {
	let perms_data = Encoder::pack(perms);
	let pubkeys_data = Encoder::pack(pubkeys);
	unsafe {
		return intrinsics::check_permission_authorization(
			account.n.into(), permission.n.into(),
			pubkeys_data.as_ptr(), pubkeys_data.len() as u32,
			perms_data.as_ptr(), perms_data.len() as u32,
			delay_us,
		);
	}
}

///
pub fn get_permission_last_used( account: Name, permission: Name ) -> TimePoint {
	unsafe {
		let elapsed = intrinsics::get_permission_last_used(account.value(), permission.value());
		return TimePoint{elapsed: elapsed};
	}
}

///
pub fn get_account_creation_time( account: Name ) -> TimePoint {
	unsafe {
		let elapsed = intrinsics::get_account_creation_time(account.value());
		return TimePoint{elapsed: elapsed};
	}
}

///
pub fn read_action_data() -> Vec<u8> {
	unsafe {
		let size = intrinsics::action_data_size();
		if size <= 0 {
			return Vec::new();
		}
		let mut data: Vec<u8> = vec![0; size as usize];
		intrinsics::read_action_data(data.as_mut_ptr(), size);
		return data;
	}
}

///
pub fn action_data_size() -> usize {
	unsafe {
		return intrinsics::action_data_size() as usize;
	}
}

///
pub fn require_recipient(name: Name) {
	unsafe {
		intrinsics::require_recipient(name.value());
	}
}

///
pub fn require_auth(name: Name) {
	unsafe {
		intrinsics::require_auth(name.value());
	}
}

///
pub fn has_auth( name: Name ) -> bool {
	unsafe {
		return intrinsics::has_auth(name.value());
	}
}

///
pub fn require_auth2( name: Name, permission: Name ) {
	unsafe {
		intrinsics::require_auth2(name.value(), permission.value());
	}
}

///
pub fn is_account( name: Name ) -> bool {
	unsafe {
		return intrinsics::is_account(name.value());
	}
}

///
pub fn send_inline(serialized_action: &[u8]) {
	unsafe {
		intrinsics::send_inline(serialized_action.as_ptr(), serialized_action.len());
	}
}

///
pub fn send_context_free_inline(serialized_action: &[u8]) {
	unsafe {
		intrinsics::send_context_free_inline(serialized_action.as_ptr(), serialized_action.len());
	}
}

///
pub fn publication_time() -> TimePoint {
	unsafe {
		let elapsed = intrinsics::publication_time();
		return TimePoint{elapsed: elapsed};
	}
}

///
pub fn current_receiver() -> Name {
	unsafe {
		let name = intrinsics::current_receiver();
		return Name{n: name};
	}
}

///
pub fn eosio_assert(test: bool, msg: &str) {
	if !test {
		system::eosio_assert_message(test as u32, msg.as_ptr(), msg.len() as u32);
	}
}

///
pub fn check(test: bool, msg: &str) {
	if !test {
		system::eosio_assert_message(0 as u32, msg.as_ptr(), msg.len() as u32);
	}
}

///
pub fn eosio_assert_code(test: u32, code: u64) {
	system::eosio_assert_code(test, code);
}

///
pub fn eosio_exit(code: i32) {
	unsafe {
		intrinsics::eosio_exit(code);
	}
}

///
pub fn current_time() -> TimePoint {
	unsafe {
		let elapsed = intrinsics::current_time();
		return TimePoint{elapsed: elapsed};
	}
}

///
pub fn is_feature_activated(feature_digest: &Checksum256) -> bool {
	unsafe {
		return intrinsics::is_feature_activated(feature_digest);
	}
}

///
pub fn get_sender() -> Name {
	unsafe {
		let name = intrinsics::get_sender();
		return Name{n: name};
	}
}

/// return resource limits of ram, net, and cpu.
pub fn get_resource_limits( account: Name) -> (i64, i64, i64) {
	unsafe {
		let mut ram_bytes = 0;
		let mut net_weight = 0;
		let mut cpu_weight = 0;
		intrinsics::get_resource_limits(account.value(), &mut ram_bytes, &mut net_weight, &mut cpu_weight);
		return (ram_bytes, net_weight, cpu_weight);
	}
}

///
pub fn set_resource_limits( account: Name, ram_bytes: i64, net_weight: i64, cpu_weight: i64) {
	unsafe {
		intrinsics::set_resource_limits(account.value(), ram_bytes, net_weight, cpu_weight);
	}
}

//TODO:
///
pub fn set_proposed_producers(producer_keys: &Vec<ProducerKey>) -> i64 {
	let packed = Encoder::pack(producer_keys);
	unsafe {
		intrinsics::set_proposed_producers(packed.as_ptr(), packed.len())
	}
}

//TODO:
///
pub fn set_proposed_producers_ex(producer_data_format: u64, producer_keys: &Vec<ProducerAuthority>) -> i64 {
	let packed = Encoder::pack(producer_keys);
	unsafe {
		intrinsics::set_proposed_producers_ex(producer_data_format, packed.as_ptr(), packed.len() as u32)
	}
}

///
pub fn is_privileged(account: Name) -> bool {
	unsafe {
		return intrinsics::is_privileged(account.value());
	}
}

///
pub fn set_privileged( account: Name, is_priv: bool) {
	unsafe {
		intrinsics::set_privileged(account.value(), is_priv);
	}
}

///
pub fn set_blockchain_parameters(params: &BlockchainParameters) {
	let data = Encoder::pack(params);
	unsafe {
		intrinsics::set_blockchain_parameters_packed(data.as_ptr(), data.len() as u32);
	}
}

///
pub fn get_blockchain_parameters() -> BlockchainParameters {
	unsafe {
		let size = intrinsics::get_blockchain_parameters_packed(0 as *mut u8, 0);
		let mut data: Vec<u8> = vec![0; size as usize];
		intrinsics::get_blockchain_parameters_packed(data.as_mut_ptr(), data.len() as u32);
		let mut ret = BlockchainParameters::default();
		ret.unpack(&data);
		return ret;
	}
}

///
pub fn preactivate_feature(feature_digest:  &Checksum256) {
	unsafe {
		intrinsics::preactivate_feature(feature_digest);
	}
}

//TODO:
///
pub fn send_deferred(sender_id: &Uint128, payer: Name, serialized_transaction: &[u8], replace_existing: u32) {
	unsafe {
		intrinsics::send_deferred(sender_id, payer.value(), serialized_transaction.as_ptr(), serialized_transaction.len(), replace_existing);
	}
}

///
pub fn cancel_deferred(sender_id: &Uint128) -> i32 {
	unsafe {
		return intrinsics::cancel_deferred(sender_id);
	}
}

///
pub fn read_transaction() -> Transaction {
	unsafe {
		let size = intrinsics::transaction_size();
		let mut data: Vec<u8> = vec![0; size];
		intrinsics::read_transaction(data.as_mut_ptr(), data.len());
		let mut ret = Transaction::default();
		ret.unpack(&data);
		return ret;
	}
}

///
pub fn transaction_size() -> usize {
	return unsafe { intrinsics::transaction_size() };
}

///
pub fn tapos_block_num() -> u32 {
	return unsafe { intrinsics::tapos_block_num() as u32 };
}

///
pub fn tapos_block_prefix() -> u32 {
	return unsafe { intrinsics::tapos_block_prefix() as u32 };
}

///
pub fn expiration() -> u32 {
	return unsafe { intrinsics::expiration() };
}

///
pub fn get_action(_type: u32, index: u32) -> Vec<u8> {
	unsafe {
		let size = intrinsics::get_action(_type, index, 0 as *mut u8, 0);
		if size <= 0 {
			return Vec::new();
		}
		let mut data: Vec<u8> = vec![0; size as usize];
		intrinsics::get_action(_type, index, data.as_mut_ptr(), data.len());
		return data;
	}
}

///
pub fn get_context_free_data(index: u32) -> Vec<u8> {
	unsafe {
		let size = intrinsics::get_context_free_data(index, 0 as *mut u8, 0);
		if size <= 0 {
			return Vec::new();
		}
		let mut data: Vec<u8> = vec![0; size as usize];
		intrinsics::get_context_free_data(index, data.as_mut_ptr(), data.len());
		return data;
	}
}

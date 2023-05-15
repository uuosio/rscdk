use crate::{
	// vec,
	vec::Vec,
};

use chaintester::{
    get_vm_api_client,
	interfaces::{
		TApplySyncClient,
	}
};

use crate::name::{
	Name,
};

// void set_action_return_value(const char *data, uint32_t data_size);
pub fn set_action_return_value(data: Vec<u8>) {
    let ret = get_vm_api_client().set_action_return_value(data);
    ret.unwrap();
}

// uint32_t get_code_hash(capi_name account, uint32_t struct_version, char* packed_result, uint32_t packed_result_len)
pub fn get_code_hash(account: Name, struct_version: u32) -> Vec<u8> {
    let ret = get_vm_api_client().get_code_hash(account.n.into(), struct_version as i64);
    ret.unwrap()
}

// uint32_t get_block_num();
pub fn get_block_num() -> u32 {
    let ret = get_vm_api_client().get_block_num();
    ret.unwrap() as u32
}

// void sha3( const char* data, uint32_t data_len, char* hash, uint32_t hash_len, int32_t keccak );
// int32_t blake2_f( uint32_t rounds, const char* state, uint32_t state_len, const char* msg, uint32_t msg_len, 
//                 const char* t0_offset, uint32_t t0_len, const char* t1_offset, uint32_t t1_len, int32_t final, char* result, uint32_t result_len);
// int32_t k1_recover( const char* sig, uint32_t sig_len, const char* dig, uint32_t dig_len, char* pub, uint32_t pub_len);
// int32_t alt_bn128_add( const char* op1, uint32_t op1_len, const char* op2, uint32_t op2_len, char* result, uint32_t result_len);
// int32_t alt_bn128_mul( const char* g1, uint32_t g1_len, const char* scalar, uint32_t scalar_len, char* result, uint32_t result_len);
// int32_t alt_bn128_pair( const char* pairs, uint32_t pairs_len);
// int32_t mod_exp( const char* base, uint32_t base_len, const char* exp, uint32_t exp_len, const char* mod, uint32_t mod_len, char* result, uint32_t result_len);

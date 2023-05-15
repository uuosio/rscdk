use crate::{
	// vec,
	vec::Vec,
};

use crate::name::{
	Name,
};

mod intrinsics {
    extern "C" {
        pub fn set_action_return_value(data: *const u8, data_size: u32);
        // uint32_t get_code_hash(capi_name account, uint32_t struct_version, char* packed_result, uint32_t packed_result_len);
        pub fn get_code_hash(account: u64, struct_version: u32, packed_result: *mut u8, packed_result_len: u32) -> u32;

        //uint32_t get_block_num();
        pub fn get_block_num() -> u32;
        // void sha3( const char* data, uint32_t data_len, char* hash, uint32_t hash_len, int32_t keccak );
        // int32_t blake2_f( uint32_t rounds, const char* state, uint32_t state_len, const char* msg, uint32_t msg_len, 
        //                 const char* t0_offset, uint32_t t0_len, const char* t1_offset, uint32_t t1_len, int32_t final, char* result, uint32_t result_len);
        // int32_t k1_recover( const char* sig, uint32_t sig_len, const char* dig, uint32_t dig_len, char* pub, uint32_t pub_len);
        // int32_t alt_bn128_add( const char* op1, uint32_t op1_len, const char* op2, uint32_t op2_len, char* result, uint32_t result_len);
        // int32_t alt_bn128_mul( const char* g1, uint32_t g1_len, const char* scalar, uint32_t scalar_len, char* result, uint32_t result_len);
        // int32_t alt_bn128_pair( const char* pairs, uint32_t pairs_len);
        // int32_t mod_exp( const char* base, uint32_t base_len, const char* exp, uint32_t exp_len, const char* mod, uint32_t mod_len, char* result, uint32_t result_len);
    }
}

// void set_action_return_value(const char *data, uint32_t data_size);
pub fn set_action_return_value(data: Vec<u8>) {
    unsafe {
        intrinsics::set_action_return_value(data.as_ptr(), data.len() as u32);
    }
}

// uint32_t get_code_hash(capi_name account, uint32_t struct_version, char* packed_result, uint32_t packed_result_len);
pub fn get_code_hash(account: Name, struct_version: u32) -> Vec<u8> {
    let mut packed_result = [0u8; 43];
    unsafe {
        let ret = intrinsics::get_code_hash(account.n, struct_version, packed_result.as_mut_ptr(), 43u32);
        crate::vmapi::eosio::eosio_assert(ret == 43u32, "bad get_code_hash return size");
    }
    packed_result.to_vec()
}

// uint32_t get_block_num();
pub fn get_block_num() -> u32 {
    unsafe {
        return intrinsics::get_block_num();
    }
}

// void sha3( const char* data, uint32_t data_len, char* hash, uint32_t hash_len, int32_t keccak );
// int32_t blake2_f( uint32_t rounds, const char* state, uint32_t state_len, const char* msg, uint32_t msg_len, 
//                 const char* t0_offset, uint32_t t0_len, const char* t1_offset, uint32_t t1_len, int32_t final, char* result, uint32_t result_len);
// int32_t k1_recover( const char* sig, uint32_t sig_len, const char* dig, uint32_t dig_len, char* pub, uint32_t pub_len);
// int32_t alt_bn128_add( const char* op1, uint32_t op1_len, const char* op2, uint32_t op2_len, char* result, uint32_t result_len);
// int32_t alt_bn128_mul( const char* g1, uint32_t g1_len, const char* scalar, uint32_t scalar_len, char* result, uint32_t result_len);
// int32_t alt_bn128_pair( const char* pairs, uint32_t pairs_len);
// int32_t mod_exp( const char* base, uint32_t base_len, const char* exp, uint32_t exp_len, const char* mod, uint32_t mod_len, char* result, uint32_t result_len);

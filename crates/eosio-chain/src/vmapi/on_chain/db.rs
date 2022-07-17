use crate::structs::*;

mod intrinsics {
    use crate::structs::*;
    extern "C" {
        ///
        pub fn db_store_i64(scope: u64, table: u64, payer: u64, id: u64,  data: *const u8, len: u32) -> i32;
        ///
        pub fn db_update_i64(iterator: i32, payer: u64, data: *const u8, len: u32);
        ///
        pub fn db_remove_i64(iterator: i32);
        ///
        pub fn db_get_i64(iterator: i32, data: *mut u8, len: u32) -> i32;
        ///
        pub fn db_next_i64(iterator: i32, primary: *mut u64) -> i32;
        ///
        pub fn db_previous_i64(iterator: i32, primary: *mut u64) -> i32;
        ///
        pub fn db_find_i64(code: u64, scope: u64, table: u64, id: u64) -> i32;
        ///
        pub fn db_lowerbound_i64(code: u64, scope: u64, table: u64, id: u64) -> i32;
        ///
        pub fn db_upperbound_i64(code: u64, scope: u64, table: u64, id: u64) -> i32;
        ///
        pub fn db_end_i64(code: u64, scope: u64, table: u64) -> i32;
    
        ///
        pub fn db_idx64_store(scope: u64, table: u64, payer: u64, id: u64, secondary: *const u64) -> i32;
        ///
        pub fn db_idx64_update(iterator: i32, payer: u64, secondary: *const u64);
        ///
        pub fn db_idx64_remove(iterator: i32);
        ///
        pub fn db_idx64_next(iterator: i32, primary: *mut u64) -> i32;
        ///
        pub fn db_idx64_previous(iterator: i32, primary: *mut u64) -> i32;
        ///
        pub fn db_idx64_find_primary(code: u64, scope: u64, table: u64, secondary: *mut u64, primary: u64) -> i32;
        ///
        pub fn db_idx64_find_secondary(code: u64, scope: u64, table: u64, secondary: *const u64, primary: *mut u64) -> i32;
        ///
        pub fn db_idx64_lowerbound(code: u64, scope: u64, table: u64, secondary: *mut u64, primary: *mut u64) -> i32;
        ///
        pub fn db_idx64_upperbound(code: u64, scope: u64, table: u64, secondary: *mut u64, primary: *mut u64) -> i32;
        ///
        pub fn db_idx64_end(code: u64, scope: u64, table: u64) -> i32;
    
        ///
        pub fn db_idx128_store(scope: u64, table: u64, payer: u64, id: u64, secondary: *const Uint128) -> i32;
        ///
        pub fn db_idx128_update(iterator: i32, payer: u64, secondary: *const Uint128);
        ///
        pub fn db_idx128_remove(iterator: i32);
        ///
        pub fn db_idx128_next(iterator: i32, primary: *mut u64) -> i32;
        ///
        pub fn db_idx128_previous(iterator: i32, primary: *mut u64) -> i32;
        ///
        pub fn db_idx128_find_primary(code: u64, scope: u64, table: u64, secondary: *mut Uint128 , primary: u64) -> i32;
        ///
        pub fn db_idx128_find_secondary(code: u64, scope: u64, table: u64, secondary: *const Uint128, primary: *mut u64) -> i32;
        ///
        pub fn db_idx128_lowerbound(code: u64, scope: u64, table: u64, secondary: *const Uint128, primary: *mut u64) -> i32;
        ///
        pub fn db_idx128_upperbound(code: u64, scope: u64, table: u64, secondary: *const Uint128, primary: *mut u64) -> i32;
        ///
        pub fn db_idx128_end(code: u64, scope: u64, table: u64) -> i32;
    
        ///
        pub fn db_idx256_store(scope: u64, table: u64, payer: u64, id: u64, data: *const Uint128, data_len: u32 ) -> i32;
        ///
        pub fn db_idx256_update(iterator: i32, payer: u64, data: *const Uint128, data_len: u32);
        ///
        pub fn db_idx256_remove(iterator: i32);
        ///
        pub fn db_idx256_next(iterator: i32, primary: *mut u64) -> i32;
        ///
        pub fn db_idx256_previous(iterator: i32, primary: *mut u64) -> i32;
        ///
        pub fn db_idx256_find_primary(code: u64, scope: u64, table: u64, data: *mut Uint128, data_len: u32, primary: u64) -> i32;
        ///
        pub fn db_idx256_find_secondary(code: u64, scope: u64, table: u64, data: *const Uint128, data_len: u32, primary: *mut u64) -> i32;
        ///
        pub fn db_idx256_lowerbound(code: u64, scope: u64, table: u64, data: *mut Uint128, data_len: u32, primary: *mut u64) -> i32;
        ///
        pub fn db_idx256_upperbound(code: u64, scope: u64, table: u64, data: *mut Uint128, data_len: u32, primary: *mut u64) -> i32;
        ///
        pub fn db_idx256_end(code: u64, scope: u64, table: u64) -> i32;
    
        ///
        pub fn db_idx_double_store(scope: u64, table: u64, payer: u64, id: u64, secondary: *const f64) -> i32;
        ///
        pub fn db_idx_double_update(iterator: i32, payer: u64, secondary: *const f64);
        ///
        pub fn db_idx_double_remove(iterator: i32);
        ///
        pub fn db_idx_double_next(iterator: i32, primary: *mut u64) -> i32;
        ///
        pub fn db_idx_double_previous(iterator: i32, primary: *mut u64) -> i32;
        ///
        pub fn db_idx_double_find_primary(code: u64, scope: u64, table: u64, secondary: *mut f64, primary: u64) -> i32;
        ///
        pub fn db_idx_double_find_secondary(code: u64, scope: u64, table: u64, secondary: *const f64, primary: *mut u64) -> i32;
        ///
        pub fn db_idx_double_lowerbound(code: u64, scope: u64, table: u64, secondary: *mut f64, primary: *mut u64) -> i32;
        ///
        pub fn db_idx_double_upperbound(code: u64, scope: u64, table: u64, secondary: *mut f64, primary: *mut u64) -> i32;
        ///
        pub fn db_idx_double_end(code: u64, scope: u64, table: u64) -> i32;
    
        ///
        pub fn db_idx_long_double_store(scope: u64, table: u64, payer: u64, id: u64, secondary: *const Float128) -> i32;
        ///
        pub fn db_idx_long_double_update(iterator: i32, payer: u64, secondary: *const Float128);
        ///
        pub fn db_idx_long_double_remove(iterator: i32);
        ///
        pub fn db_idx_long_double_next(iterator: i32, primary: *mut u64) -> i32;
        ///
        pub fn db_idx_long_double_previous(iterator: i32, primary: *mut u64) -> i32;
        ///
        pub fn db_idx_long_double_find_primary(code: u64, scope: u64, table: u64, secondary: *const Float128, primary: u64) -> i32;
        ///
        pub fn db_idx_long_double_find_secondary(code: u64, scope: u64, table: u64, secondary: *const Float128, primary: *mut u64) -> i32;
        ///
        pub fn db_idx_long_double_lowerbound(code: u64, scope: u64, table: u64, secondary: *const Float128, primary: *mut u64) -> i32;
        ///
        pub fn db_idx_long_double_upperbound(code: u64, scope: u64, table: u64, secondary: *const Float128, primary: *mut u64) -> i32;
        ///
        pub fn db_idx_long_double_end(code: u64, scope: u64, table: u64) -> i32;
    }    
}

///
pub fn db_store_i64(scope: u64, table: u64, payer: u64, id: u64,  data: *const u8, len: u32) -> i32{
    unsafe {
        return intrinsics::db_store_i64(scope, table, payer, id, data, len);
    }
}

///
pub fn db_update_i64(iterator: i32, payer: u64, data: *const u8, len: u32){
    unsafe {
        return intrinsics::db_update_i64(iterator, payer, data, len);
    }
}

///
pub fn db_remove_i64(iterator: i32){
    unsafe {
        return intrinsics::db_remove_i64(iterator);
    }
}

///
pub fn db_get_i64(iterator: i32) -> Vec<u8> {
    unsafe {
        let size = intrinsics::db_get_i64(iterator, 0 as *mut u8, 0);
        if size == 0 {
            return Vec::new();
        }
        let data = vec![0u8, size];
        intrinsics::db_get_i64(iterator, data.as_ptr(), size);
        return data
    }
}

///
pub fn db_next_i64(iterator: i32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_next_i64(iterator, primary);
    }
}

///
pub fn db_previous_i64(iterator: i32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_previous_i64(iterator, primary);
    }
}

///
pub fn db_find_i64(code: u64, scope: u64, table: u64, id: u64) -> i32{
    unsafe {
        return intrinsics::db_find_i64(code, scope, table, id);
    }
}

///
pub fn db_lowerbound_i64(code: u64, scope: u64, table: u64, id: u64) -> i32{
    unsafe {
        return intrinsics::db_lowerbound_i64(code, scope, table, id);
    }
}

///
pub fn db_upperbound_i64(code: u64, scope: u64, table: u64, id: u64) -> i32{
    unsafe {
        return intrinsics::db_upperbound_i64(code, scope, table, id);
    }
}

///
pub fn db_end_i64(code: u64, scope: u64, table: u64) -> i32{
    unsafe {
        return intrinsics::db_end_i64(code, scope, table);
    }
}

///
pub fn db_idx64_store(scope: u64, table: u64, payer: u64, id: u64, secondary: *const u64) -> i32{
    unsafe {
        return intrinsics::db_idx64_store(scope, table, payer, id, secondary);
    }
}

///
pub fn db_idx64_update(iterator: i32, payer: u64, secondary: *const u64){
    unsafe {
        return intrinsics::db_idx64_update(iterator, payer, secondary);
    }
}

///
pub fn db_idx64_remove(iterator: i32){
    unsafe {
        return intrinsics::db_idx64_remove(iterator);
    }
}

///
pub fn db_idx64_next(iterator: i32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx64_next(iterator, primary);
    }
}

///
pub fn db_idx64_previous(iterator: i32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx64_previous(iterator, primary);
    }
}

///
pub fn db_idx64_find_primary(code: u64, scope: u64, table: u64, secondary: *mut u64, primary: u64) -> i32{
    unsafe {
        return intrinsics::db_idx64_find_primary(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx64_find_secondary(code: u64, scope: u64, table: u64, secondary: *const u64, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx64_find_secondary(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx64_lowerbound(code: u64, scope: u64, table: u64, secondary: *mut u64, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx64_lowerbound(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx64_upperbound(code: u64, scope: u64, table: u64, secondary: *mut u64, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx64_upperbound(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx64_end(code: u64, scope: u64, table: u64) -> i32{
    unsafe {
        return intrinsics::db_idx64_end(code, scope, table);
    }
}

///
pub fn db_idx128_store(scope: u64, table: u64, payer: u64, id: u64, secondary: *const Uint128) -> i32{
    unsafe {
        return intrinsics::db_idx128_store(scope, table, payer, id, secondary);
    }
}

///
pub fn db_idx128_update(iterator: i32, payer: u64, secondary: *const Uint128){
    unsafe {
        return intrinsics::db_idx128_update(iterator, payer, secondary);
    }
}

///
pub fn db_idx128_remove(iterator: i32){
    unsafe {
        return intrinsics::db_idx128_remove(iterator);
    }
}

///
pub fn db_idx128_next(iterator: i32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx128_next(iterator, primary);
    }
}

///
pub fn db_idx128_previous(iterator: i32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx128_previous(iterator, primary);
    }
}

///
pub fn db_idx128_find_primary(code: u64, scope: u64, table: u64, secondary: *mut Uint128 , primary: u64) -> i32{
    unsafe {
        return intrinsics::db_idx128_find_primary(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx128_find_secondary(code: u64, scope: u64, table: u64, secondary: *const Uint128, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx128_find_secondary(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx128_lowerbound(code: u64, scope: u64, table: u64, secondary: *const Uint128, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx128_lowerbound(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx128_upperbound(code: u64, scope: u64, table: u64, secondary: *const Uint128, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx128_upperbound(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx128_end(code: u64, scope: u64, table: u64) -> i32{
    unsafe {
        return intrinsics::db_idx128_end(code, scope, table);
    }
}

///
pub fn db_idx256_store(scope: u64, table: u64, payer: u64, id: u64, data: *const Uint128, data_len: u32 ) -> i32{
    unsafe {
        return intrinsics::db_idx256_store(scope, table, payer, id, data, data_len);
    }
}

///
pub fn db_idx256_update(iterator: i32, payer: u64, data: *const Uint128, data_len: u32){
    unsafe {
        return intrinsics::db_idx256_update(iterator, payer, data, data_len);
    }
}

///
pub fn db_idx256_remove(iterator: i32){
    unsafe {
        return intrinsics::db_idx256_remove(iterator);
    }
}

///
pub fn db_idx256_next(iterator: i32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx256_next(iterator, primary);
    }
}

///
pub fn db_idx256_previous(iterator: i32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx256_previous(iterator, primary);
    }
}

///
pub fn db_idx256_find_primary(code: u64, scope: u64, table: u64, data: *mut Uint128, data_len: u32, primary: u64) -> i32{
    unsafe {
        return intrinsics::db_idx256_find_primary(code, scope, table, data, data_len, primary);
    }
}

///
pub fn db_idx256_find_secondary(code: u64, scope: u64, table: u64, data: *const Uint128, data_len: u32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx256_find_secondary(code, scope, table, data, data_len, primary);
    }
}

///
pub fn db_idx256_lowerbound(code: u64, scope: u64, table: u64, data: *mut Uint128, data_len: u32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx256_lowerbound(code, scope, table, data, data_len, primary);
    }
}

///
pub fn db_idx256_upperbound(code: u64, scope: u64, table: u64, data: *mut Uint128, data_len: u32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx256_upperbound(code, scope, table, data, data_len, primary);
    }
}

///
pub fn db_idx256_end(code: u64, scope: u64, table: u64) -> i32{
    unsafe {
        return intrinsics::db_idx256_end(code, scope, table);
    }
}

///
pub fn db_idx_double_store(scope: u64, table: u64, payer: u64, id: u64, secondary: *const f64) -> i32{
    unsafe {
        return intrinsics::db_idx_double_store(scope, table, payer, id, secondary);
    }
}

///
pub fn db_idx_double_update(iterator: i32, payer: u64, secondary: *const f64){
    unsafe {
        return intrinsics::db_idx_double_update(iterator, payer, secondary);
    }
}

///
pub fn db_idx_double_remove(iterator: i32){
    unsafe {
        return intrinsics::db_idx_double_remove(iterator);
    }
}

///
pub fn db_idx_double_next(iterator: i32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx_double_next(iterator, primary);
    }
}

///
pub fn db_idx_double_previous(iterator: i32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx_double_previous(iterator, primary);
    }
}

///
pub fn db_idx_double_find_primary(code: u64, scope: u64, table: u64, secondary: *mut f64, primary: u64) -> i32{
    unsafe {
        return intrinsics::db_idx_double_find_primary(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx_double_find_secondary(code: u64, scope: u64, table: u64, secondary: *const f64, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx_double_find_secondary(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx_double_lowerbound(code: u64, scope: u64, table: u64, secondary: *mut f64, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx_double_lowerbound(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx_double_upperbound(code: u64, scope: u64, table: u64, secondary: *mut f64, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx_double_upperbound(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx_double_end(code: u64, scope: u64, table: u64) -> i32{
    unsafe {
        return intrinsics::db_idx_double_end(code, scope, table);
    }
}

///
pub fn db_idx_long_double_store(scope: u64, table: u64, payer: u64, id: u64, secondary: *const Float128) -> i32{
    unsafe {
        return intrinsics::db_idx_long_double_store(scope, table, payer, id, secondary);
    }
}

///
pub fn db_idx_long_double_update(iterator: i32, payer: u64, secondary: *const Float128){
    unsafe {
        return intrinsics::db_idx_long_double_update(iterator, payer, secondary);
    }
}

///
pub fn db_idx_long_double_remove(iterator: i32){
    unsafe {
        return intrinsics::db_idx_long_double_remove(iterator);
    }
}

///
pub fn db_idx_long_double_next(iterator: i32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx_long_double_next(iterator, primary);
    }
}

///
pub fn db_idx_long_double_previous(iterator: i32, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx_long_double_previous(iterator, primary);
    }
}

///
pub fn db_idx_long_double_find_primary(code: u64, scope: u64, table: u64, secondary: *const Float128, primary: u64) -> i32{
    unsafe {
        return intrinsics::db_idx_long_double_find_primary(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx_long_double_find_secondary(code: u64, scope: u64, table: u64, secondary: *const Float128, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx_long_double_find_secondary(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx_long_double_lowerbound(code: u64, scope: u64, table: u64, secondary: *const Float128, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx_long_double_lowerbound(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx_long_double_upperbound(code: u64, scope: u64, table: u64, secondary: *const Float128, primary: *mut u64) -> i32{
    unsafe {
        return intrinsics::db_idx_long_double_upperbound(code, scope, table, secondary, primary);
    }
}

///
pub fn db_idx_long_double_end(code: u64, scope: u64, table: u64) -> i32{
    unsafe {
        return intrinsics::db_idx_long_double_end(code, scope, table);
    }
}

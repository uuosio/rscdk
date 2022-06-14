use crate::structs::*;

///
pub fn db_store_i64(_scope: u64, _table: u64, _payer: u64, _id: u64,  _data: *const u8, _len: u32) -> i32 {
    return 0;
}

///
pub fn db_update_i64(_iterator: i32, _payer: u64, _data: *const u8, _len: u32) {
    return;
}

///
pub fn db_remove_i64(_iterator: i32) {
    return;
}

///
pub fn db_get_i64(_iterator: i32, _data: *const u8, _len: u32) -> i32 {
    return 0;
}

///
pub fn db_next_i64(_iterator: i32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_previous_i64(_iterator: i32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_find_i64(_code: u64, _scope: u64, _table: u64, _id: u64) -> i32 {
    return 0;
}

///
pub fn db_lowerbound_i64(_code: u64, _scope: u64, _table: u64, _id: u64) -> i32 {
    return 0;
}

///
pub fn db_upperbound_i64(_code: u64, _scope: u64, _table: u64, _id: u64) -> i32 {
    return 0;
}

///
pub fn db_end_i64(_code: u64, _scope: u64, _table: u64) -> i32 {
    return 0;
}


///
pub fn db_idx64_store(_scope: u64, _table: u64, _payer: u64, _id: u64, _secondary: *const u64) -> i32 {
    return 0;
}

///
pub fn db_idx64_update(_iterator: i32, _payer: u64, _secondary: *const u64) {
    return;
}

///
pub fn db_idx64_remove(_iterator: i32) {
    return;
}

///
pub fn db_idx64_next(_iterator: i32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx64_previous(_iterator: i32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx64_find_primary(_code: u64, _scope: u64, _table: u64, _secondary: *mut u64, _primary: u64) -> i32 {
    return 0;
}

///
pub fn db_idx64_find_secondary(_code: u64, _scope: u64, _table: u64, _secondary: *const u64, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx64_lowerbound(_code: u64, _scope: u64, _table: u64, _secondary: *mut u64, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx64_upperbound(_code: u64, _scope: u64, _table: u64, _secondary: *mut u64, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx64_end(_code: u64, _scope: u64, _table: u64) -> i32 {
    return 0;
}


///
pub fn db_idx128_store(_scope: u64, _table: u64, _payer: u64, _id: u64, _secondary: *const Uint128) -> i32 {
    return 0;
}

///
pub fn db_idx128_update(_iterator: i32, _payer: u64, _secondary: *const Uint128) {
    return;
}

///
pub fn db_idx128_remove(_iterator: i32) {
    return;
}

///
pub fn db_idx128_next(_iterator: i32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx128_previous(_iterator: i32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx128_find_primary(_code: u64, _scope: u64, _table: u64, _secondary: *mut Uint128 , _primary: u64) -> i32 {
    return 0;
}

///
pub fn db_idx128_find_secondary(_code: u64, _scope: u64, _table: u64, _secondary: *const Uint128, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx128_lowerbound(_code: u64, _scope: u64, _table: u64, _secondary: *const Uint128, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx128_upperbound(_code: u64, _scope: u64, _table: u64, _secondary: *const Uint128, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx128_end(_code: u64, _scope: u64, _table: u64) -> i32 {
    return 0;
}


///
pub fn db_idx256_store(_scope: u64, _table: u64, _payer: u64, _id: u64, _data: *const Uint128, _data_len: u32 ) -> i32 {
    return 0;
}

///
pub fn db_idx256_update(_iterator: i32, _payer: u64, _data: *const Uint128, _data_len: u32) {
    return;
}

///
pub fn db_idx256_remove(_iterator: i32) {
    return;
}

///
pub fn db_idx256_next(_iterator: i32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx256_previous(_iterator: i32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx256_find_primary(_code: u64, _scope: u64, _table: u64, _data: *mut Uint128, _data_len: u32, _primary: u64) -> i32 {
    return 0;
}

///
pub fn db_idx256_find_secondary(_code: u64, _scope: u64, _table: u64, _data: *const Uint128, _data_len: u32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx256_lowerbound(_code: u64, _scope: u64, _table: u64, _data: *mut Uint128, _data_len: u32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx256_upperbound(_code: u64, _scope: u64, _table: u64, _data: *mut Uint128, _data_len: u32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx256_end(_code: u64, _scope: u64, _table: u64) -> i32 {
    return 0;
}


///
pub fn db_idx_double_store(_scope: u64, _table: u64, _payer: u64, _id: u64, _secondary: *const f64) -> i32 {
    return 0;
}

///
pub fn db_idx_double_update(_iterator: i32, _payer: u64, _secondary: *const f64) {
    return;
}

///
pub fn db_idx_double_remove(_iterator: i32) {
    return;
}

///
pub fn db_idx_double_next(_iterator: i32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx_double_previous(_iterator: i32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx_double_find_primary(_code: u64, _scope: u64, _table: u64, _secondary: *mut f64, _primary: u64) -> i32 {
    return 0;
}

///
pub fn db_idx_double_find_secondary(_code: u64, _scope: u64, _table: u64, _secondary: *const f64, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx_double_lowerbound(_code: u64, _scope: u64, _table: u64, _secondary: *mut f64, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx_double_upperbound(_code: u64, _scope: u64, _table: u64, _secondary: *mut f64, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx_double_end(_code: u64, _scope: u64, _table: u64) -> i32 {
    return 0;
}


///
pub fn db_idx_long_double_store(_scope: u64, _table: u64, _payer: u64, _id: u64, _secondary: *const Float128) -> i32 {
    return 0;
}

///
pub fn db_idx_long_double_update(_iterator: i32, _payer: u64, _secondary: *const Float128) {
    return;
}

///
pub fn db_idx_long_double_remove(_iterator: i32) {
    return;
}

///
pub fn db_idx_long_double_next(_iterator: i32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx_long_double_previous(_iterator: i32, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx_long_double_find_primary(_code: u64, _scope: u64, _table: u64, _secondary: *const Float128, _primary: u64) -> i32 {
    return 0;
}

///
pub fn db_idx_long_double_find_secondary(_code: u64, _scope: u64, _table: u64, _secondary: *const Float128, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx_long_double_lowerbound(_code: u64, _scope: u64, _table: u64, _secondary: *const Float128, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx_long_double_upperbound(_code: u64, _scope: u64, _table: u64, _secondary: *const Float128, _primary: *mut u64) -> i32 {
    return 0;
}

///
pub fn db_idx_long_double_end(_code: u64, _scope: u64, _table: u64) -> i32 {
    return 0;
}


use crate::structs::*;

use eosio_chaintester::{
    get_vm_api_client, 
    interfaces::{
        TApplySyncClient,
    }
};

use core::slice;

///
pub fn db_store_i64(scope: u64, table: u64, payer: u64, id: u64,  data: *const u8, length: u32) -> i32 {
    let mut _data = unsafe {
        slice::from_raw_parts(data, length as usize)
    };

    get_vm_api_client().db_store_i64(scope.into(), table.into(), payer.into(), id.into(), _data.to_vec()).unwrap()
}

///
pub fn db_update_i64(iterator: i32, payer: u64, data: *const u8, len: u32) {
    let mut _data = unsafe {
        slice::from_raw_parts(data, len as usize)
    };
    get_vm_api_client().db_update_i64(iterator, payer.into(), _data.to_vec()).unwrap()
}

///
pub fn db_remove_i64(iterator: i32) {
    get_vm_api_client().db_remove_i64(iterator).unwrap()
}

///
pub fn db_get_i64(iterator: i32, data: *mut u8, len: u32) -> i32 {
    let ret = get_vm_api_client().db_get_i64(iterator, len as i32).unwrap();
    let size = ret.size.unwrap();

    if let Some(value) = &ret.buffer {
        let buffer = ret.buffer.unwrap();
        if buffer.len() > 0 {
            crate::vmapi::eosio::memcpy(data, buffer.as_ptr(), size as usize);
        }
    }
    return size;
}

///
pub fn db_next_i64(iterator: i32, primary: *mut u64) -> i32 {
    let ret = get_vm_api_client().db_next_i64(iterator).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            *primary = ret.primary.unwrap().into();
        }
    }
    it
}

///
pub fn db_previous_i64(iterator: i32, primary: *mut u64) -> i32 {
    let ret = get_vm_api_client().db_previous_i64(iterator).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            *primary = ret.primary.unwrap().into();
        }
    }
    it
}

///
pub fn db_find_i64(code: u64, scope: u64, table: u64, id: u64) -> i32 {
    get_vm_api_client().db_find_i64(code.into(), scope.into(), table.into(), id.into()).unwrap()
}

///
pub fn db_lowerbound_i64(code: u64, scope: u64, table: u64, id: u64) -> i32 {
    get_vm_api_client().db_lowerbound_i64(code.into(), scope.into(), table.into(), id.into()).unwrap()
}

///
pub fn db_upperbound_i64(code: u64, scope: u64, table: u64, id: u64) -> i32 {
    get_vm_api_client().db_upperbound_i64(code.into(), scope.into(), table.into(), id.into()).unwrap()
}

///
pub fn db_end_i64(code: u64, scope: u64, table: u64) -> i32 {
    get_vm_api_client().db_end_i64(code.into(), scope.into(), table.into()).unwrap()
}

///
pub fn db_idx64_store(scope: u64, table: u64, payer: u64, id: u64, secondary: *const u64) -> i32 {
    let _secondary = unsafe {
        *secondary
    };
    get_vm_api_client().db_idx64_store(scope.into(), table.into(), payer.into(), id.into(), _secondary.into()).unwrap()
}

///
pub fn db_idx64_update(iterator: i32, payer: u64, secondary: *const u64) {
    let _secondary = unsafe {
        *secondary
    };
    get_vm_api_client().db_idx64_update(iterator, payer.into(), _secondary.into()).unwrap()
}

///
pub fn db_idx64_remove(iterator: i32) {
    get_vm_api_client().db_idx64_remove(iterator).unwrap()
}

///
pub fn db_idx64_next(iterator: i32, primary: *mut u64) -> i32 {
    let ret = get_vm_api_client().db_idx64_next(iterator).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            *primary = ret.primary.unwrap().into();
        }
    }
    it
}

///
pub fn db_idx64_previous(iterator: i32, primary: *mut u64) -> i32 {
    let ret = get_vm_api_client().db_idx64_previous(iterator).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            *primary = ret.primary.unwrap().into();
        }
    }
    it
}

///
pub fn db_idx64_find_primary(code: u64, scope: u64, table: u64, secondary: *mut u64, primary: u64) -> i32 {
    let ret = get_vm_api_client().db_idx64_find_primary(code.into(), scope.into(), table.into(), primary.into()).unwrap();
    unsafe {
        *secondary = u64::from_le_bytes(ret.secondary.unwrap().try_into().expect("db_idx64_findprimary: invalid secondary value"));
    }
    ret.iterator.unwrap()
}

///
pub fn db_idx64_find_secondary(code: u64, scope: u64, table: u64, secondary: *const u64, primary: *mut u64) -> i32 {
    let _secondary = unsafe {
        *secondary
    };
    let ret = get_vm_api_client().db_idx64_find_secondary(code.into(), scope.into(), table.into(), _secondary.into()).unwrap();

    unsafe {
        *primary = ret.primary.unwrap().into();
    }
    ret.iterator.unwrap()
}

///
pub fn db_idx64_lowerbound(code: u64, scope: u64, table: u64, secondary: *mut u64, primary: *mut u64) -> i32 {
    let (_secondary, _primary) = unsafe {
        (*secondary, *primary)
    };

    let ret = get_vm_api_client().db_idx64_lowerbound(code.into(), scope.into(), table.into(), _secondary.into(), _primary.into()).unwrap();
    unsafe {
        if ret.iterator.unwrap() >= 0 {
            *secondary = u64::from_le_bytes(ret.secondary.unwrap().try_into().expect("db_idx64_lowerbound: invalid secondary value"));
            *primary = ret.primary.unwrap().into();
        }
    }
    ret.iterator.unwrap()
}

///
pub fn db_idx64_upperbound(code: u64, scope: u64, table: u64, secondary: *mut u64, primary: *mut u64) -> i32 {
    let (_secondary, _primary) = unsafe {
        (*secondary, *primary)
    };

    let ret = get_vm_api_client().db_idx64_upperbound(code.into(), scope.into(), table.into(), _secondary.into(), _primary.into()).unwrap();
    unsafe {
        if ret.iterator.unwrap() >= 0 {
            *secondary = u64::from_le_bytes(ret.secondary.unwrap().try_into().expect("db_idx64_lowerbound: invalid secondary value"));
            *primary = ret.primary.unwrap().into();
        }
    }
    ret.iterator.unwrap()
}

///
pub fn db_idx64_end(code: u64, scope: u64, table: u64) -> i32 {
    get_vm_api_client().db_idx64_end(code.into(), scope.into(), table.into()).unwrap()
}

///
pub fn db_idx128_store(scope: u64, table: u64, payer: u64, id: u64, secondary: *const Uint128) -> i32 {
    let _secondary = unsafe {
        slice::from_raw_parts(secondary as *const u8, 16)
    };
    get_vm_api_client().db_idx128_store(scope.into(), table.into(), payer.into(), id.into(), _secondary.into()).unwrap()
}

///
pub fn db_idx128_update(iterator: i32, payer: u64, secondary: *const Uint128) {
    let _secondary = unsafe {
        slice::from_raw_parts(secondary as *const u8, 16)
    };
    get_vm_api_client().db_idx128_update(iterator, payer.into(), _secondary.into()).unwrap()
}

///
pub fn db_idx128_remove(iterator: i32) {
    get_vm_api_client().db_idx128_remove(iterator).unwrap()
}

///
pub fn db_idx128_next(iterator: i32, primary: *mut u64) -> i32 {
    let ret = get_vm_api_client().db_idx128_next(iterator).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            *primary = ret.primary.unwrap().into();
        }
    }
    it
}

///
pub fn db_idx128_previous(iterator: i32, primary: *mut u64) -> i32 {
    let ret = get_vm_api_client().db_idx128_previous(iterator).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            *primary = ret.primary.unwrap().into();
        }
    }
    it
}

///
pub fn db_idx128_find_primary(code: u64, scope: u64, table: u64, secondary: *mut Uint128 , primary: u64) -> i32 {
    let ret = get_vm_api_client().db_idx128_find_primary(code.into(), scope.into(), table.into(), primary.into()).unwrap();
    let it = ret.iterator.unwrap();
    if it >= 0 {
        crate::vmapi::eosio::memcpy(secondary as *mut u8, ret.secondary.unwrap().as_ptr(), 16);
    }
    it
}

///
pub fn db_idx128_find_secondary(code: u64, scope: u64, table: u64, secondary: *const Uint128, primary: *mut u64) -> i32 {
    let _secondary = unsafe {
        slice::from_raw_parts(secondary as *const u8, 16)
    };
    let ret = get_vm_api_client().db_idx128_find_secondary(code.into(), scope.into(), table.into(), _secondary.into()).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        *primary = ret.primary.unwrap().into();
    }
    it
}

///
pub fn db_idx128_lowerbound(code: u64, scope: u64, table: u64, secondary: *const Uint128, primary: *mut u64) -> i32 {
    let (_secondary, _primary) = unsafe {
        (slice::from_raw_parts(secondary as *const u8, 16), *primary)
    };

    let ret = get_vm_api_client().db_idx128_lowerbound(code.into(), scope.into(), table.into(), _secondary.into(), _primary.into()).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            if ret.iterator.unwrap() >= 0 {
                crate::vmapi::eosio::memcpy(secondary as *mut u8, ret.secondary.unwrap().as_ptr(), 16);
                *primary = ret.primary.unwrap().into();
            }    
        }
    }
    it
}

///
pub fn db_idx128_upperbound(code: u64, scope: u64, table: u64, secondary: *const Uint128, primary: *mut u64) -> i32 {
    let (_secondary, _primary) = unsafe {
        (slice::from_raw_parts(secondary as *const u8, 16), *primary)
    };

    let ret = get_vm_api_client().db_idx128_upperbound(code.into(), scope.into(), table.into(), _secondary.into(), _primary.into()).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            if ret.iterator.unwrap() >= 0 {
                crate::vmapi::eosio::memcpy(secondary as *mut u8, ret.secondary.unwrap().as_ptr(), 16);
                *primary = ret.primary.unwrap().into();
            }    
        }
    }
    it
}

///
pub fn db_idx128_end(code: u64, scope: u64, table: u64) -> i32 {
    get_vm_api_client().db_idx128_end(code.into(), scope.into(), table.into()).unwrap()
}

///
pub fn db_idx256_store(scope: u64, table: u64, payer: u64, id: u64, data: *const Uint128, data_len: u32 ) -> i32 {
    if data_len != 2 {
        panic!("db_idx256_store: invalid data_len");
    }
    let _secondary = unsafe {
        slice::from_raw_parts(data as *const u8, 32)
    };
    get_vm_api_client().db_idx256_store(scope.into(), table.into(), payer.into(), id.into(), _secondary.into()).unwrap()
}

///
pub fn db_idx256_update(iterator: i32, payer: u64, data: *const Uint128, data_len: u32) {
    if data_len != 2 {
        panic!("db_idx256_update: invalid data_len");
    }

    let _secondary = unsafe {
        slice::from_raw_parts(data as *const u8, 32)
    };
    get_vm_api_client().db_idx256_update(iterator, payer.into(), _secondary.into()).unwrap()
}

///
pub fn db_idx256_remove(iterator: i32) {
    get_vm_api_client().db_idx256_remove(iterator).unwrap()
}

///
pub fn db_idx256_next(iterator: i32, primary: *mut u64) -> i32 {
    let ret = get_vm_api_client().db_idx256_next(iterator).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            *primary = ret.primary.unwrap().into();
        }
    }
    it
}

///
pub fn db_idx256_previous(iterator: i32, primary: *mut u64) -> i32 {
    let ret = get_vm_api_client().db_idx256_previous(iterator).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            *primary = ret.primary.unwrap().into();
        }
    }
    it
}

///
pub fn db_idx256_find_primary(code: u64, scope: u64, table: u64, data: *mut Uint128, _data_len: u32, primary: u64) -> i32 {
    let ret = get_vm_api_client().db_idx256_find_primary(code.into(), scope.into(), table.into(), primary.into()).unwrap();
    let it = ret.iterator.unwrap();
    if it >= 0 {
        crate::vmapi::eosio::memcpy(data as *mut u8, ret.secondary.unwrap().as_ptr(), 32);
    }
    it
}

///
pub fn db_idx256_find_secondary(code: u64, scope: u64, table: u64, data: *const Uint128, data_len: u32, primary: *mut u64) -> i32 {
    let _secondary = unsafe {
        slice::from_raw_parts(data as *const u8, 32)
    };
    let ret = get_vm_api_client().db_idx256_find_secondary(code.into(), scope.into(), table.into(), _secondary.into()).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        *primary = ret.primary.unwrap().into();
    }
    it
}

///
pub fn db_idx256_lowerbound(code: u64, scope: u64, table: u64, data: *mut Uint128, data_len: u32, primary: *mut u64) -> i32 {
    if data_len != 2 {
        panic!("db_idx256_lowerbound: bad data_len!");
    }

    let (_secondary, _primary) = unsafe {
        (slice::from_raw_parts(data as *const u8, 32), *primary)
    };

    let ret = get_vm_api_client().db_idx256_lowerbound(code.into(), scope.into(), table.into(), _secondary.into(), _primary.into()).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            if ret.iterator.unwrap() >= 0 {
                crate::vmapi::eosio::memcpy(data as *mut u8, ret.secondary.unwrap().as_ptr(), 32);
                *primary = ret.primary.unwrap().into();
            }    
        }
    }
    it
}

///
pub fn db_idx256_upperbound(code: u64, scope: u64, table: u64, data: *mut Uint128, data_len: u32, primary: *mut u64) -> i32 {
    let (_secondary, _primary) = unsafe {
        (slice::from_raw_parts(data as *const u8, 32), *primary)
    };

    let ret = get_vm_api_client().db_idx256_upperbound(code.into(), scope.into(), table.into(), _secondary.into(), _primary.into()).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            if ret.iterator.unwrap() >= 0 {
                crate::vmapi::eosio::memcpy(data as *mut u8, ret.secondary.unwrap().as_ptr(), 32);
                *primary = ret.primary.unwrap().into();
            }
        }
    }
    it
}

///
pub fn db_idx256_end(code: u64, scope: u64, table: u64) -> i32 {
    get_vm_api_client().db_idx256_end(code.into(), scope.into(), table.into()).unwrap()
}

///
pub fn db_idx_double_store(scope: u64, table: u64, payer: u64, id: u64, secondary: *const f64) -> i32 {
    let _secondary = unsafe {
        slice::from_raw_parts(secondary as *const u8, 8)
    };
    get_vm_api_client().db_idx_double_store(scope.into(), table.into(), payer.into(), id.into(), _secondary.into()).unwrap()
}

///
pub fn db_idx_double_update(iterator: i32, payer: u64, secondary: *const f64) {
    let _secondary = unsafe {
        slice::from_raw_parts(secondary as *const u8, 8)
    };
    get_vm_api_client().db_idx_double_update(iterator, payer.into(), _secondary.into()).unwrap()
}

///
pub fn db_idx_double_remove(iterator: i32) {
    get_vm_api_client().db_idx_double_remove(iterator).unwrap()
}

///
pub fn db_idx_double_next(iterator: i32, primary: *mut u64) -> i32 {
    let ret = get_vm_api_client().db_idx_double_next(iterator).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            *primary = ret.primary.unwrap().into();
        }
    }
    it
}

///
pub fn db_idx_double_previous(iterator: i32, primary: *mut u64) -> i32 {
    let ret = get_vm_api_client().db_idx_double_previous(iterator).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            *primary = ret.primary.unwrap().into();
        }
    }
    it
}

///
pub fn db_idx_double_find_primary(code: u64, scope: u64, table: u64, secondary: *mut f64, primary: u64) -> i32 {
    let ret = get_vm_api_client().db_idx_double_find_primary(code.into(), scope.into(), table.into(), primary.into()).unwrap();
    let it = ret.iterator.unwrap();
    if it >= 0 {
        crate::vmapi::eosio::memcpy(secondary as *mut u8, ret.secondary.unwrap().as_ptr(), 8);
    }
    it
}

///
pub fn db_idx_double_find_secondary(code: u64, scope: u64, table: u64, secondary: *const f64, primary: *mut u64) -> i32 {
    let _secondary = unsafe {
        slice::from_raw_parts(secondary as *const u8, 8)
    };
    let ret = get_vm_api_client().db_idx_double_find_secondary(code.into(), scope.into(), table.into(), _secondary.into()).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        *primary = ret.primary.unwrap().into();
    }
    it
}

///
pub fn db_idx_double_lowerbound(code: u64, scope: u64, table: u64, secondary: *mut f64, primary: *mut u64) -> i32 {
    let (_secondary, _primary) = unsafe {
        (slice::from_raw_parts(secondary as *const u8, 8), *primary)
    };

    let ret = get_vm_api_client().db_idx_double_lowerbound(code.into(), scope.into(), table.into(), _secondary.into(), _primary.into()).unwrap();
    let it = ret.iterator.unwrap();
    if it >= 0 {
        if ret.iterator.unwrap() >= 0 {
            crate::vmapi::eosio::memcpy(secondary as *mut u8, ret.secondary.unwrap().as_ptr(), 8);
            unsafe {
                *primary = ret.primary.unwrap().into();
            }
        }    
    }
    it
}

///
pub fn db_idx_double_upperbound(code: u64, scope: u64, table: u64, secondary: *mut f64, primary: *mut u64) -> i32 {
    let (_secondary, _primary) = unsafe {
        (slice::from_raw_parts(secondary as *const u8, 8), *primary)
    };

    let ret = get_vm_api_client().db_idx_double_upperbound(code.into(), scope.into(), table.into(), _secondary.into(), _primary.into()).unwrap();
    let it = ret.iterator.unwrap();
    if it >= 0 {
        if ret.iterator.unwrap() >= 0 {
            crate::vmapi::eosio::memcpy(secondary as *mut u8, ret.secondary.unwrap().as_ptr(), 8);
            unsafe {
                *primary = ret.primary.unwrap().into();
            }
        }    
    }
    it
}

///
pub fn db_idx_double_end(code: u64, scope: u64, table: u64) -> i32 {
    get_vm_api_client().db_idx_double_end(code.into(), scope.into(), table.into()).unwrap()
}

///
pub fn db_idx_long_double_store(scope: u64, table: u64, payer: u64, id: u64, secondary: *const Float128) -> i32 {
    let _secondary = unsafe {
        slice::from_raw_parts(secondary as *const u8, 16)
    };
    get_vm_api_client().db_idx_long_double_store(scope.into(), table.into(), payer.into(), id.into(), _secondary.into()).unwrap()
}

///
pub fn db_idx_long_double_update(iterator: i32, payer: u64, secondary: *const Float128) {
    let _secondary = unsafe {
        slice::from_raw_parts(secondary as *const u8, 16)
    };
    get_vm_api_client().db_idx_long_double_update(iterator, payer.into(), _secondary.into()).unwrap()
}

///
pub fn db_idx_long_double_remove(iterator: i32) {
    get_vm_api_client().db_idx_long_double_remove(iterator).unwrap()
}

///
pub fn db_idx_long_double_next(iterator: i32, primary: *mut u64) -> i32 {
    let ret = get_vm_api_client().db_idx_long_double_next(iterator).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            *primary = ret.primary.unwrap().into();
        }
    }
    it
}

///
pub fn db_idx_long_double_previous(iterator: i32, primary: *mut u64) -> i32 {
    let ret = get_vm_api_client().db_idx_long_double_previous(iterator).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        if it >= 0 {
            *primary = ret.primary.unwrap().into();
        }
    }
    it
}

///
pub fn db_idx_long_double_find_primary(code: u64, scope: u64, table: u64, secondary: *const Float128, primary: u64) -> i32 {
    let ret = get_vm_api_client().db_idx_long_double_find_primary(code.into(), scope.into(), table.into(), primary.into()).unwrap();
    let it = ret.iterator.unwrap();
    if it >= 0 {
        crate::vmapi::eosio::memcpy(secondary as *mut u8, ret.secondary.unwrap().as_ptr(), 16);
    }
    it
}

///
pub fn db_idx_long_double_find_secondary(code: u64, scope: u64, table: u64, secondary: *const Float128, primary: *mut u64) -> i32 {
    let _secondary = unsafe {
        slice::from_raw_parts(secondary as *const u8, 16)
    };
    let ret = get_vm_api_client().db_idx_long_double_find_secondary(code.into(), scope.into(), table.into(), _secondary.into()).unwrap();
    let it = ret.iterator.unwrap();
    unsafe {
        *primary = ret.primary.unwrap().into();
    }
    it
}

///
pub fn db_idx_long_double_lowerbound(code: u64, scope: u64, table: u64, secondary: *const Float128, primary: *mut u64) -> i32 {
    let (_secondary, _primary) = unsafe {
        (slice::from_raw_parts(secondary as *const u8, 16), *primary)
    };

    let ret = get_vm_api_client().db_idx_long_double_lowerbound(code.into(), scope.into(), table.into(), _secondary.into(), _primary.into()).unwrap();
    let it = ret.iterator.unwrap();
    if it >= 0 {
        if ret.iterator.unwrap() >= 0 {
            crate::vmapi::eosio::memcpy(secondary as *mut u8, ret.secondary.unwrap().as_ptr(), 16);
            unsafe {
                *primary = ret.primary.unwrap().into();
            }
        }    
    }
    it
}

///
pub fn db_idx_long_double_upperbound(code: u64, scope: u64, table: u64, secondary: *const Float128, primary: *mut u64) -> i32 {
    let (_secondary, _primary) = unsafe {
        (slice::from_raw_parts(secondary as *const u8, 16), *primary)
    };

    let ret = get_vm_api_client().db_idx_long_double_upperbound(code.into(), scope.into(), table.into(), _secondary.into(), _primary.into()).unwrap();
    let it = ret.iterator.unwrap();
    if it >= 0 {
        if ret.iterator.unwrap() >= 0 {
            crate::vmapi::eosio::memcpy(secondary as *mut u8, ret.secondary.unwrap().as_ptr(), 16);
            unsafe {
                *primary = ret.primary.unwrap().into();
            }
        }    
    }
    it
}

///
pub fn db_idx_long_double_end(code: u64, scope: u64, table: u64) -> i32 {
    get_vm_api_client().db_idx_long_double_end(code.into(), scope.into(), table.into()).unwrap()
}


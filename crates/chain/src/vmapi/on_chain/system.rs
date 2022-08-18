mod intrinsics {
    extern "C" {
        pub fn eosio_assert_message(test: u32, msg: *const u8, msg_len: u32);
        pub fn eosio_assert_code(test: u32, code: u64);
    }
}

///
pub fn eosio_assert_message(test: u32, msg: *const u8, msg_len: u32) {
    unsafe {
        intrinsics::eosio_assert_message(test, msg, msg_len);
    }
}

///
pub fn eosio_assert_code(test: u32, code: u64) {
    unsafe {
        intrinsics::eosio_assert_code(test, code);
    }
}

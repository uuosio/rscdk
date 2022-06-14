use crate::vmapi::eosio::{eosio_assert};

extern "C" {
    // pub fn get_active_producers(producers: *const u8, datalen: u32) -> u32;
}

///
pub fn panic(s: &str) {
    eosio_assert(0, s);
}

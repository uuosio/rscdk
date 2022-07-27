#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc_error_handler, panic_info_message, core_intrinsics))]

#[cfg(not(feature = "std"))]
mod allocator;

#[cfg(not(feature = "std"))]
#[cfg(not(feature = "wee-alloc"))]
#[global_allocator]
static mut ALLOC: allocator::bump::BumpAllocator = allocator::bump::BumpAllocator {};

#[cfg(all(not(feature = "std"), target_arch = "wasm32"))]
#[allow(unused_variables)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let msg = format!("{:?}", info.message().unwrap().as_str().unwrap());
    self::vmapi::eosio::check(false, &msg);
    core::arch::wasm32::unreachable();
}

#[cfg(not(feature = "std"))]
extern crate alloc;

use cfg_if::cfg_if;

#[cfg(feature = "std")]
pub use eosio_chaintester;

#[cfg(feature = "std")]
pub use eosio_chaintester::{
    ChainTester,
};

cfg_if! {
    if #[cfg(feature = "std")] {
        pub use std::{
            borrow,
            boxed,
            format,
            string,
            vec,
            rc,
        };

        /// Collection types.
        pub mod collections {
            pub use self::{
                binary_heap::BinaryHeap,
                btree_map::BTreeMap,
                btree_set::BTreeSet,
                linked_list::LinkedList,
                vec_deque::VecDeque,
                Bound,
            };
            pub use std::collections::*;
        }
    } else {
        pub use alloc::{
            borrow,
            boxed,
            format,
            string,
            vec,
            rc,
        };

        /// Collection types.
        pub mod collections {
            pub use self::{
                BTreeMap,
                BTreeSet,
                BinaryHeap,
                LinkedList,
                VecDeque,
            };
            pub use alloc::collections::*;
            pub use core::ops::Bound;
        }
    }
}


///
pub mod vmapi;
///
pub mod structs;
pub use self::structs::*;

///
pub mod transaction;
pub use self::transaction::{
    TransactionExtension,
    Transaction,
};

///
pub mod serializer;
///
pub mod db;

///
#[macro_use]
pub mod print;

///
pub mod mi;

///
pub mod mi_not_generic;

///
mod asset;
pub use asset::{
    Asset,
    Symbol,
};

///
pub mod crypto;
pub use crypto::{
    assert_sha256,
    assert_sha1,
    assert_sha512,
    assert_ripemd160,

    sha256,
    sha1,
    sha512,
    ripemd160,

    recover_key,
    assert_recover_key,
};

pub use self::vmapi::eosio:: {
    get_active_producers,
    get_permission_last_used,
    get_account_creation_time,
    read_action_data,
    action_data_size,
    require_recipient,
    require_auth,
    has_auth,
    require_auth2,
    is_account,
    send_inline,
    send_context_free_inline,
    publication_time,
    current_receiver,
    check,
    eosio_assert_code,
    eosio_exit,
    current_time,
    is_feature_activated,
    get_sender,
    get_resource_limits,
    set_resource_limits,
    set_proposed_producers,
    set_proposed_producers_ex,
    is_privileged,
    set_privileged,
    set_blockchain_parameters_packed,
    get_blockchain_parameters_packed,
    preactivate_feature,
    send_deferred,
    cancel_deferred,
    read_transaction,
    transaction_size,
    tapos_block_num,
    tapos_block_prefix,
    expiration,
    get_action,
    get_context_free_data,
};

///
#[macro_use]
pub mod name;

pub use name::{
    SAME_PAYER,
    ACTIVE,
    OWNER,
    CODE,
};

///
pub mod action;

///
pub mod utils;
///
pub mod varint;

///
pub mod binary_extension;
pub use binary_extension::BinaryExtension;

///
pub mod intrinsic_abi_types;
pub use intrinsic_abi_types::*;

pub use eosio_macro::{
    contract,
    // chain,
};

#[cfg(feature = "std")]
pub use eosio_scale_info;

#[cfg(feature = "std")]
pub mod abi;

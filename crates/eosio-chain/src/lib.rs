#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc_error_handler, core_intrinsics))]

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
    core::arch::wasm32::unreachable();
}

#[cfg(not(feature = "std"))]
extern crate alloc;

use cfg_if::cfg_if;

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
pub mod serializer;
///
pub mod db;
///
pub mod print;
///
pub mod mi;
///
pub mod asset;

pub use self::vmapi::eosio:: {
    eosio_assert,
    check,
};

///
pub mod name;
pub use self::name::{
    Name,
};

///
pub mod action;

///
pub mod utils;
///
pub mod varint;

pub use eosio_macro::{
    contract,
    // chain,
};

#[cfg(feature = "std")]
pub use eosio_scale_info;

#[cfg(feature = "std")]
pub mod abi;

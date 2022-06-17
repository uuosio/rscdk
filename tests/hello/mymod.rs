#![cfg_attr(not(feature = "std"), no_std)]

use eosio_chain as chain;

#[chain::contract]
pub mod hello {
    #[chain(packer)]
    #[derive(Clone, Eq, PartialEq)]
    pub struct AA {
        value: u64,
    }

    #[chain(packer)]
    #[derive(Clone, Eq, PartialEq)]
    pub struct BB {
        value: u64,
        cc: CC,
    }

    #[derive(Clone, Eq, PartialEq)]
    pub struct CC {
        value: u64,
    }
}

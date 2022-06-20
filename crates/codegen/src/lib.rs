#[macro_use]
mod error;

mod fixedstring;
mod name;

mod action;
mod table;
mod attrs;
mod contract;
mod contract_tests;

pub use self::{
    contract::Contract,
    fixedstring::FixedString,
};

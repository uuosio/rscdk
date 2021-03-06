// use quote::quote;

extern crate proc_macro;

mod contract;

use proc_macro::{
    TokenStream,
};

#[proc_macro_attribute]
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract::generate(attr.into(), item.into()).into()
}

// #[proc_macro_attribute]
// pub fn chain(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     item
// }

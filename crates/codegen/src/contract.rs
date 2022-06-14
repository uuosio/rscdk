use core::convert::TryFrom;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use quote::quote_spanned;

use syn::{
    spanned::Spanned,
    token,
};

use proc_macro2::{
    Ident,
    Span,
};

use crate::{
    action::Action,
    attrs,
    FixedString,
    format_err_spanned,
    attrs::Attrs as _,
};

/// An contract definition consisting of the configuration and module.

#[derive(Debug, PartialEq, Eq)]
pub struct Contract {
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    ident: Ident,
    items: Vec<syn::Item>,
    variants: Vec<syn::ItemEnum>,
    actions: Vec<Action>,
    structs: Vec<syn::ItemStruct>,
    others: Vec<syn::Item>,
}

impl TryFrom<syn::ItemMod> for Contract {
    type Error = syn::Error;

    fn try_from(module: syn::ItemMod) -> Result<Self, Self::Error> {
        let module_span = module.span();
        let (brace, items) = match module.content {
            Some((brace, items)) => (brace, items),
            None => {
                return Err(format_err_spanned!(
                    module,
                    "out-of-line modules are not supported, use `#[chain::contract] mod name {{ ... }}`",
                ))
            }
        };

        let (_, other_attrs) = attrs::partition_attributes(module.attrs)?;

        let mut contract = Self {
            attrs: other_attrs,
            ident: module.ident,
            vis: module.vis,
            items: items,
            variants: Vec::new(),
            actions: Vec::new(),
            structs: Vec::new(),
            others: Vec::new(),
        };

        contract.items.iter_mut().for_each(|item|{
            match item {
                syn::Item::Struct(x) => {
                    let (_, other_attrs) = attrs::partition_attributes(x.attrs.clone()).unwrap();
                    x.attrs = other_attrs;
                    x.fields.iter_mut().for_each(|field| {
                        let (_, other_attrs) = attrs::partition_attributes(field.attrs.clone()).unwrap();
                        field.attrs = other_attrs;
                    });
                    contract.structs.push(x.clone());
                }
                syn::Item::Impl(x) => {
                    for impl_item in &mut x.items {
                        match impl_item {
                            syn::ImplItem::Method(method_item) => {
                                let (_, other_attrs) = attrs::partition_attributes(method_item.attrs.clone()).unwrap();
                                method_item.attrs = other_attrs;        
                                if attrs::contains_chain_attributes(&method_item.attrs) {
                                    // contract.others.push(item.clone());
                                } else {
                                    contract.actions.push(
                                        Action{
                                            item: method_item.clone(),
                                            is_notify: false,
                                            action_name: FixedString::new(""),
                                        }
                                    )
                                }
                            }
                            _ => {
                                // contract.others.push(item.clone());
                            }
                        }
                    }
                }
                syn::Item::Enum(x) => {
                    contract.variants.push(x.clone());
                }
                _ => {}
            }
        });

        return Ok(contract);
    }
}

impl Contract {
    /// Creates a new contract from the given configuration and module
    pub fn new(
        config: TokenStream2,
        module: TokenStream2,
    ) -> Result<Self, syn::Error> {
        let module = syn::parse2::<syn::ItemMod>(module)?;
        return Contract::try_from(module);
    }

    /// Returns all non-chain attributes of the chain module.
    pub fn attrs(&self) -> &[syn::Attribute] {
        &self.attrs
    }

    /// Returns the visibility of the chain module.
    pub fn vis(&self) -> &syn::Visibility {
        &self.vis
    }

    pub fn has_main_struct(&self) -> bool {
        return true;
    }
    
    pub fn generate_code(&self) -> TokenStream2 {
        // let items = self.items.iter().map(|item| {
        //     let span = item.span();
        //     quote_spanned!(span=>
        //         #item
        //     )
        // });
        let items = &self.items;
        let ident = &self.ident;
        let attrs = self.attrs();
        let vis = self.vis();
        quote! {
            use eosio_chain::{
                serializer::Packer,
                db::SecondaryType,
                db::SecondaryValue,
                db::MultiIndexValue,
                db::ToPrimaryValue,
                db::ToSecondaryValue,
                db::FromSecondaryValue,
                structs::*,
                asset::{
                    Symbol,
                    SymbolCode,
                    Asset,
                    ExtendedAsset,
                },
                varint::{
                    VarUint32,
                },
                print::*,
                vmapi::eosio::{
                    check,
                },
                mi,
            };

            use eosio_chain::name::{
                Name,
            };

            use eosio_chain::{
                eosio_print,
                eosio_println,
            };

            use eosio_chain::{
                vec,
                vec::Vec,
                boxed::Box,
                string::String,
            };

            #( #attrs )*
            #vis mod #ident {
                use super::*;
                #( #items ) *

                #[no_mangle]
                fn apply(receiver: u64, first_receiver: u64, action: u64) {
                    let mut contract = Hello::new(receiver, first_receiver, action);
                    contract.say_hello();
                }    
            }
        }
    }

    pub fn get_items(self) -> Vec<syn::Item> {
        return self.items;
    }

    pub fn is_action_impl_block(
        item_impl: &syn::ItemImpl,
    ) -> Result<bool, syn::Error> {
        // Quick check in order to efficiently bail out in case where there are
        // no attributes:
        if !attrs::contains_chain_attributes(&item_impl.attrs)
            && item_impl
                .items
                .iter()
                .all(|item| !attrs::contains_chain_attributes(item.attrs()))
        {
            return Ok(false)
        }

        // Check if any of the implementation block's methods either resembles
        // an contract constructor or an contract message:
        'repeat: for item in &item_impl.items {
            match item {
                syn::ImplItem::Method(method_item) => {
                    if !attrs::contains_chain_attributes(&method_item.attrs) {
                        continue 'repeat
                    }
                    let attr = attrs::first_chain_attribute(&method_item.attrs)?
                        .expect("missing expected contract attribute for struct");
                    match attr.first().kind() {
                        attrs::AttributeArg::Action(_) => {
                            return Ok(true)
                        }
                        _ => continue 'repeat,
                    }
                }
                _ => continue 'repeat,
            }
        }
        Ok(false)
    }
}


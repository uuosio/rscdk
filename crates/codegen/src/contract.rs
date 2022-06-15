use std::collections::HashMap;

use core::convert::TryFrom;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use quote::quote_spanned;

use syn::{
    spanned::Spanned,
    // token,
};

use proc_macro2::{
    Ident,
    // Span,
};

use crate::{
    action::Action,
    attrs,
    // FixedString,
    name::s2n,
    format_err_spanned,
    attrs::Attrs as _,
};

/// An contract definition consisting of the configuration and module.

#[derive(Debug, PartialEq, Eq)]
pub struct Contract {
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    ident: Ident,
    main_struct: Option<syn::ItemStruct>,
    items: Vec<syn::Item>,
    variants: Vec<syn::ItemEnum>,
    actions: Vec<Action>,
    structs: Vec<syn::ItemStruct>,
    packers: Vec<syn::ItemStruct>,
    others: Vec<syn::Item>,
}

impl TryFrom<syn::ItemMod> for Contract {
    type Error = syn::Error;

    fn try_from(module: syn::ItemMod) -> Result<Self, Self::Error> {
        let _module_span = module.span();
        let (_brace, items) = match module.content {
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
            main_struct: None,
            items: items,
            variants: Vec::new(),
            actions: Vec::new(),
            structs: Vec::new(),
            packers: Vec::new(),
            others: Vec::new(),
        };
        contract.analyze_items();
        return Ok(contract);
    }
}

impl Contract {
    /// Creates a new contract from the given configuration and module
    pub fn new(
        _config: TokenStream2,
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

    pub fn analyze_items(&mut self) {
        let mut arg_types: HashMap<String, String> = HashMap::new();
        self.items.iter_mut().for_each(|item|{
            match item {
                syn::Item::Struct(x) => {
                    let (chain_attrs, other_attrs) = attrs::partition_attributes(x.attrs.clone()).unwrap();
                    for attr in &chain_attrs {
                        if attr.args().len() != 1 {
                            panic!("more that one chain attribute specified in {}", x.ident);
                        }
                        match attr.args().last().unwrap().arg {
                            attrs::AttributeArg::MainStruct => {
                                self.main_struct = Some(x.clone())
                            }
                            attrs::AttributeArg::Packer | attrs::AttributeArg::Table(_) => {
                                self.packers.push(x.clone());
                                x.fields.iter().for_each(|field|{
                                    if let syn::Type::Path(type_path) = &field.ty {
                                        let path_seg = type_path.path.segments.last().unwrap();
                                        let name = path_seg.ident.to_string();
                                        if name == "Option" {
                                            if let syn::PathArguments::AngleBracketed(x) = &type_path.path.segments.last().unwrap().arguments {
                                                if let syn::GenericArgument::Type(tp) = &x.args.last().unwrap() {
                                                    if let syn::Type::Path(type_path) = tp {
                                                        let name = type_path.path.segments.last().unwrap().ident.to_string();
                                                        arg_types.insert(name.clone(), name);
                                                    }
                                                }
                                            }
                                        } else {
                                            arg_types.insert(name.clone(), name);
                                        }
                                    }
                                });
                            }
                            _ => {
                                panic!("only packer or table attribute is supported by struct {}", x.ident);
                            }
                        }
                    }
                    x.attrs = other_attrs;
                    x.fields.iter_mut().for_each(|field| {
                        let (_, other_attrs) = attrs::partition_attributes(field.attrs.clone()).unwrap();
                        field.attrs = other_attrs;
                    });
                    self.structs.push(x.clone());
                }
                syn::Item::Impl(x) => {
                    for impl_item in &mut x.items {
                        match impl_item {
                            syn::ImplItem::Method(method_item) => {
                                method_item.sig.inputs.iter().for_each(|arg|{
                                    match arg {
                                        syn::FnArg::Receiver(_x) => {
                                            //
                                        }
                                        syn::FnArg::Typed(x) => {
                                            if let syn::Type::Path(type_path) = &*x.ty {
                                                let path_seg = type_path.path.segments.last().unwrap();
                                                let name = path_seg.ident.to_string();
                                                if name == "Option" {
                                                    if let syn::PathArguments::AngleBracketed(x) = &type_path.path.segments.last().unwrap().arguments {
                                                        if let syn::GenericArgument::Type(tp) = &x.args.last().unwrap() {
                                                            if let syn::Type::Path(type_path) = tp {
                                                                let name = type_path.path.segments.last().unwrap().ident.to_string();
                                                                arg_types.insert(name.clone(), name);
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    arg_types.insert(name.clone(), name);
                                                }        
                                            }
                                            if let syn::Pat::Ident(pat_ident) = &*x.pat {
                                                pat_ident.ident.to_string();
                                            }
                                        }
                                    }
                                });
                                let (chain_attrs, other_attrs) = attrs::partition_attributes(method_item.attrs.clone()).unwrap();
                                method_item.attrs = other_attrs;
                                if chain_attrs.len() > 0 {
                                    let attr = &chain_attrs[0];
                                    if let Some(name) = attr.action_name() {
                                        self.actions.push(
                                            Action{
                                                item: method_item.clone(),
                                                is_notify: attr.is_notify(),
                                                action_name: name,
                                            }
                                        )
                                    }
                                }
                            }
                            _ => {
                                // contract.others.push(item.clone());
                            }
                        }
                    }
                }
                //TODO: parse field in enum
                syn::Item::Enum(x) => {
                    self.variants.push(x.clone());
                }
                _ => {}
            }
        });

        for (ty, _) in arg_types {
            println!("++++++++ty: {}", ty);
            self.add_packer(&ty);
        }
        
    }
    
    pub fn add_packer(&mut self, name: &str) {
        match name {
            "String" | "Name" | "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" => {
                return;
            }
            _ => {}
        }

        let mut names: Vec<String> = Vec::new();
        self.items.iter().any(|item|{
            match item {
                syn::Item::Struct(x) => {
                    if x.ident.to_string() != name {
                        return false;
                    }
                    self.packers.push(x.clone());
                    for field in &x.fields {
                        if let syn::Type::Path(type_path) = &field.ty {
                            //TODO: construct full path
                            let path_seg = type_path.path.segments.last().unwrap();
                            let name = path_seg.ident.to_string();
                            names.push(name);
                        }
                    }
                    return true;
                }
                syn::Item::Enum(_x) => {
                    return false;
                }
                _ => {
                    return false;
                }
            }
        });

        for name in &names {
            self.add_packer(name);
        }
        return;
    }

    fn generate_code_for_packers(&self) -> TokenStream2 {
        let packers_code = self.packers.iter().map(move |packer| {
            let span = packer.span();
            let ident = &packer.ident;
            // let (_, other_attrs) = attrs::partition_attributes(packer.attrs.clone()).unwrap();

            let serialize = packer.fields.iter().map(|packer_field| {
                let span = packer_field.span();
                let ident = &packer_field.ident;
                let ty = &packer_field.ty;
                quote_spanned!(span=>
                    enc.pack::<#ty>(&self.#ident);
                )
            });

            let deserialize = packer.fields.iter().map(|packer_field| {
                let span = packer_field.span();
                let ident = &packer_field.ident;
                let ty = &packer_field.ty;
                quote_spanned!(span=>
                    dec.unpack::<#ty>(&mut self.#ident);
                )
            });

            let get_size = packer.fields.iter().map(|packer_field| {
                let span = packer_field.span();
                let ident = &packer_field.ident;
                quote_spanned!(span=>
                    _size += self.#ident.size();
                )
            });

            let packed = quote_spanned!(span =>
                impl ::eosio_chain::serializer::Packer for #ident {
                    fn size(&self) -> usize {
                        let mut _size: usize = 0;
                        #( #get_size )*
                        return _size;
                    }
                
                    fn pack(&self) -> Vec<u8> {
                        let mut enc = ::eosio_chain::serializer::Encoder::new(self.size());
                        #( #serialize )*
                        return enc.get_bytes();
                    }
                
                    fn unpack(&mut self, data: &[u8]) -> usize {
                        let mut dec = ::eosio_chain::serializer::Decoder::new(data);
                        #( #deserialize )*
                        return dec.get_pos();
                    }
                }
            );
            quote_spanned!(span =>
                // #( #other_attrs )*
                // #[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
                // #[derive(Default)]
                #packed
            )
        });
        quote! {
            #( #packers_code )*
        }
    }

    fn generate_action_structs(&self) -> TokenStream2 {
        let action_structs_code = self.actions.iter().map(|action|{
            let item = &action.item;
            let span = item.span();
            let ident = &item.sig.ident;
            let struct_name = "_".to_string() + &ident.to_string(); 
            let struct_name_ident = proc_macro2::Ident::new(&struct_name, proc_macro2::Span::call_site());

            let fields = item.sig.inputs.iter().filter(|arg| {
                if let syn::FnArg::Typed(_) = arg {
                    true
                } else {
                    false
                }
            });

            let serialize = item.sig.inputs.iter().map(|arg| {
                if let syn::FnArg::Typed(pat_type) = arg {
                    let span = arg.span();
                    let ty = &*pat_type.ty;
                    if let syn::Pat::Ident(x) = &*pat_type.pat {
                        quote_spanned!(span=>
                            enc.pack::<#ty>(&self.#x);
                        )
                    } else {
                        quote!{}
                    }
                } else {
                    quote!{}
                }
            });

            let deserialize = item.sig.inputs.iter().map(|arg| {
                if let syn::FnArg::Typed(pat_type) = arg {
                    let span = arg.span();
                    let ty = &*pat_type.ty;
                    if let syn::Pat::Ident(x) = &*pat_type.pat {
                        quote_spanned!(span=>
                            dec.unpack::<#ty>(&mut self.#x);
                        )
                    } else {
                        quote!{}
                    }
                } else {
                    quote!{}
                }
            });

            let get_size = item.sig.inputs.iter().map(|arg| {
                if let syn::FnArg::Typed(pat_type) = arg {
                    let span = arg.span();
                    // let ty = &*pat_type.ty;
                    if let syn::Pat::Ident(x) = &*pat_type.pat {
                        quote_spanned!(span=>
                            _size += self.#x.size();
                        )
                    } else {
                        quote!{}
                    }
                } else {
                    quote!{}
                }
            });

            let error_name = struct_name_ident.to_string() + ".unpack: buffer overflow";
            let error_lit = proc_macro2::Literal::string(&error_name);

            let packed = quote_spanned!(span =>
                impl ::eosio_chain::serializer::Packer for #struct_name_ident {
                    fn size(&self) -> usize {
                        #[allow(unused_mut)]
                        let mut _size: usize = 0;
                        #( #get_size )*
                        return _size;
                    }

                    fn pack(&self) -> Vec<u8> {
                        #[allow(unused_mut)]
                        let mut enc = ::eosio_chain::serializer::Encoder::new(self.size());
                        #( #serialize )*
                        return enc.get_bytes();
                    }
                
                    fn unpack<'a>(&mut self, data: &'a [u8]) -> usize {
                        check(data.len() >= self.size(), #error_lit);
                        #[allow(unused_mut)]
                        let mut dec = ::eosio_chain::serializer::Decoder::new(data);
                        #( #deserialize )*
                        return dec.get_pos();
                    }
                }
            );
            quote! {
                #[derive(Default)]
                struct #struct_name_ident {
                    # ( #fields ), *
                }
                #packed
            }
        });
        quote!{
            #( #action_structs_code ) *
        }
    }

    fn generate_action_handle_code(&self, notify: bool) -> TokenStream2 {
        let actions = self.actions.iter().filter(|action|{
            if notify {
                if action.is_notify {
                    return true;
                }
                return false;
            } else {
                if !action.is_notify {
                    return true;
                }
                return false;
            }
        });

        let action_structs_code = actions.map(|action|{
            let item = &action.item;
            let ident = &item.sig.ident;
            let struct_name = "_".to_string() + &ident.to_string(); 
            let struct_name_ident = proc_macro2::Ident::new(&struct_name, proc_macro2::Span::call_site());

            let action_name_n = proc_macro2::Literal::u64_suffixed(s2n(&action.action_name.str()));

            let args = item.sig.inputs.iter().filter(|arg|{
                if let syn::FnArg::Typed(_) = arg {
                    return true;
                }
                return false;
            });

            let args = args.map(|arg| {
                if let syn::FnArg::Typed(pat_type) = arg {
                    let span = arg.span();
                    if let syn::Pat::Ident(x) = &*pat_type.pat {
                        quote_spanned!(span=>
                            action.#x
                        )
                    } else {
                        quote!{}
                    }
                } else {
                    quote!{}
                }
            });

            quote! {
                #action_name_n => {
                    let mut action: #struct_name_ident = Default::default();
                    action.unpack(&::eosio_chain::vmapi::eosio::read_action_data());
                    contract.#ident(#( #args ),*);
                }
            }
        });
        quote! {
            #( #action_structs_code ) *
        }
    }

    fn generate_apply_code(&self) -> TokenStream2 {
        if self.main_struct.is_none() {
            return quote!{};
        }

        let ident = &self.main_struct.as_ref().unwrap().ident;
        let notify_handle_code = self.generate_action_handle_code(true);
        let action_handle_code = self.generate_action_handle_code(false);
        quote!{
            #[no_mangle]
            fn apply(receiver: u64, first_receiver: u64, action: u64) {
                let _receiver = Name{n: receiver};
                let _first_receiver = Name{n: first_receiver};
                let _action = Name{n: action};
                #[allow(unused_mut)]
                let mut contract: #ident = #ident::new(_receiver, _first_receiver, _action);
                if receiver == first_receiver {
                    match action {
                        #action_handle_code
                        _ => {}
                    }
                }

                if receiver != first_receiver {
                    match action {
                        #notify_handle_code
                        _ => {}
                    }
                }                
            }
        }
    }

    pub fn generate_code(&self) -> TokenStream2 {
        let action_structs_code = self.generate_action_structs();
        let apply_code = self.generate_apply_code();
        let packers_code = self.generate_code_for_packers();
        let items = self.items.iter().map(|item|{
            match item {
                syn::Item::Struct(x) => {
                    if self.packers.iter().any(|packer|{
                        packer.ident == x.ident
                    }) {
                        quote!{
                            #[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
                            #[derive(Default)]
                            #item
                        }
                    } else {
                        quote!{
                            #item
                        }
                    }
                }
                syn::Item::Enum(_) => {
                    quote!{
                        #[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
                        #[derive(Default)]
                        #item
                    }
                }
                _ => {
                    quote!{
                        #item
                    }
                }
            }
        });
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
                #packers_code
                #action_structs_code
                #apply_code
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


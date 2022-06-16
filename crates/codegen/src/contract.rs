use std::collections::HashMap;

use core::convert::TryFrom;
use itertools::Itertools;
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
    table::Table,
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
    tables: Vec<Table>,
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
            tables: Vec::new(),
            others: Vec::new(),
        };
        contract.analyze_items()?;
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

    fn verify_variant(item: &syn::ItemEnum) -> Result<(), syn::Error> {
        for v in &item.variants {
            match v.fields {
                syn::Fields::Unnamed(_) => {}
                _ => {
                    return Err(format_err_spanned!(
                        v.fields,
                        "invalid variant field"
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn analyze_items(&mut self) -> Result<(), syn::Error> {
        let mut arg_types: HashMap<String, String> = HashMap::new();
        for item in &mut self.items {
            match item {
                syn::Item::Struct(ref mut x) => {
                    let (chain_attrs, other_attrs) = attrs::partition_attributes(x.attrs.clone()).unwrap();
                    let x_backup = x.clone();
                    x.attrs = other_attrs;
                    x.fields.iter_mut().for_each(|field| {
                        let (_, other_attrs) = attrs::partition_attributes(field.attrs.clone()).unwrap();
                        field.attrs = other_attrs;
                    });
                    self.structs.push(x.clone());

                    if chain_attrs.len() == 0 {
                        continue;
                    }

                    if chain_attrs.len() > 1 {
                        return Err(format_err_spanned!(
                            x,
                            "more than one chain attribute specified to struct {}", x.ident
                        ));
                    }

                    let attr = &chain_attrs[0];
                    if attr.args().len() < 1 {
                        return Err(format_err_spanned!(
                            x,
                            "wrong chain attribute in {}", x.ident
                        ));
                    }

                    let arg = &attr.args().next().unwrap().arg;
                    match arg {
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
                            return Err(format_err_spanned!(
                                x,
                                "only packer or table attribute is supported by struct {}", x.ident
                            ));
                        }
                    }

                    match &arg {
                        attrs::AttributeArg::Table(_) => {
                            if let Some(name) = attr.table_name() {
                                self.tables.push(
                                    Table {
                                        item: x_backup,
                                        table_name: name,
                                        singleton: attr.is_singleton(),
                                    }
                                )
                            }
                        }
                        _ => {}
                    }
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

                syn::Item::Enum(x) => {
                    let (chain_attrs, other_attrs) = attrs::partition_attributes(x.attrs.clone())?;
                    if chain_attrs.len() == 0 {
                        continue;
                    }

                    if chain_attrs.len() > 1 {
                        return Err(format_err_spanned!(
                            x,
                            "more than one chain attribute specified to struct {}", x.ident
                        ));
                    }

                    let attr = &chain_attrs[0];
                    if attr.args().len() < 1 {
                        return Err(format_err_spanned!(
                            x,
                            "wrong chain attribute in {}", x.ident
                        ));
                    }

                    x.attrs = other_attrs;
                    let arg = &attr.args().next().unwrap().arg;
                    if attrs::AttributeArg::Variant == *arg {
                        Self::verify_variant(x)?;
                        self.variants.push(x.clone());
                    } else {
                        return Err(format_err_spanned!(
                            x,
                            "only variant attribute supported for enum {}", x.ident
                        ));
                    }
                }
                _ => {}
            }
        };

        for (ty, _) in arg_types {
            self.add_packer(&ty)?;
        }
        return Ok(())
        
    }
    
    pub fn add_packer(&mut self, name: &str) -> Result<(), syn::Error> {
        match name {
            "String" | "Name" | "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" => {
                return Ok(());
            }
            _ => {}
        }

        let mut names: Vec<String> = Vec::new();
        for item in &self.items {
            match item {
                syn::Item::Struct(x) => {
                    if x.ident.to_string() != name {
                        continue;
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
                    break;
                }
                syn::Item::Enum(x) => {
                    if x.ident.to_string() != name {
                        continue;
                    }
                    Self::verify_variant(x)?;
                    self.variants.push(x.clone());
                    break;
                }
                _ => {}
            }
        }

        for name in &names {
            self.add_packer(name)?;
        }
        Ok(())
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
            let struct_name = action.action_name.str();
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
                #[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
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

    fn generate_tables_code(&self) -> Result<TokenStream2, syn::Error> {
        let mut action_structs_code: Vec<TokenStream2> = Vec::new();
        for table in &self.tables {
            let item = &table.item;
            let span = item.span();
            let table_ident = &item.ident;
            let mut primary_impl: Option<TokenStream2> = None;

            for field in &item.fields {
                let (chain_attrs, _) = attrs::partition_attributes(field.attrs.clone()).unwrap();
                if chain_attrs.len() == 0 {
                    continue;
                }
                let field_ident = field.ident.as_ref().expect(&format!("invalid field in {}", table_ident).to_string());
                let attr = &chain_attrs[0];
                
                if attr.args().len() == 0 {
                    return Err(format_err_spanned!(
                        field,
                        "no chain attribute specified for {}", field.ident.as_ref().unwrap(),
                    ));
                }

                let first_attr = attr.args().next().unwrap();
                match first_attr.arg {
                    attrs::AttributeArg::Primary => {
                        if primary_impl.is_some() {
                            return Err(format_err_spanned!(
                                field,
                                "more than one primary field specified in {}", item.ident
                            ));
                        }
                        primary_impl = Some(quote_spanned!(span =>
                            fn get_primary(&self) -> u64 {
                                return self.#field_ident.to_primary_value();
                            }
                        ))
                    }
                    _ => {}
                }
            };

            if table.singleton {
                if primary_impl.is_some() {
                    return Err(format_err_spanned!(
                        item,
                        "singelton table does not need a primary attribute in struct {}", item.ident
                    ));
                }
            } else {
                if primary_impl.is_none() {
                    return Err(format_err_spanned!(
                        item,
                        "primary index does not specified in struct {}", item.ident
                    ));
                }
            }

            let mut secondary_fields: Vec<(attrs::AttributeArg, syn::Field)> = Vec::new();

            for field in &item.fields {
                let (chain_attrs, _) = attrs::partition_attributes(field.attrs.clone()).unwrap();
                if chain_attrs.len() == 0 {
                    continue;
                }

                let attr = &chain_attrs[0];
                if attr.args().len() == 0 {
                    return Err(format_err_spanned!(
                        field,
                        "no chain attribute specified",
                    ));
                }

                let first_attr = attr.args().next().unwrap();
                match first_attr.arg {
                    attrs::AttributeArg::Primary => {},
                    attrs::AttributeArg::Idx64(_) |
                    attrs::AttributeArg::Idx128(_) |
                    attrs::AttributeArg::Idx256(_) |
                    attrs::AttributeArg::IdxF64(_) |
                    attrs::AttributeArg::IdxF128(_) => {
                        secondary_fields.push((first_attr.arg.clone(), field.clone()));
                    }
                    _ => {
                        return Err(format_err_spanned!(
                            field,
                            "invalid attribute"
                        ));
                    }
                }
            };

            let secondary_getter_impls = secondary_fields.iter()
            .enumerate()
            .map(|(index, (attr_arg, field))|{
                let field_ident = field.ident.as_ref().unwrap();
                let error_msg: proc_macro2::Literal;
                let idx_ident: proc_macro2::Ident;

                match attr_arg {
                    attrs::AttributeArg::Idx64(_) => {
                        idx_ident = proc_macro2::Ident::new("Idx64", proc_macro2::Span::call_site());
                        error_msg = proc_macro2::Literal::string("Invalid Idx64 value!");
                    }
                    attrs::AttributeArg::Idx128(_) => {
                        idx_ident = proc_macro2::Ident::new("Idx128", proc_macro2::Span::call_site());
                        error_msg = proc_macro2::Literal::string("Invalid Idx128 value!");
                    }
                    attrs::AttributeArg::Idx256(_) => {
                        idx_ident = proc_macro2::Ident::new("Idx256", proc_macro2::Span::call_site());
                        error_msg = proc_macro2::Literal::string("Invalid Idx256 value!");
                    }
                    attrs::AttributeArg::IdxF64(_) => {
                        idx_ident = proc_macro2::Ident::new("IdxF64", proc_macro2::Span::call_site());
                        error_msg = proc_macro2::Literal::string("Invalid IdxF64 value!");
                    }
                    attrs::AttributeArg::IdxF128(_) => {
                        idx_ident = proc_macro2::Ident::new("IdxF128", proc_macro2::Span::call_site());
                        error_msg = proc_macro2::Literal::string("Invalid IdxF128 value!");
                    }
                    _ => {
                        idx_ident = proc_macro2::Ident::new("", proc_macro2::Span::call_site());
                        error_msg = proc_macro2::Literal::string("Invalid value!");
                    }
                }
                return quote! {
                    if i == #index {
                        let ret = self.#field_ident.to_secondary_value(eosio_chain::db::SecondaryType::#idx_ident);
                        match ret {
                            SecondaryValue::#idx_ident(_) => {
                                return ret;
                            }
                            _ => {
                                ::eosio_chain::vmapi::eosio::eosio_assert(false, #error_msg);
                            }
                        }
                        return ret;
                    }
                }
            });

            let secondary_setter_impls = secondary_fields.iter()
            .enumerate()
            .map(|(index, (_attr_arg, field))|{
                let field_ident = field.ident.as_ref().unwrap();
                let ty = &field.ty;
                return quote!{
                    if i == #index {
                        self.#field_ident = #ty::from_secondary_value(value);
                    }
                }
            });

            let secondary_impls = quote_spanned!(span =>    
                #[allow(unused_variables, unused_mut)]
                fn get_secondary_value(&self, i: usize) -> SecondaryValue {
                    #( #secondary_getter_impls )*
                    return SecondaryValue::None;
                }

                #[allow(unused_variables, unused_mut)]
                fn set_secondary_value(&mut self, i: usize, value: SecondaryValue) {
                    #( #secondary_setter_impls )*
                }
            );

            let mi_impls = self.generate_mi_impls(table, &secondary_fields);

            if !table.singleton {
                let primary_impl_code = primary_impl.unwrap();
                action_structs_code.push(quote_spanned!(span =>
                    impl ::eosio_chain::db::DBInterface for #table_ident {
                        #primary_impl_code
                        #secondary_impls
                    }
                    #mi_impls
                ));
            } else {
                let table_name = proc_macro2::Literal::string(&table.table_name.str());
                action_structs_code.push(quote_spanned!(span =>
                    impl ::eosio_chain::db::DBInterface for #table_ident {
                        fn get_primary(&self) -> u64 {
                            return Name::new(#table_name);
                        }
                        #secondary_impls
                    }
                    #mi_impls
                ));
            }
        };

        return Ok(quote!{
            #( #action_structs_code ) *
        });
    }

    fn generate_mi_impls(&self, table: &Table, secondary_fields: &Vec<(attrs::AttributeArg, syn::Field)>) -> TokenStream2 {
        let table_name = table.table_name.str();

        let span = table.item.span();
        let table_ident = &table.item.ident;


        let len_secondary = secondary_fields.len();

        let secondary_types = secondary_fields
            .iter()
            .enumerate()
            .map(|(_n, (arg, _))| {
                match arg {
                    attrs::AttributeArg::Idx64(_) => {
                        return quote! {
                            SecondaryType::Idx64
                        }
                    }
                    attrs::AttributeArg::Idx128(_) => {
                        return quote! {
                            SecondaryType::Idx128
                        }
                    }
                    attrs::AttributeArg::Idx256(_) => {
                        return quote! {
                            SecondaryType::Idx256
                        }
                    }
                    attrs::AttributeArg::IdxF64(_) => {
                        return quote! {
                            SecondaryType::IdxF64
                        }
                    }
                    attrs::AttributeArg::IdxF128(_) => {
                        return quote! {
                            SecondaryType::IdxF128
                        }
                    }
                    _ => {
                        quote!{}
                    }
                }
            });

            let get_idx_db_funcs = secondary_fields
                .iter()
                .enumerate()
                .map(|(i, (idx, field))| {
                    match idx {
                        attrs::AttributeArg::Idx64(_) |
                        attrs::AttributeArg::Idx128(_) |
                        attrs::AttributeArg::Idx256(_) |
                        attrs::AttributeArg::IdxF64(_) |
                        attrs::AttributeArg::IdxF128(_)
                         => {
                            let span = field.span();
                            let ty = &field.ty;
                            let method_name = String::from("get_idx_by_") + &field.ident.as_ref().unwrap().to_string();
                            let method_ident = syn::Ident::new(&method_name, span);
                            return quote_spanned!(span =>
                                #[allow(dead_code)]
                                fn #method_ident(&self) -> ::eosio_chain::db::IndexDBProxy<#ty, 0> {
                                    return ::eosio_chain::db::IndexDBProxy::<#ty, 0>::new(self.mi.idxdbs[#i].as_ref());
                                }
                            )
                        }
                        _ => return quote!(),
                    };
                });

            let mi_name = table_ident.to_string() + "MultiIndex";
            let mi_ident = syn::Ident::new(&mi_name, span);
    
            return quote_spanned!(span =>

                pub struct #mi_ident {
                    mi: ::eosio_chain::mi::MultiIndex<#table_ident>
                }
            
                #[allow(dead_code)]
                impl #mi_ident {
                    ///
                    pub fn new(code: Name, scope: Name, table: Name, indexes: &[SecondaryType], unpacker: fn(&[u8]) -> Box<#table_ident>) -> Self {
                        Self {
                            mi: ::eosio_chain::mi::MultiIndex::<#table_ident>::new(code, scope, table, indexes, unpacker),
                        }
                    }
            
                    ///
                    pub fn store(&self, value: &#table_ident, payer: Name) -> ::eosio_chain::db::Iterator {
                        return self.mi.store(value, payer);
                    }
                
                    ///
                    pub fn update(&self, iterator: ::eosio_chain::db::Iterator, value: &#table_ident, payer: Name) {
                        return self.mi.update(iterator, value, payer);
                    }
                
                    ///
                    pub fn remove(&self, iterator: ::eosio_chain::db::Iterator) {
                        return self.mi.remove(iterator);
                    }
                
                    ///
                    pub fn get(&self, iterator: ::eosio_chain::db::Iterator) -> Option<Box<#table_ident>> {
                        return self.mi.get(iterator)
                    }
                
                    ///
                    pub fn get_by_primary(&self, primary: u64) -> Option<Box<#table_ident>> {
                        return self.mi.get_by_primary(primary);
                    }

                    ///
                    pub fn next(&self, iterator: ::eosio_chain::db::Iterator) -> (::eosio_chain::db::Iterator, u64) {
                        return self.mi.db.next(iterator);
                    }

                    ///
                    pub fn previous(&self, iterator: ::eosio_chain::db::Iterator) -> (::eosio_chain::db::Iterator, u64) {
                        return self.mi.db.previous(iterator);
                    }

                    ///
                    pub fn find(&self, id: u64) -> ::eosio_chain::db::Iterator {
                        return self.mi.db.find(id);
                    }
                
                    ///
                    pub fn lowerbound(&self, id: u64) -> ::eosio_chain::db::Iterator {
                        return self.mi.db.lowerbound(id);
                    }
                
                    ///
                    pub fn upperbound(&self, id: u64) -> ::eosio_chain::db::Iterator {
                        return self.mi.db.upperbound(id);
                    }
                
                    ///
                    pub fn end(&self) -> ::eosio_chain::db::Iterator {
                        return self.mi.db.end();
                    }
                
                    ///
                    pub fn get_idx_db(&self, i: usize) -> &dyn ::eosio_chain::db::IndexDB {
                        return self.mi.idxdbs[i].as_ref();
                    }
                
                    ///
                    pub fn idx_update(&self, it: ::eosio_chain::db::SecondaryIterator, value: ::eosio_chain::db::SecondaryValue, payer: Name) {
                        self.mi.idx_update(it, value, payer);
                    }

                    #( #get_idx_db_funcs )*
                }

                impl #table_ident {
                    #[allow(dead_code)]
                    fn new_mi(code: Name, scope: Name) -> Box<#mi_ident> {
                        let indexes: [SecondaryType; #len_secondary] = [#( #secondary_types ),*];
                        #[allow(dead_code)]
                        fn unpacker(data: &[u8]) -> Box<#table_ident> {
                            let mydata = #table_ident::default();
                            let mut ret = Box::new(mydata);// as Box<dyn MultiIndexValue>;
                            ret.unpack(data);
                            return ret;
                        }
                        return Box::new(#mi_ident::new(code, scope, Name::new(#table_name), &indexes, unpacker));
                    }
                }
            );
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
            let struct_name = action.action_name.str();
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

    fn gather_scale_info(&self) -> TokenStream2 {
        let packer_scale_info_code = self.packers
            .iter()
            .map(|packer| {
                let ident = &packer.ident;
                quote!{
                    info.structs.push(#ident::type_info());
                }
            });
        
        let packer_scale_info_code2 = self.actions
            .iter()
            .map(|action| {
                let struct_name_ident = proc_macro2::Ident::new(&action.action_name.str(), proc_macro2::Span::call_site());
                quote!{
                    info.structs.push(#struct_name_ident::type_info());
                }
            });

        let table_scale_info_code = self.tables
            .iter()
            .map(|table| {
                let ident = &table.item.ident;
                let table_name_lit = proc_macro2::Literal::string(&table.table_name.str());
                quote!{
                    info.tables.push(
                        ::eosio_chain::abi::TableInfo {
                            name: String::from(#table_name_lit),
                            info: #ident::type_info(),
                        });
                }
            });

        let action_scale_info_code = self.actions
            .iter()
            .map(|action| {
                let ident = &action.item.sig.ident;
                let struct_name = action.action_name.str();
                let action_name_lit = proc_macro2::Literal::string(&action.action_name.str());

                let struct_name_ident = proc_macro2::Ident::new(&struct_name, proc_macro2::Span::call_site());
                quote!{
                    info.actions.push(
                        ::eosio_chain::abi::ActionInfo {
                            name: String::from(#action_name_lit),
                            info: #struct_name_ident::type_info(),
                        });
                }
            });

        let variants_scale_info_code = self.variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;
                quote!{
                    info.variants.push(#ident::type_info());
                }
            });

        return quote!{
            #[cfg(feature = "std")]
            const _: () = {
                #[no_mangle]
                pub fn __eosio_generate_abi() -> String {
                    let mut info = ::eosio_chain::abi::ABIInfo {
                        actions: Vec::new(),
                        tables: Vec::new(),
                        structs: Vec::new(),
                        variants: Vec::new(),
                    };
                    #( #packer_scale_info_code ) *
                    #( #packer_scale_info_code2 ) *
                    #( #action_scale_info_code ) *
                    #( #table_scale_info_code ) *
                    #( #variants_scale_info_code ) *
                    return ::eosio_chain::abi::parse_abi_info(&info);
                }
            };
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

    fn generate_variants_code(&self) -> Result<TokenStream2, syn::Error> {
        //
        let variants_code = self.variants.iter().map(|item| {
            let first_field = &item.variants.iter().next().unwrap().ident;
            let span = item.span();
            let variant_ident = &item.ident;
            let pack_code = item.variants
                .iter()
                .enumerate()
                .map(|(i, field)| {
                    let field_ident = &field.ident;
        
                    let index = syn::LitInt::new(&i.to_string(), proc_macro2::Span::call_site());
                    if let syn::Fields::Unnamed(_) = &field.fields {
                        // let ty = &x.unnamed.last().unwrap().ty;
                        return quote!{
                            #variant_ident::#field_ident(x) => {
                                let mut i: u8 = #index as u8;
                                enc.pack(&i);
                                enc.pack(x);
                            }
                        }
                    } else {
                        quote!{}
                    }
            });

            let unpack_code = item.variants
                .iter()
                .enumerate()
                .map(|(i,field)| {
                    let field_ident = &field.ident;
                    let index = syn::LitInt::new(&i.to_string(), proc_macro2::Span::call_site());
                    if let syn::Fields::Unnamed(x) = &field.fields {
                        let ty = &x.unnamed.last().unwrap().ty;
                        quote!{
                            #index => {
                                let mut v: #ty = Default::default();
                                dec.unpack(&mut v);
                                *self = #variant_ident::#field_ident(v);
                            }
                        }
                    } else {
                        quote!{}
                    }
            });

            let getsize_code = item.variants
                .iter()
                .enumerate()
                .map(|(i,field)| {
                    let field_ident = &field.ident;
                    let index = syn::LitInt::new(&i.to_string(), proc_macro2::Span::call_site());
                    quote!{
                        #variant_ident::#field_ident(x) => {
                            _size = 1 + x.size();
                        }
                    }
            });

            quote_spanned!(span =>
                impl Default for #variant_ident {
                    ///
                    #[inline]
                    fn default() -> Self {
                        #variant_ident::#first_field(Default::default())
                    }
                }

                impl ::eosio_chain::serializer::Packer for #variant_ident {
                    fn size(&self) -> usize {
                        let mut _size: usize = 0;
                        match self {
                            #( #getsize_code )*
                            _=> {}
                        }
                        return _size;
                    }
                
                    fn pack(&self) -> Vec<u8> {
                        let mut enc = ::eosio_chain::serializer::Encoder::new(self.size());
                        match self {
                            #( #pack_code )*
                            _=> {}
                        }
                        return enc.get_bytes();
                    }
                
                    fn unpack<'a>(&mut self, data: &'a [u8]) -> usize {
                        let mut dec = ::eosio_chain::serializer::Decoder::new(data);
                        let mut variant_type_index: u8 = 0;
                        dec.unpack(&mut variant_type_index);
                        match variant_type_index {
                            #( #unpack_code )*
                            _ => {
                                ::eosio_chain::vmapi::eosio::eosio_assert(false, "bad variant index!");
                            }
                        }
                        return dec.get_pos();
                    }
                }
            )
        });
        Ok(
            quote!{
                #( #variants_code ) *
            }
        )
    }

    pub fn generate_code(&self) -> Result<TokenStream2, syn::Error> {
        let action_structs_code = self.generate_action_structs();
        let tables_code = self.generate_tables_code()?;
        let apply_code = self.generate_apply_code();
        let packers_code = self.generate_code_for_packers();
        let variants_code = self.generate_variants_code()?;
        let scale_info = self.gather_scale_info();

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
                syn::Item::Enum(x) => {
                    if self.variants.iter().any(|variant|{
                        variant.ident == x.ident
                    }) {
                        quote!{
                            #[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
                            // #[derive(Default)]
                            #item
                        }    
                    } else {
                        quote!{
                            #item
                        }
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
        Ok(quote! {
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

            #[cfg(feature = "std")]
            use eosio_scale_info::TypeInfo as _;

            #( #attrs )*
            #vis mod #ident {
                use super::*;
                #( #items ) *
                #packers_code
                #variants_code
                #action_structs_code
                #tables_code
                #apply_code
                #scale_info
            }
        })
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


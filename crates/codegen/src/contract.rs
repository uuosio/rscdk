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
    table::Table,
    attrs,
    // FixedString,
    name::{
        s2n,
        is_name_valid,
    },
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

        let (_, other_attrs) = attrs::partition_attributes(module.attrs.clone())?;

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
        return self.main_struct.is_some();
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

    fn check_struct_name(item: &syn::ItemStruct) -> Result<(), syn::Error> {
        if let Some(_) = item.ident.to_string().find("_") {
            println!("++++++++item.ident:{}", item.ident);
            return Err(format_err_spanned!(
                item,
                "structs with `_` in name are not supported by contract"
            ));
        }
        return Ok(());
    }

    pub fn has_trait(&self, s: &str, trait_: &str) -> bool {
        for item in &self.items {
            match item {
                syn::Item::Impl(x) => {
                    if let Some((_, trait_path, _)) = &x.trait_ {
                        if let Some(segment) = trait_path.segments.last() {
                            if trait_ == segment.ident.to_string() {
                                if let syn::Type::Path(ty) = &*x.self_ty {
                                    if let Some(segment) = ty.path.segments.last() {
                                        if segment.ident.to_string() == s {
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        return false;
    }

    pub fn has_primary_value_interface_trait(&self, s: &str) -> bool {
        return self.has_trait(s, "PrimaryValueInterface");
    }

    pub fn has_secondary_value_interface_trait(&self, s: &str) -> bool {
        return self.has_trait(s, "SecondaryValueInterface");
    }

    pub fn analyze_items(&mut self) -> Result<(), syn::Error> {
        let mut arg_types: HashMap<String, String> = HashMap::new();
        for item in &mut self.items {
            match item {
                syn::Item::Struct(ref mut x) => {
                    Self::check_struct_name(&x)?;
                    let (chain_attrs, other_attrs) = attrs::partition_attributes(x.attrs.clone())?;
                    let x_backup = x.clone();
                    x.attrs = other_attrs;
                    for field in &mut x.fields {
                        let (_, other_attrs) = attrs::partition_attributes(field.attrs.clone())?;
                        field.attrs = other_attrs;
                    }

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
                            let length = x.fields.len();
                            for (i, field) in &mut x.fields.iter_mut().enumerate() {
                                if Self::is_binary_extension_type(&field.ty) {
                                    if i + 1 != length {
                                        return Err(format_err_spanned!(
                                            field,
                                            "BinaryExtension type can only appear at the last field of a struct",
                                        ));
                                    }
                                }
                            }
                            self.packers.push(x.clone());
                            for field in &x.fields {
                                let (type_name, _) = Self::extract_type(&field.ty)?;
                                arg_types.insert(type_name.clone(), type_name);
                            };
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
                                if !is_name_valid(&name.str()) {
                                    return Err(format_err_spanned!(
                                        attr.args().next().unwrap().ast,
                                        "table name contain invalid character(s), valid charaters are a-z & 1-5: {}", name.str()
                                    ));
                                }
                                if self.tables.iter().any(|table| {
                                    table.table_name == name
                                }) {
                                    return Err(format_err_spanned!(
                                        attr.args().next().unwrap().ast,
                                        "dumplicated table name: {}", name.str()
                                    ));
                                }
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
                                let (chain_attrs, other_attrs) = attrs::partition_attributes(method_item.attrs.clone())?;
                                method_item.attrs = other_attrs;
                                if chain_attrs.len() <= 0 {
                                    continue;
                                }
                                if chain_attrs.len() > 1 {
                                    return Err(format_err_spanned!(
                                        chain_attrs[1].args().next().unwrap().ast,
                                        "only one chain attribute supported"
                                    ));
                                }
                                let attr = &chain_attrs[0];
                                if let Some(name) = attr.action_name() {
                                    if !is_name_valid(&name.str()) {
                                        return Err(format_err_spanned!(
                                            attr.args().next().unwrap().ast,
                                            "action name contain invalid character(s), valid charaters are a-z & 1-5: {}", name.str()
                                        ));
                                    }
                                    if self.actions.iter().any(|action| {
                                        action.action_name == name
                                    }) {
                                        return Err(format_err_spanned!(
                                            attr.args().next().unwrap().ast,
                                            "dumplicated action name: {}", name.str()
                                        ));
                                    }

                                    let length = method_item.sig.inputs.len();
                                    for (i, arg) in method_item.sig.inputs.iter().enumerate() {
                                        match arg {
                                            syn::FnArg::Receiver(_) => {}
                                            syn::FnArg::Typed(x) => {
                                                if Self::is_binary_extension_type(&x.ty) {
                                                    if i + 1 != length {
                                                        return Err(format_err_spanned!(
                                                            x,
                                                            "BinaryExtension type can only appear at the last argument of a method",
                                                        ));
                                                    }
                                                }
                                                let (type_name, _) = Self::extract_type(&x.ty)?;
                                                arg_types.insert(type_name.clone(), type_name);
                                            }
                                        }
                                    };

                                    self.actions.push(
                                        Action{
                                            item: method_item.clone(),
                                            is_notify: attr.is_notify(),
                                            action_name: name,
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
                            "only variant attribute is supported by contract"
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
    
    fn is_primitive_type(name: &str) -> bool {
        match name {
            "bool" | "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f32" | "f64" | "i128" | "u128" |
            "String" |
            "Varint32" | "VarUint32" | "Float128" | "TimePoint" | "TimePointSec" |
            "BlockTimeStampType" | "Name" | "Checksum160" | "Checksum256" | "Uint256" |
            "Checksum512" | "PublicKey" | "Signature" | "Symbol" | "SymbolCode" | "Asset" |
            "ExtendedAsset" => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    pub fn get_type_name(field: &syn::Field) -> Result<String, syn::Error> {
        if let syn::Type::Path(type_path) = &field.ty {
            if type_path.path.segments.len() != 1 {
                return Err(format_err_spanned!(
                    field,
                    "type with multiple segments does not support by contract!"
                ));
            }
            let path_seg = type_path.path.segments.last().unwrap();
            return Ok(path_seg.ident.to_string());
        } else {
            return Err(format_err_spanned!(
                field,
                "Invalid contract type!"
            ));
        }
    }

    pub fn add_packer(&mut self, name: &str) -> Result<(), syn::Error> {
        if Self::is_primitive_type(name) {
            return Ok(());
        }

        let mut names: HashMap<String, bool> = HashMap::new();
        for item in &self.items {
            match item {
                syn::Item::Struct(x) => {
                    if x.ident.to_string() != name {
                        continue;
                    }
                    if !self.packers.iter().any(|packer| {
                        x.ident == packer.ident
                    }) {
                        self.packers.push(x.clone());
                    }

                    let length = x.fields.len();
                    for (i, field) in x.fields.iter().enumerate() {
                        if Self::is_binary_extension_type(&field.ty) {
                            if i + 1 != length {
                                return Err(format_err_spanned!(
                                    field,
                                    "BinaryExtension type can only appear at the last field of a struct",
                                ));
                            }
                        }
                    }

                    for field in &x.fields {
                        let name = Self::get_type_name(field)?;
                        names.insert(name, true);
                    }
                    break;
                }
                syn::Item::Enum(x) => {
                    if x.ident.to_string() != name {
                        continue;
                    }
                    if !self.variants.iter().any(|v| {
                        v.ident == x.ident
                    }) {
                        self.variants.push(x.clone());
                    }
                    for v in &x.variants {
                        match &v.fields {
                            syn::Fields::Unnamed(x) => {
                                if x.unnamed.len() != 1 {
                                    return Err(format_err_spanned!(
                                        x,
                                        "multiple fields in variant does not support by contract!"
                                    ));
                                }
                                let field = x.unnamed.last().unwrap();
                                let name = Self::get_type_name(field)?;
                                names.insert(name, true);
                            }
                            _ => {
                                return Err(format_err_spanned!(
                                    v.fields,
                                    "invalid variant field"
                                ));
                            }
                        }
                    }
                    break;
                }
                _ => {}
            }
        }

        for (name, _) in &names {
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
            // let ident = &item.sig.ident;
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
                        eosio_chain::check(data.len() >= self.size(), #error_lit);
                        #[allow(unused_mut)]
                        let mut dec = ::eosio_chain::serializer::Decoder::new(data);
                        #( #deserialize )*
                        return dec.get_pos();
                    }
                }
            );

            quote! {
                #[cfg_attr(feature = "std", derive(::eosio_chain::eosio_scale_info::TypeInfo))]
                #[cfg_attr(feature = "std", scale_info(crate = ::eosio_chain::eosio_scale_info))]
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
                let (chain_attrs, _) = attrs::partition_attributes(field.attrs.clone())?;
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
                            impl ::eosio_chain::db::PrimaryValueInterface for #table_ident {
                                fn get_primary(&self) -> u64 {
                                    return self.#field_ident.into();
                                }
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
                let (chain_attrs, _) = attrs::partition_attributes(field.attrs.clone())?;
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
                    attrs::AttributeArg::Secondary => {
                        if !Self::is_secondary_type(&field.ty) {
                            return Err(format_err_spanned!(
                                first_attr.ast,
                                "invalid secondary type, only  \"u64\", \"u128\", \"Uint256\", \"f64\" or \"Float128\" supported"
                            ));
                        }
                        secondary_fields.push((first_attr.arg.clone(), field.clone()));
                    }
                    _ => {
                        return Err(format_err_spanned!(
                            first_attr.ast,
                            "invalid attribute argument"
                        ));
                    }
                }
            };

            let secondary_impls;
            if !self.has_secondary_value_interface_trait(&item.ident.to_string()) {
                let secondary_getter_impls = secondary_fields.iter()
                .enumerate()
                .map(|(index, (_, field))|{
                    let field_ident = field.ident.as_ref().unwrap();
                    return quote! {
                        if i == #index {
                            return self.#field_ident.into();
                        }
                    }
                });
    
                let secondary_setter_impls = secondary_fields.iter()
                .enumerate()
                .map(|(index, (_attr_arg, field))|{
                    let field_ident = field.ident.as_ref().unwrap();
                    return quote!{
                        if i == #index {
                            self.#field_ident = value.into();
                        }
                    }
                });
    
                secondary_impls = quote_spanned!(span =>
                    impl ::eosio_chain::db::SecondaryValueInterface for #table_ident {
                        #[allow(unused_variables, unused_mut)]
                        fn get_secondary_value(&self, i: usize) -> eosio_chain::db::SecondaryValue {
                            #( #secondary_getter_impls )*
                            return eosio_chain::db::SecondaryValue::None;
                        }
        
                        #[allow(unused_variables, unused_mut)]
                        fn set_secondary_value(&mut self, i: usize, value: eosio_chain::db::SecondaryValue) {
                            #( #secondary_setter_impls )*
                        }
                    }
                );
            } else {
                secondary_impls = quote!{};
            }

            let mi_impls = self.generate_mi_impls(table, &secondary_fields);

            if !table.singleton {
                if self.has_primary_value_interface_trait(&item.ident.to_string()) {
                    primary_impl = None;
                }

                action_structs_code.push(quote_spanned!(span =>
                    #primary_impl
                    #secondary_impls
                    #mi_impls
                ));
            } else {
                let table_name = proc_macro2::Literal::string(&table.table_name.str());
                action_structs_code.push(quote_spanned!(span =>
                    impl ::eosio_chain::db::PrimaryValueInterface for #table_ident {
                        fn get_primary(&self) -> u64 {
                            return eosio_chain::name!(#table_name).value();
                        }
                    }

                    #secondary_impls
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
            .map(|(_, field)| {
                let secondary_type_name = Self::to_secondary_type(&field.ty);
                match secondary_type_name {
                    Some("Idx64") => {
                        return quote! {
                            eosio_chain::db::SecondaryType::Idx64
                        }
                    }
                    Some("Idx128") => {
                        return quote! {
                            eosio_chain::db::SecondaryType::Idx128
                        }
                    }
                    Some("Idx256") => {
                        return quote! {
                            eosio_chain::db::SecondaryType::Idx256
                        }
                    }
                    Some("IdxF64") => {
                        return quote! {
                            eosio_chain::db::SecondaryType::IdxF64
                        }
                    }
                    Some("IdxF128") => {
                        return quote! {
                            eosio_chain::db::SecondaryType::IdxF128
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
                .map(|(i, (_, field))| {
                    let idx_type: usize;
                    let secondary_type = Self::to_secondary_type(&field.ty);
                    match secondary_type {
                        Some("Idx64") => { idx_type = 0; }
                        Some("Idx128") => { idx_type = 1; }
                        Some("Idx256") => { idx_type = 2; }
                        Some("IdxF64") => { idx_type = 3; }
                        Some("IdxF128") => { idx_type = 4; }
                        _ => {
                            return quote!()
                        }
                    }

                    match secondary_type {
                        Some("Idx64") | Some("Idx128") | Some("Idx256") | Some("IdxF64") | Some("IdxF128") => {
                            let span = field.span();
                            let ty = &field.ty;
                            let method_name = String::from("get_idx_by_") + &field.ident.as_ref().unwrap().to_string();
                            let method_ident = syn::Ident::new(&method_name, span);
                            return quote_spanned!(span =>
                                #[allow(dead_code)]
                                fn #method_ident(&self) -> ::eosio_chain::db::IndexDBProxy<#ty, #idx_type> {
                                    return ::eosio_chain::db::IndexDBProxy::<#ty, #idx_type>::new(self.mi.idxdbs[#i].as_ref());
                                }
                            )
                        }
                        _ => return quote!(),
                    };
                });

            let mi_name = table_ident.to_string() + "MultiIndex";
            let mi_ident = syn::Ident::new(&mi_name, span);
            if table.singleton {
                return quote_spanned!(span =>
                    pub struct #mi_ident {
                        mi: ::eosio_chain::mi::MultiIndex<#table_ident>
                    }
                
                    #[allow(dead_code)]
                    impl #mi_ident {
                        ///
                        pub fn new(code: eosio_chain::Name, scope: eosio_chain::Name, table: eosio_chain::Name) -> Self {
                            Self {
                                mi: ::eosio_chain::mi::MultiIndex::<#table_ident>::new(code, scope, table, &[eosio_chain::db::SecondaryType::Idx64; 0]),
                            }
                        }

                        fn get(&self) -> Option<#table_ident> {
                            let it = self.mi.find(eosio_chain::Name::new(#table_name).value());
                            return self.mi.get(&it);
                        }

                        fn set(&self, value: &#table_ident, payer: eosio_chain::Name) {
                            let it = self.mi.find(eosio_chain::Name::new(#table_name).value());
                            if it.is_ok() {
                                self.mi.update(&it, value, payer);
                            } else {
                                self.mi.set(eosio_chain::Name::new(#table_name).value(), value, payer);
                            }
                        }
                    }

                    impl #table_ident {
                        #[allow(dead_code)]
                        fn new_mi(code: eosio_chain::Name, scope: eosio_chain::Name) -> Box<#mi_ident> {
                            return Box::new(#mi_ident::new(code, scope, eosio_chain::Name::new(#table_name)));
                        }
                    }
                );
            }
    
            return quote_spanned!(span =>

                pub struct #mi_ident {
                    mi: ::eosio_chain::mi::MultiIndex<#table_ident>
                }
            
                #[allow(dead_code)]
                impl #mi_ident {

                    pub fn new(code: eosio_chain::Name, scope: eosio_chain::Name, table: eosio_chain::Name, indexes: &[eosio_chain::db::SecondaryType]) -> Self {
                        Self {
                            mi: ::eosio_chain::mi::MultiIndex::<#table_ident>::new(code, scope, table, indexes),
                        }
                    }

                    pub fn store(&self, value: &#table_ident, payer: eosio_chain::Name) -> ::eosio_chain::db::Iterator<#table_ident> {
                        return self.mi.store(value, payer);
                    }
                
                    pub fn update(&self, iterator: &::eosio_chain::db::Iterator<#table_ident>, value: &#table_ident, payer: eosio_chain::Name) {
                        return self.mi.update(iterator, value, payer);
                    }
                
                    pub fn remove(&self, iterator: &::eosio_chain::db::Iterator<#table_ident>) {
                        return self.mi.remove(iterator);
                    }
                
                    pub fn get(&self, iterator: &::eosio_chain::db::Iterator<#table_ident>) -> Option<#table_ident> {
                        return self.mi.get(iterator)
                    }
                
                    pub fn get_by_primary(&self, primary: u64) -> Option<#table_ident> {
                        return self.mi.get_by_primary(primary);
                    }

                    pub fn next(&self, iterator: &::eosio_chain::db::Iterator<#table_ident>) -> ::eosio_chain::db::Iterator<#table_ident> {
                        return self.mi.db.next(iterator);
                    }

                    pub fn previous(&self, iterator: &::eosio_chain::db::Iterator<#table_ident>) -> ::eosio_chain::db::Iterator<#table_ident> {
                        return self.mi.db.previous(iterator);
                    }

                    pub fn find(&self, id: u64) -> ::eosio_chain::db::Iterator<#table_ident> {
                        return self.mi.db.find(id);
                    }
                
                    pub fn lowerbound(&self, id: u64) -> ::eosio_chain::db::Iterator<#table_ident> {
                        return self.mi.db.lowerbound(id);
                    }
                
                    pub fn upperbound(&self, id: u64) -> ::eosio_chain::db::Iterator<#table_ident> {
                        return self.mi.db.upperbound(id);
                    }
                
                    pub fn end(&self) -> ::eosio_chain::db::Iterator<#table_ident> {
                        return self.mi.db.end();
                    }
                
                    pub fn get_idx_db(&self, i: usize) -> &dyn ::eosio_chain::db::IndexDB {
                        return self.mi.idxdbs[i].as_ref();
                    }
                
                    pub fn idx_update(&self, it: ::eosio_chain::db::SecondaryIterator, value: ::eosio_chain::db::SecondaryValue, payer: eosio_chain::Name) {
                        self.mi.idx_update(it, value, payer);
                    }

                    #( #get_idx_db_funcs )*
                }

                impl #table_ident {
                    #[allow(dead_code)]
                    fn new_mi(code: eosio_chain::Name, scope: eosio_chain::Name) -> Box<#mi_ident> {
                        let indexes: [eosio_chain::db::SecondaryType; #len_secondary] = [#( #secondary_types ),*];
                        return Box::new(#mi_ident::new(code, scope, eosio_chain::Name::new(#table_name), &indexes));
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

    #[allow(dead_code)]
    fn is_option_type(ty: &syn::Type) -> bool {
        if let syn::Type::Path(type_path) = ty {
            if type_path.path.segments.len() != 1 {
                return false;
            }

            let path_seg = &type_path.path.segments[0];
            let name = path_seg.ident.to_string();
            if name == "Option" {
                return true;
            } else {
                return false;
            }
        }
        return false;
    }

    fn extract_type(ty: &syn::Type) -> Result<(String, &syn::Type), syn::Error> {
        if let syn::Type::Path(type_path) = ty {
            if type_path.path.segments.len() != 1 {
                return Err(format_err_spanned!(
                    ty,
                    "can not parse type with multiple segments",
                ))
            }

            let path_seg = &type_path.path.segments[0];
            let name = path_seg.ident.to_string();
            if name == "Option" || name == "Vec" || name == "BinaryExtension" {
                if let syn::PathArguments::AngleBracketed(x) = &path_seg.arguments {
                    if x.args.len() != 1 {
                        return Err(format_err_spanned!(
                            x,
                            "can not parse option type with multiple arguments",
                        ))      
                    }

                    let arg = &x.args[0];
                    if let syn::GenericArgument::Type(ty) = arg {
                        if let syn::Type::Path(type_path) = ty {
                            if type_path.path.segments.len() != 1 {
                                return Err(format_err_spanned!(
                                    type_path,
                                    "can not parse type in option with multiple segments",
                                ))
                            }
                            let name = type_path.path.segments.last().unwrap().ident.to_string();
                            return Ok((name, ty));
                        }
                    }
                }
            } else {
                return Ok((name, ty));
            }
        }

        Err(format_err_spanned!(
            ty,
            "unsupported type",
        ))
    }

    fn is_binary_extension_type(ty: &syn::Type) -> bool {
        if let syn::Type::Path(type_path) = ty {
            if type_path.path.segments.len() != 1 {
                false;
            }

            let path_seg = &type_path.path.segments[0];
            let name = path_seg.ident.to_string();
            if name == "BinaryExtension" {
                return true;
            } else {
                return false;
            }
        }
        return false;
    }

    fn is_secondary_type(ty: &syn::Type) -> bool {
        if let syn::Type::Path(type_path) = ty {
            if type_path.path.segments.len() != 1 {
                return false;
            }

            let path_seg = &type_path.path.segments[0];
            let name = path_seg.ident.to_string();
            if name == "u64" || name == "u128" || name == "Uint256" || name == "f64" || name == "Float128" {
                return true;
            } else {
                return false;
            }
        }
        return false;
    }

    fn to_secondary_type(ty: &syn::Type) -> Option<&'static str> {
        if let syn::Type::Path(type_path) = ty {
            if type_path.path.segments.len() != 1 {
                return None;
            }

            let path_seg = &type_path.path.segments[0];
            let name = path_seg.ident.to_string();
            if name == "u64" {
                return Some("Idx64");
            }  else if name == "u128" {
                return Some("Idx128");
            } else if name == "Uint256" {
                return Some("Idx256");
            } else if name == "f64" {
                return Some("IdxF64");
            } else if name == "Float128" {
                return Some("IdxF128");
            } else {
                return None;
            }
        }
        return None;
    }

    fn add_abi_type<'a>(&'a self, tp_name: &str, abi_types: &mut HashMap<String, &'a syn::Type>) -> Result<(), syn::Error> {
        if Self::is_primitive_type(tp_name) {
            return Ok(());
        }

        let mut ty_names: Vec<String> = Vec::new();
        for item in &self.items {
            match item {
                syn::Item::Struct(x) => {
                    if x.ident.to_string() != tp_name {
                        continue;
                    }
                    for field in &x.fields {
                        let (type_name, ty) = Self::extract_type(&field.ty)?;
                        if Self::is_primitive_type(&type_name) {
                            continue;
                        }
                        if abi_types.insert(type_name.clone(), ty).is_none() {
                            ty_names.push(type_name);
                        }
                    }
                    break;
                }
                syn::Item::Enum(x) => {
                    if x.ident.to_string() != tp_name {
                        continue;
                    }

                    for field in &x.variants {
                        // let field_ident = &field.ident;
                        if let syn::Fields::Unnamed(unnamed_fields) = &field.fields {
                            let unnamed_field = unnamed_fields.unnamed.last().unwrap();
                            let (type_name, _) = Self::extract_type(&unnamed_field.ty)?;
                            if Self::is_primitive_type(&type_name) {
                                continue;
                            }
                            if abi_types.insert(type_name.clone(), &unnamed_field.ty).is_none() {
                                ty_names.push(type_name);
                            }
                        }
                        //DODO: return error
                    };
                    break;
                }
                _ => {}
            }
        }

        for name in &ty_names {
            self.add_abi_type(name, abi_types)?;
        }
        Ok(())
    }

    fn gather_scale_info(&self) -> Result<TokenStream2, syn::Error> {
        let mut abi_types: HashMap<String, &syn::Type> = HashMap::new();

        for action in &self.actions {
            let item = &action.item;
            // let span = item.span();
            for arg in item.sig.inputs.iter() {
                if let syn::FnArg::Typed(pat_type) = arg {
                    let (type_name, ty) = Self::extract_type(&pat_type.ty)?;
                    if Self::is_primitive_type(&type_name) {
                        continue;
                    }
                    abi_types.insert(type_name.clone(), ty);
                    self.add_abi_type(&type_name, &mut abi_types)?;
                }
            }
        }

        for table in &self.tables {
            for field in &table.item.fields {
                let (type_name, tp) = Self::extract_type(&field.ty)?;
                if Self::is_primitive_type(&type_name) {
                    continue;
                }
                abi_types.insert(type_name.clone(), tp);
                self.add_abi_type(&type_name, &mut abi_types)?;
            }
        }

        let mut structs_code: Vec<TokenStream2> = Vec::new();
        for (_, tp) in abi_types {
            structs_code.push(
                quote!{
                    info.structs.push(#tp::type_info());
                }
            );
        }

        self.actions
            .iter()
            .for_each(|action| {
                let struct_name_ident = proc_macro2::Ident::new(&action.action_name.str(), proc_macro2::Span::call_site());
                structs_code.push(
                    quote!{
                        info.structs.push(#struct_name_ident::type_info());
                    }
                );
            });

        self.tables
            .iter()
            .for_each(|table| {
                let struct_name_ident = &table.item.ident;
                structs_code.push(
                    quote!{
                        info.structs.push(#struct_name_ident::type_info());
                    }
                );
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
                // let ident = &action.item.sig.ident;
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


        return Ok(quote!{
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
                    #( #structs_code ) *
                    #( #action_scale_info_code ) *
                    #( #table_scale_info_code ) *
                    return ::eosio_chain::abi::parse_abi_info(&mut info);
                }
            };
        });
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
                let _receiver = eosio_chain::Name{n: receiver};
                let _first_receiver = eosio_chain::Name{n: first_receiver};
                let _action = eosio_chain::Name{n: action};
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
                .map(|field| {
                    let field_ident = &field.ident;
                    // let index = syn::LitInt::new(&i.to_string(), proc_macro2::Span::call_site());
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
        let scale_info: Option<TokenStream2>;

        if self.has_main_struct() {
            scale_info = Some(self.gather_scale_info()?);
        } else {
            scale_info  = None;
        }

        let items = self.items.iter().map(|item|{
            match item {
                syn::Item::Struct(x) => {
                    if self.packers.iter().any(|packer|{
                        packer.ident == x.ident
                    }) {
                        quote!{
                            #[cfg_attr(feature = "std", derive(eosio_chain::eosio_scale_info::TypeInfo))]
                            #[cfg_attr(feature = "std", scale_info(crate = ::eosio_chain::eosio_scale_info))]
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
                            #[cfg_attr(feature = "std", derive(eosio_chain::eosio_scale_info::TypeInfo))]
                            #[cfg_attr(feature = "std", scale_info(crate = ::eosio_chain::eosio_scale_info))]
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
            #( #attrs )*
            #vis mod #ident {
                use eosio_chain::{
                    vec,
                    vec::Vec,
                    boxed::Box,
                    string::String,
                };

                use eosio_chain::{
                    serializer::Packer as _,
                    db::SecondaryType as _,
                    print::Printable as _,
                };

                #[cfg(feature = "std")]
                use eosio_chain::eosio_scale_info::TypeInfo as _;
    
                #[cfg(feature = "std")]
                use eosio_chain::eosio_scale_info;

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

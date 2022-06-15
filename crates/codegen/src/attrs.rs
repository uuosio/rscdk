#![allow(dead_code)]
use std::collections::HashMap;

use crate::{
    error::ExtError as _,
    FixedString,
    format_err,
    format_err_spanned,
};

use syn::spanned::Spanned;

use proc_macro2::{
    Group as Group2,
    Ident,
    Span,
    TokenStream as TokenStream2,
    TokenTree as TokenTree2,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Attribute {
    /// An chain specific attribute, e.g. `#[chain(action)]`.
    Chain(ChainAttribute),
    /// Any other attribute.
    ///
    /// This can be a known `#[derive(Debug)]` or a specific attribute of another
    /// crate.
    Other(syn::Attribute),
}

impl TryFrom<syn::Attribute> for Attribute {
    type Error = syn::Error;

    fn try_from(attr: syn::Attribute) -> Result<Self, Self::Error> {
        if attr.path.is_ident("chain") {
            return <ChainAttribute as TryFrom<_>>::try_from(attr).map(Into::into)
        }
        Ok(Attribute::Other(attr))
    }
}

impl From<ChainAttribute> for Attribute {
    fn from(attr: ChainAttribute) -> Self {
        Attribute::Chain(attr)
    }
}

impl Attribute {
    /// Returns `Ok` if the given iterator yields no duplicate chain attributes.
    ///
    /// # Errors
    ///
    /// If the given iterator yields duplicate chain attributes.
    /// Note: Duplicate non-chain attributes are fine.
    fn ensure_no_duplicate_attrs<'a, I>(attrs: I) -> Result<(), syn::Error>
    where
        I: IntoIterator<Item = &'a ChainAttribute>,
    {
        use std::collections::HashSet;
        let mut seen: HashSet<&ChainAttribute> = HashSet::new();
        for attr in attrs.into_iter() {
            if let Some(seen) = seen.get(attr) {
                use crate::error::ExtError as _;
                return Err(format_err!(
                    attr.span(),
                    "encountered duplicate chain attribute"
                )
                .into_combine(format_err!(seen.span(), "first chain attribute here")))
            }
            seen.insert(attr);
        }
        Ok(())
    }
}

/// The kind of an chain attribute argument.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AttributeArgKind {
    MainStruct,
    Action,
    Notify,
    Packer,
    Variant,
    Table,
    Singleton,
    Primary,
    Secondary,
    Idx64,
    Idx128,
    Idx256,
    IdxF64,
    IdxF128,
}

impl core::fmt::Display for AttributeArgKind {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            Self::MainStruct => write!(f, "main"),
            Self::Table => write!(f, "table"),
            Self::Singleton => write!(f, "singleton"),
            Self::Packer => write!(f, "packer"),
            Self::Variant => write!(f, "variant"),
            Self::Primary => write!(f, "primary"),
            Self::Secondary => write!(f, "secondary"),
            Self::Action => write!(f, "action"),
            Self::Notify => write!(f, "notify"),
            Self::Idx64 => write!(f, "Idx64=N:string"),
            Self::Idx128 => write!(f, "Idx128"),
            Self::Idx256 => write!(f, "Idx256"),
            Self::IdxF64 => write!(f, "IdxF64"),
            Self::IdxF128 => write!(f, "IdxF128"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AttributeArg {
    MainStruct,
    Action(FixedString),
    Notify,
    Packer,
    Variant,
    Table(FixedString),
    Singleton,
    Primary,
    Secondary,
    Idx64(FixedString),
    Idx128(FixedString),
    Idx256(FixedString),
    IdxF64(FixedString),
    IdxF128(FixedString),
}

impl AttributeArg {
    /// Returns the kind of the chain attribute argument.
    pub fn kind(&self) -> AttributeArgKind {
        match self {
            Self::MainStruct => AttributeArgKind::MainStruct,
            Self::Action(_) => AttributeArgKind::Action,
            Self::Notify => AttributeArgKind::Notify,
            Self::Packer => AttributeArgKind::Packer,
            Self::Variant => AttributeArgKind::Variant,
            Self::Table(_) => AttributeArgKind::Table,
            Self::Singleton => AttributeArgKind::Singleton,
            Self::Primary => AttributeArgKind::Primary,
            Self::Secondary => AttributeArgKind::Secondary,
            Self::Idx64(_) => AttributeArgKind::Idx64,
            Self::Idx128(_) => AttributeArgKind::Idx128,
            Self::Idx256(_) => AttributeArgKind::Idx256,
            Self::IdxF64(_) => AttributeArgKind::IdxF64,
            Self::IdxF128(_) => AttributeArgKind::IdxF128,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeFrag {
    pub ast: syn::Meta,
    pub arg: AttributeArg,
}


impl AttributeFrag {
    /// Returns a shared reference to the attribute argument kind.
    pub fn kind(&self) -> &AttributeArg {
        &self.arg
    }
}

impl Spanned for AttributeFrag {
    fn span(&self) -> Span {
        self.ast.span()
    }
}

impl TryFrom<syn::NestedMeta> for AttributeFrag {
    type Error = syn::Error;

    fn try_from(nested_meta: syn::NestedMeta) -> Result<Self, Self::Error> {
        match nested_meta {
            syn::NestedMeta::Meta(meta) => {
                match &meta {
                    syn::Meta::NameValue(name_value) => {
                        if name_value.path.is_ident("Idx64") {
                            if let syn::Lit::Str(lit_str) = &name_value.lit {
                                let value = lit_str.value();
                                return Ok(AttributeFrag {
                                    ast: meta,
                                    arg: AttributeArg::Idx64(FixedString::new(&value)),
                                })
                            }
                            return Err(format_err!(name_value, "expected `str` value type for `flag` in #[chain(Idx64)]"))
                        }

                        if name_value.path.is_ident("Idx128") {
                            if let syn::Lit::Str(lit_str) = &name_value.lit {
                                let value = lit_str.value();
                                return Ok(AttributeFrag {
                                    ast: meta,
                                    arg: AttributeArg::Idx128(FixedString::new(&value)),
                                })
                            }
                            return Err(format_err!(name_value, "expected `str` value type for `flag` in #[chain(Idx128)]"))
                        }

                        if name_value.path.is_ident("Idx256") {
                            if let syn::Lit::Str(lit_str) = &name_value.lit {
                                let value = lit_str.value();
                                return Ok(AttributeFrag {
                                    ast: meta,
                                    arg: AttributeArg::Idx256(FixedString::new(&value)),
                                })
                            }
                            return Err(format_err!(name_value, "expected `str` value type for `flag` in #[chain(Idx256)]"))
                        }

                        if name_value.path.is_ident("IdxF64") {
                            if let syn::Lit::Str(lit_str) = &name_value.lit {
                                let value = lit_str.value();
                                return Ok(AttributeFrag {
                                    ast: meta,
                                    arg: AttributeArg::IdxF64(FixedString::new(&value)),
                                })
                            }
                            return Err(format_err!(name_value, "expected `str` value type for `flag` in #[chain(IdxF64)]"))
                        }

                        if name_value.path.is_ident("IdxF128") {
                            if let syn::Lit::Str(lit_str) = &name_value.lit {
                                let value = lit_str.value();
                                return Ok(AttributeFrag {
                                    ast: meta,
                                    arg: AttributeArg::IdxF128(FixedString::new(&value)),
                                })
                            }
                            return Err(format_err!(name_value, "expected `str` value type for `flag` in #[chain(IdxF128)]"))
                        }

                        if name_value.path.is_ident("action") {
                            if let syn::Lit::Str(lit_str) = &name_value.lit {
                                let value = lit_str.value();
                                return Ok(AttributeFrag {
                                    ast: meta,
                                    arg: AttributeArg::Action(FixedString::new(&value)),
                                })
                            }
                            return Err(format_err!(name_value, "expected `str` value type for `flag` in #[chain(action = name)]"))
                        }

                        if name_value.path.is_ident("table") {
                            if let syn::Lit::Str(lit_str) = &name_value.lit {
                                let value = lit_str.value();
                                return Ok(AttributeFrag {
                                    ast: meta,
                                    arg: AttributeArg::Table(FixedString::new(&value)),
                                })
                            }
                            return Err(format_err!(name_value, "expected `str` value type for `flag` in #[chain(table = name)]"))
                        }
                        Err(format_err_spanned!(
                            meta,
                            "unknown chain attribute argument (name = value)",
                        ))
                    }
                    syn::Meta::Path(path) => {
                        path
                            .get_ident()
                            .map(Ident::to_string)
                            .ok_or_else(|| format_err_spanned!(meta, "unknown chain attribute (path)"))
                            .and_then(|ident| match ident.as_str() {
                                "main" => Ok(AttributeArg::MainStruct),
                                "singleton" => Ok(AttributeArg::Singleton),
                                "packer" => Ok(AttributeArg::Packer),
                                "variant" => Ok(AttributeArg::Variant),
                                "primary" => Ok(AttributeArg::Primary),
                                "secondary" => Ok(AttributeArg::Secondary),
                                "notify" => Ok(AttributeArg::Notify),
                                "Idx64" => Ok(AttributeArg::Idx64(FixedString::new(""))),
                                "Idx128" => Ok(AttributeArg::Idx128(FixedString::new(""))),
                                "Idx256" => Ok(AttributeArg::Idx256(FixedString::new(""))),
                                "IdxF64" => Ok(AttributeArg::IdxF64(FixedString::new(""))),
                                "IdxF128" => Ok(AttributeArg::IdxF128(FixedString::new(""))),
                                _ => Err(format_err_spanned!(
                                    meta, "unknown chain attribute (path)"
                                ))
                            })
                            .map(|kind| AttributeFrag { ast: meta, arg: kind, })
                    }
                    syn::Meta::List(_) => {
                        Err(format_err_spanned!(
                            meta,
                            "unknown chain attribute argument (list)"
                        ))
                    }
                }
            }
            syn::NestedMeta::Lit(_) => {
                Err(format_err_spanned!(
                    nested_meta,
                    "unknown chain attribute argument (literal)"
                ))
            }
        }
    }
}

/// Types implementing this trait can return a slice over their `syn` attributes.
pub trait Attrs {
    /// Returns the slice of attributes of an AST entity.
    fn attrs(&self) -> &[syn::Attribute];
}

impl Attrs for syn::ImplItem {
    fn attrs(&self) -> &[syn::Attribute] {
        match self {
            syn::ImplItem::Const(item) => &item.attrs,
            syn::ImplItem::Method(item) => &item.attrs,
            syn::ImplItem::Type(item) => &item.attrs,
            syn::ImplItem::Macro(item) => &item.attrs,
            _ => &[],
        }
    }
}

impl Attrs for syn::Item {
    fn attrs(&self) -> &[syn::Attribute] {
        use syn::Item;
        match self {
            Item::Const(syn::ItemConst { attrs, .. })
            | Item::Enum(syn::ItemEnum { attrs, .. })
            | Item::ExternCrate(syn::ItemExternCrate { attrs, .. })
            | Item::Fn(syn::ItemFn { attrs, .. })
            | Item::ForeignMod(syn::ItemForeignMod { attrs, .. })
            | Item::Impl(syn::ItemImpl { attrs, .. })
            | Item::Macro(syn::ItemMacro { attrs, .. })
            | Item::Macro2(syn::ItemMacro2 { attrs, .. })
            | Item::Mod(syn::ItemMod { attrs, .. })
            | Item::Static(syn::ItemStatic { attrs, .. })
            | Item::Struct(syn::ItemStruct { attrs, .. })
            | Item::Trait(syn::ItemTrait { attrs, .. })
            | Item::TraitAlias(syn::ItemTraitAlias { attrs, .. })
            | Item::Type(syn::ItemType { attrs, .. })
            | Item::Union(syn::ItemUnion { attrs, .. })
            | Item::Use(syn::ItemUse { attrs, .. }) => attrs,
            _ => &[],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChainAttribute {
    /// The internal non-empty sequence of arguments of the chain attribute.
    args: Vec<AttributeFrag>,
}

impl Spanned for ChainAttribute {
    fn span(&self) -> Span {
        self.args
            .iter()
            .map(|arg| arg.span())
            .fold(self.first().span(), |fst, snd| {
                fst.join(snd).unwrap_or_else(|| self.first().span())
            })
    }
}

impl TryFrom<syn::Attribute> for ChainAttribute {
    type Error = syn::Error;

    fn try_from(mut attr: syn::Attribute) -> Result<Self, Self::Error> {
        if !attr.path.is_ident("chain") {
            return Err(format_err_spanned!(attr, "unexpected non-chain attribute"))
        }

        let ts: TokenStream2 = attr
            .tokens
            .into_iter()
            .map(|tt| {
                match tt {
                    TokenTree2::Group(grp) => transform_wildcard_selector_to_string(grp),
                    _ => tt,
                }
            })
            .collect();
        attr.tokens = ts;

        match attr.parse_meta().map_err(|_| {
            format_err_spanned!(attr, "unexpected chain attribute structure")
        })? {
            syn::Meta::List(meta_list) => {
                let args = meta_list
                    .nested
                    .into_iter()
                    .map(<AttributeFrag as TryFrom<_>>::try_from)
                    .collect::<Result<Vec<_>, syn::Error>>()?;
                Self::ensure_no_duplicate_args(&args)?;
                if args.is_empty() {
                    return Err(format_err_spanned!(
                        attr,
                        "encountered unsupported empty chain attribute"
                    ))
                }
                Ok(ChainAttribute { args })
            }
            _ => Err(format_err_spanned!(attr, "unknown chain attribute")),
        }
    }
}

impl ChainAttribute {
    /// Returns the first chain attribute argument.
    pub fn first(&self) -> &AttributeFrag {
        self.args
            .first()
            .expect("encountered invalid empty chain attribute list")
    }

    /// Ensure that the first chain attribute argument is of expected kind.
    ///
    /// # Errors
    ///
    /// If the first chain attribute argument is not of expected kind.
    pub fn ensure_first(&self, expected: &AttributeArgKind) -> Result<(), syn::Error> {
        if &self.first().arg.kind() != expected {
            return Err(format_err!(
                self.span(),
                "unexpected first chain attribute argument",
            ))
        }
        Ok(())
    }

    /// Ensures that the given iterator of chain attribute arguments do not have
    /// duplicates.
    ///
    /// # Errors
    ///
    /// If the given iterator yields duplicate chain attribute arguments.
    fn ensure_no_duplicate_args<'a, A>(args: A) -> Result<(), syn::Error>
    where
        A: IntoIterator<Item = &'a AttributeFrag>,
    {
        use crate::error::ExtError as _;
        use std::collections::HashSet;
        let mut seen: HashSet<&AttributeFrag> = HashSet::new();
        let mut seen2: HashMap<AttributeArgKind, Span> = HashMap::new();
        for arg in args.into_iter() {
            if let Some(seen) = seen.get(arg) {
                return Err(format_err!(
                    arg.span(),
                    "encountered duplicate chain attribute arguments"
                )
                .into_combine(format_err!(
                    seen.span(),
                    "first equal chain attribute argument here"
                )))
            }
            if let Some(seen) = seen2.get(&arg.kind().kind()) {
                return Err(format_err!(
                    arg.span(),
                    "encountered chain attribute arguments with equal kinds"
                )
                .into_combine(format_err!(
                    *seen,
                    "first equal chain attribute argument with equal kind here"
                )))
            }
            seen.insert(arg);
            seen2.insert(arg.kind().kind(), arg.span());
        }
        Ok(())
    }

    pub fn ensure_no_conflicts<'a, P>(
        &'a self,
        mut is_conflicting: P,
    ) -> Result<(), syn::Error>
    where
        P: FnMut(&'a AttributeFrag) -> Result<(), Option<syn::Error>>,
    {
        let mut err: Option<syn::Error> = None;
        for arg in self.args() {
            if let Err(reason) = is_conflicting(arg) {
                let conflict_err = format_err!(
                    arg.span(),
                    "encountered conflicting chain attribute argument!!",
                );
                match &mut err {
                    Some(err) => {
                        err.combine(conflict_err);
                    }
                    None => {
                        err = Some(conflict_err);
                    }
                }
                if let Some(reason) = reason {
                    err.as_mut()
                        .expect("must be `Some` at this point")
                        .combine(reason);
                }
            }
        }
        if let Some(err) = err {
            return Err(err)
        }
        Ok(())
    }

    pub fn from_expanded<A>(attrs: A) -> Result<Self, syn::Error>
    where
        A: IntoIterator<Item = Self>,
    {
        let args = attrs
            .into_iter()
            .flat_map(|attr| attr.args)
            .collect::<Vec<_>>();
        if args.is_empty() {
            return Err(format_err!(
                Span::call_site(),
                "encountered unexpected empty expanded chain attribute arguments",
            ))
        }
        Self::ensure_no_duplicate_args(&args)?;
        Ok(Self { args })
    }

    pub fn args(&self) -> ::core::slice::Iter<AttributeFrag> {
        self.args.iter()
    }

    pub fn table_name(&self) -> Option<FixedString> {
        if self.args().len() == 0 {
            return None;
        }
        if let AttributeArg::Table(x) = self.args[0].kind() {
            return Some(x.clone());
        }
        return None;
    }

    pub fn is_singleton(&self) -> bool {
        if self.args().len() < 2 {
            return false;
        }

        if let AttributeArg::Table(_) = self.args[0].kind() {
            if let AttributeArg::Singleton = self.args[1].kind() {
                return true;
            }    
        }

        return true;
    }

    pub fn action_name(&self) -> Option<FixedString> {
        self.args().find_map(|arg| {
            if let AttributeArg::Action(name) = arg.kind() {
                return Some(*name)
            }
            None
        })
    }

    pub fn is_notify(&self) -> bool {
        self.args
            .iter()
            .any(|arg| matches!(arg.kind(), AttributeArg::Notify))
    }

}

/// Returns `true` if the given iterator yields at least one attribute of the form
/// `#[chain(...)]` or `#[chain]`.
///
/// # Note
///
/// This does not check at this point whether the attribute is valid since
/// this check is optimized for efficiency.
pub fn contains_chain_attributes<'a, I>(attrs: I) -> bool
where
    I: IntoIterator<Item = &'a syn::Attribute>,
{
    attrs.into_iter().any(|attr| attr.path.is_ident("chain"))
}


/// Returns the first valid attribute, if any.
///
/// Returns `None` if there are no attributes.
///
/// # Errors
///
/// Returns an error if the first attribute is invalid.
pub fn first_chain_attribute<'a, I>(
    attrs: I,
) -> Result<Option<ChainAttribute>, syn::Error>
where
    I: IntoIterator<Item = &'a syn::Attribute>,
{
    let first = attrs.into_iter().find(|attr| {
        attr.path.is_ident("chain")
    });
    match first {
        None => Ok(None),
        Some(chain_attr) => ChainAttribute::try_from(chain_attr.clone()).map(Some),
    }
}

/// This function replaces occurrences of a `TokenTree::Ident` of the sequence
/// `selector = _` with the sequence `selector = "_"`.
///
/// This is done because `syn::Attribute::parse_meta` does not support parsing a
/// verbatim like `_`. For this we would need to switch to `syn::Attribute::parse_args`,
/// which requires a more in-depth rewrite of our IR parsing.
fn transform_wildcard_selector_to_string(group: Group2) -> TokenTree2 {
    let mut found_selector = false;
    let mut found_equal = false;

    let new_group: TokenStream2 = group
        .stream()
        .into_iter()
        .map(|tt| {
            match tt {
                TokenTree2::Group(grp) => transform_wildcard_selector_to_string(grp),
                TokenTree2::Ident(ident)
                    if found_selector && found_equal && ident == "_" =>
                {
                    let mut lit = proc_macro2::Literal::string("_");
                    lit.set_span(ident.span());
                    found_selector = false;
                    found_equal = false;
                    TokenTree2::Literal(lit)
                }
                TokenTree2::Ident(ident) if ident == "selector" => {
                    found_selector = true;
                    TokenTree2::Ident(ident)
                }
                TokenTree2::Punct(punct) if punct.as_char() == '=' => {
                    found_equal = true;
                    TokenTree2::Punct(punct)
                }
                _ => tt,
            }
        })
        .collect();
    TokenTree2::Group(Group2::new(group.delimiter(), new_group))
}

/// Partitions the given attributes into chain specific and non-chain specific attributes.
///
/// # Error
///
/// Returns an error if some chain specific attributes could not be successfully parsed.
pub fn partition_attributes<I>(
    attrs: I,
) -> Result<(Vec<ChainAttribute>, Vec<syn::Attribute>), syn::Error>
where
    I: IntoIterator<Item = syn::Attribute>,
{
    use either::Either;
    use itertools::Itertools as _;
    let (chain_attrs, others) = attrs
        .into_iter()
        .map(<Attribute as TryFrom<_>>::try_from)
        .collect::<Result<Vec<Attribute>, syn::Error>>()?
        .into_iter()
        .partition_map(|attr| {
            match attr {
                Attribute::Chain(chain_attr) => Either::Left(chain_attr),
                Attribute::Other(other_attr) => Either::Right(other_attr),
            }
        });
    Attribute::ensure_no_duplicate_attrs(&chain_attrs)?;
    Ok((chain_attrs, others))
}

pub fn sanitize_attributes<I, C>(
    parent_span: Span,
    attrs: I,
    is_valid_first: &AttributeArgKind,
    is_conflicting_attr: C,
) -> Result<(ChainAttribute, Vec<syn::Attribute>), syn::Error>
where
    I: IntoIterator<Item = syn::Attribute>,
    C: FnMut(&AttributeFrag) -> Result<(), Option<syn::Error>>,
{
    let (chain_attrs, other_attrs) = partition_attributes(attrs)?;
    let normalized = ChainAttribute::from_expanded(chain_attrs).map_err(|err| {
        err.into_combine(format_err!(parent_span, "at this invocation",))
    })?;
    normalized.ensure_first(is_valid_first).map_err(|err| {
        err.into_combine(format_err!(
            parent_span,
            "expected {} as first chain attribute argument",
            is_valid_first,
        ))
    })?;
    normalized.ensure_no_conflicts(is_conflicting_attr)?;
    Ok((normalized, other_attrs))
}

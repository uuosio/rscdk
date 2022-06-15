use crate::FixedString;

///
#[derive(Debug, PartialEq, Eq)]
pub struct Table {
    /// The underlying Rust method item.
    pub item: syn::ItemStruct,
    pub table_name: FixedString,
    pub singleton: bool,
}

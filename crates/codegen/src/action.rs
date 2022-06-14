use crate::FixedString;

///
#[derive(Debug, PartialEq, Eq)]
pub struct Action {
    /// The underlying Rust method item.
    pub item: syn::ImplItemMethod,
    pub is_notify: bool,
    pub action_name: FixedString,
}

#![doc = include_str!("../doc/lib.md")]
#![deny(missing_docs)]

use proc_macro::TokenStream;

pub(crate) mod func;
pub(crate) mod invoke;
pub(crate) mod select;

pub(crate) use proc_macro2::TokenStream as TokenStream2;
pub(crate) use syn::Result as SynResult;

#[doc = include_str!("../doc/func.md")]
#[proc_macro_attribute]
pub fn func(attr: TokenStream, item: TokenStream) -> TokenStream {
    func::func(attr, item)
}

#[doc = include_str!("../doc/invoke.md")]
#[proc_macro]
pub fn invoke(input: TokenStream) -> TokenStream {
    invoke::invoke(input)
}

#[doc = include_str!("../doc/select.md")]
#[proc_macro]
pub fn select(input: TokenStream) -> TokenStream {
    select::select(input)
}

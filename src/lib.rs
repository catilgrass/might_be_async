#![doc = include_str!("../doc/lib.md")]
#![deny(missing_docs)]

use proc_macro::TokenStream;

pub(crate) mod config;
pub(crate) mod func;
pub(crate) mod invoke;
pub(crate) mod select;

pub(crate) use proc_macro2::TokenStream as TokenStream2;
pub(crate) use syn::Result as SynResult;

#[doc = include_str!("../doc/func.md")]
///
/// # How to use?
///
/// ```
/// # use might_be_async::*;
#[doc = include_str!("../doc/usage/func.rs")]
/// ```
///
/// # Expanded
///
/// The above code will be expanded into the following:
///
/// Sync:
///
/// ```
/// # use might_be_async::*;
#[doc = include_str!("../doc/usage/func_expand.rs")]
/// ```
///
/// Async:
///
/// ```
/// # use might_be_async::*;
#[doc = include_str!("../doc/usage/func_async_expand.rs")]
/// ```
#[proc_macro_attribute]
pub fn func(attr: TokenStream, item: TokenStream) -> TokenStream {
    func::func(attr, item)
}

#[doc = include_str!("../doc/invoke.md")]
///
/// # How to use?
///
/// ```
/// # use might_be_async::*;
#[doc = include_str!("../doc/usage/invoke.rs")]
/// ```
///
/// # Expanded
///
/// The above code will be expanded into the following:
///
/// Sync:
///
/// ```
/// # use might_be_async::*;
#[doc = include_str!("../doc/usage/invoke_expand.rs")]
/// ```
///
/// Async:
///
/// ```
/// # use might_be_async::*;
#[doc = include_str!("../doc/usage/invoke_async_expand.rs")]
/// ```
#[proc_macro]
pub fn invoke(input: TokenStream) -> TokenStream {
    invoke::invoke(input)
}

#[doc = include_str!("../doc/select.md")]
///
/// # How to use?
///
/// ```
/// # use might_be_async::*;
#[doc = include_str!("../doc/usage/select.rs")]
/// ```
///
/// # Expanded
///
/// The above code will be expanded into the following:
///
/// Sync:
///
/// ```
/// # use might_be_async::*;
#[doc = include_str!("../doc/usage/select_expand.rs")]
/// ```
///
/// Async:
///
/// ```
/// # use might_be_async::*;
#[doc = include_str!("../doc/usage/select_async_expand.rs")]
/// ```
#[proc_macro]
pub fn select(input: TokenStream) -> TokenStream {
    select::select(input)
}

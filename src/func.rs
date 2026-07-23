use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

use crate::args::FuncArgs;

/// Attribute macro that generates both a sync and an async version of a function,
/// gated by a Cargo feature flag.
///
/// # Usage
///
/// ```ignore
/// /// Doc comments are preserved
/// #[might_be_async::func]
/// pub fn my_function<T: Clone>(arg: T) -> ReturnType
/// where T: Debug
/// {
///     // body — written as a regular (non-async) function
/// }
/// ```
///
/// Expands to:
/// - `#[cfg(not(feature = "async"))] fn my_function(...)` — sync version
/// - `#[cfg(feature = "async")] async fn my_function(...)` — async version
///
/// An explicit feature name can be provided:
///
/// ```ignore
/// #[might_be_async::func("tokio_rt")]
/// pub fn my_function() { ... }
/// ```
pub fn func(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as FuncArgs);
    let feature_name = &args.feature_name;

    let input_fn = parse_macro_input!(item as ItemFn);
    let attrs = &input_fn.attrs;
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;

    let expanded = quote! {
        #(#attrs)*
        #[cfg(not(feature = #feature_name))]
        #vis #sig #block

        #(#attrs)*
        #[cfg(feature = #feature_name)]
        #vis async #sig #block
    };

    TokenStream::from(expanded)
}

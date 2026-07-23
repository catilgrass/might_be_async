use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

use crate::args::FuncArgs;

pub(crate) fn func(attr: TokenStream, item: TokenStream) -> TokenStream {
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

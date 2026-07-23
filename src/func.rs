use crate::SynResult;
use crate::config::default_feature_name;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ItemFn, LitStr,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[doc = include_str!("../doc/args/func.md")]
pub struct FuncArgs {
    pub feature_name: String,
}

impl Default for FuncArgs {
    fn default() -> Self {
        FuncArgs {
            feature_name: default_feature_name().to_string(),
        }
    }
}

impl Parse for FuncArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.is_empty() {
            return Ok(FuncArgs::default());
        }

        // Parse: "feature_name"
        let feat_lit: LitStr = input.parse()?;
        let feature_name = feat_lit.value();

        Ok(FuncArgs { feature_name })
    }
}

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

/// Test module for `FuncArgs` parsing.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_defaults_to_async() {
        // Input: empty → defaults to "foo_async"
        let input = proc_macro2::TokenStream::new();
        let args: FuncArgs = syn::parse2(input).unwrap();
        assert_eq!(args.feature_name, "foo_async");
    }

    #[test]
    fn custom_feature_name_parsed() {
        // Input: "custom_name"
        let input: proc_macro2::TokenStream = "\"custom_name\"".parse().unwrap();
        let args: FuncArgs = syn::parse2(input).unwrap();
        assert_eq!(args.feature_name, "custom_name");
    }
}

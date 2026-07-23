use crate::SynResult;
use crate::TokenStream2;
use crate::config::default_feature_name;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[doc = include_str!("../doc/args/invoke.md")]
pub enum InvokeArgs {
    /// invoke!(expr) — feature name defaults to "async"
    Default(TokenStream2),

    /// invoke!("feat" => expr) — explicit feature name
    Explicit(LitStr, TokenStream2),
}

impl Parse for InvokeArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(LitStr) {
            let feat: LitStr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let expr: TokenStream2 = input.parse()?;
            Ok(InvokeArgs::Explicit(feat, expr))
        } else {
            let expr: TokenStream2 = input.parse()?;
            Ok(InvokeArgs::Default(expr))
        }
    }
}

pub(crate) fn invoke(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InvokeArgs);

    let expanded = match input {
        InvokeArgs::Default(expr) => {
            let feat = default_feature_name();
            quote! {{
                #[cfg(feature = #feat)]
                { #expr.await }
                #[cfg(not(feature = #feat))]
                { #expr }
            }}
        }
        InvokeArgs::Explicit(feat, expr) => {
            let feat_name = &feat;
            quote! {{
                #[cfg(feature = #feat_name)]
                { #expr.await }
                #[cfg(not(feature = #feat_name))]
                { #expr }
            }}
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_variant() {
        // Input: an expression without feature name → Default variant
        let input: proc_macro2::TokenStream = "compute(5)".parse().unwrap();
        let args: InvokeArgs = syn::parse2(input).unwrap();
        match args {
            InvokeArgs::Default(_) => {} // expected
            _ => panic!("expected Default variant"),
        }
    }

    #[test]
    fn explicit_variant() {
        // Input: "my_ft" => expr → Explicit variant with feature "my_ft"
        let input: proc_macro2::TokenStream = "\"my_ft\" => compute(5)".parse().unwrap();
        let args: InvokeArgs = syn::parse2(input).unwrap();
        match args {
            InvokeArgs::Explicit(feat, _) => {
                assert_eq!(feat.value(), "my_ft");
            }
            _ => panic!("expected Explicit variant"),
        }
    }
}

use crate::SynResult;
use crate::TokenStream2;
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
            quote! {{
                #[cfg(feature = "async")]
                { #expr.await }
                #[cfg(not(feature = "async"))]
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

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::args::InvokeInput;

/// Wraps a call expression, adding `.await` when the async feature is enabled.
///
/// # Usage
///
/// ```ignore
/// // Default feature name ("async"):
/// let result = might_be_async::invoke!(some_async_fn(args));
///
/// // Explicit feature name:
/// let result = might_be_async::invoke!("tokio_rt" => some_async_fn(args));
/// ```
pub fn invoke(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InvokeInput);

    let expanded = match input {
        InvokeInput::Default(expr) => {
            quote! {{
                #[cfg(feature = "async")]
                { #expr.await }
                #[cfg(not(feature = "async"))]
                { #expr }
            }}
        }
        InvokeInput::Explicit(feat, expr) => {
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

use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Token};

/// Arguments for the `#[func]` attribute.
pub struct FuncArgs {
    pub feature_name: String,
}

impl Default for FuncArgs {
    fn default() -> Self {
        FuncArgs {
            feature_name: "async".to_string(),
        }
    }
}

impl Parse for FuncArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(FuncArgs::default());
        }

        // Parse: "feature_name"
        let feat_lit: LitStr = input.parse()?;
        let feature_name = feat_lit.value();

        Ok(FuncArgs { feature_name })
    }
}

/// Input for the `invoke!` macro.
pub enum InvokeInput {
    /// invoke!(expr) — feature name defaults to "async"
    Default(proc_macro2::TokenStream),
    /// invoke!("feat" => expr) — explicit feature name
    Explicit(LitStr, proc_macro2::TokenStream),
}

impl Parse for InvokeInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            let feat: LitStr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let expr: proc_macro2::TokenStream = input.parse()?;
            Ok(InvokeInput::Explicit(feat, expr))
        } else {
            let expr: proc_macro2::TokenStream = input.parse()?;
            Ok(InvokeInput::Default(expr))
        }
    }
}

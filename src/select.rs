use crate::SynResult;
use crate::TokenStream2;
use proc_macro::TokenStream;
use proc_macro2::{Spacing, TokenTree};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, LitStr, Token, parse_macro_input};

#[doc = include_str!("../doc/args/select_arm.md")]
pub enum SelectArmArgs {
    /// "feat_name" => expr
    Explicit { feat: LitStr, body: Expr },

    /// ! => expr
    Not { body: Expr },

    /// expr (no feature name — auto-detect by .await)
    Implicit { body: Expr },
}

impl Parse for SelectArmArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        // Try to parse an explicit feature name: "feature_name" => expr
        if input.peek(LitStr) {
            let feat: LitStr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let body: Expr = input.parse()?;
            Ok(SelectArmArgs::Explicit { feat, body })
        }
        // Try to parse a negation arm: ! => expr
        else if input.peek(Token![!]) {
            let _not: Token![!] = input.parse()?;
            input.parse::<Token![=>]>()?;
            let body: Expr = input.parse()?;
            Ok(SelectArmArgs::Not { body })
        }
        // Otherwise treat as an implicit arm (auto-detect whether it contains .await)
        else {
            let body: Expr = input.parse()?;
            Ok(SelectArmArgs::Implicit { body })
        }
    }
}

struct SelectInput {
    arms: Vec<SelectArmArgs>,
}

impl Parse for SelectInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let punctuated: Punctuated<SelectArmArgs, Token![,]> = Punctuated::parse_terminated(input)?;

        if punctuated.len() != 2 {
            return Err(syn::Error::new(
                input.span(),
                "select! requires exactly 2 arms",
            ));
        }

        let arms: Vec<SelectArmArgs> = punctuated.into_iter().collect();
        Ok(SelectInput { arms })
    }
}

pub(crate) fn select(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SelectInput);
    let expanded = input.expand();
    TokenStream::from(expanded)
}

impl SelectInput {
    fn expand(&self) -> TokenStream2 {
        let arm0 = &self.arms[0];
        let arm1 = &self.arms[1];

        match (arm0, arm1) {
            // Both explicit — use cfg!() since no .await to worry about
            (
                SelectArmArgs::Explicit { feat: f0, body: b0 },
                SelectArmArgs::Explicit { feat: f1, body: b1 },
            ) => {
                let f0_str = f0.value();
                let f1_str = f1.value();

                if has_not_prefix(&f0_str) && has_not_prefix(&f1_str) {
                    quote! { if cfg!(feature = "async") { #b0 } else { #b1 } }
                } else if has_not_prefix(&f0_str) {
                    let inner = &f0_str[1..];
                    quote! { if cfg!(feature = #inner) { #b1 } else { #b0 } }
                } else if has_not_prefix(&f1_str) {
                    quote! { if cfg!(feature = #f0_str) { #b0 } else { #b1 } }
                } else {
                    quote! { if cfg!(feature = #f0_str) { #b0 } else { #b1 } }
                }
            }

            // Explicit + Not — cfg!() safe (no .await)
            (SelectArmArgs::Explicit { feat, body }, SelectArmArgs::Not { body: not_body }) => {
                let feat_str = feat.value();
                if has_not_prefix(&feat_str) {
                    let inner = &feat_str[1..];
                    quote! { if cfg!(feature = #inner) { #not_body } else { #body } }
                } else {
                    quote! { if cfg!(feature = #feat_str) { #body } else { #not_body } }
                }
            }

            // Not + Explicit — cfg!() safe (no .await)
            (SelectArmArgs::Not { body: not_body }, SelectArmArgs::Explicit { feat, body }) => {
                let feat_str = feat.value();
                if has_not_prefix(&feat_str) {
                    let inner = &feat_str[1..];
                    quote! { if cfg!(feature = #inner) { #body } else { #not_body } }
                } else {
                    quote! { if cfg!(feature = #feat_str) { #body } else { #not_body } }
                }
            }

            // Explicit + Implicit — cfg!() safe (arms have no .await from this context)
            (
                SelectArmArgs::Explicit { feat, body },
                SelectArmArgs::Implicit { body: imp_body },
            ) => {
                let feat_str = feat.value();
                if has_not_prefix(&feat_str) {
                    let inner = &feat_str[1..];
                    quote! { if cfg!(feature = #inner) { #imp_body } else { #body } }
                } else {
                    quote! { if cfg!(feature = #feat_str) { #body } else { #imp_body } }
                }
            }

            // Implicit + Explicit — cfg!() safe (arms have no .await from this context)
            (
                SelectArmArgs::Implicit { body: imp_body },
                SelectArmArgs::Explicit { feat, body },
            ) => {
                let feat_str = feat.value();
                if has_not_prefix(&feat_str) {
                    let inner = &feat_str[1..];
                    quote! { if cfg!(feature = #inner) { #body } else { #imp_body } }
                } else {
                    quote! { if cfg!(feature = #feat_str) { #body } else { #imp_body } }
                }
            }

            // Both implicit — use #[cfg] blocks to handle .await correctly
            (SelectArmArgs::Implicit { body: b0 }, SelectArmArgs::Implicit { body: b1 }) => {
                let b0_has_await = token_stream_has_await(&b0.to_token_stream());
                let b1_has_await = token_stream_has_await(&b1.to_token_stream());

                match (b0_has_await, b1_has_await) {
                    (true, false) => cfg_block(&quote! { #b0 }, &quote! { #b1 }),
                    (false, true) => cfg_block(&quote! { #b1 }, &quote! { #b0 }),
                    (true, true) => {
                        let b1_stripped = strip_await_from_tokens(&b1.to_token_stream());
                        cfg_block(&quote! { #b0 }, &quote! { #b1_stripped })
                    }
                    (false, false) => {
                        // Neither has .await — use first as async branch as-is
                        cfg_block(&quote! { #b0 }, &quote! { #b1 })
                    }
                }
            }

            // Not + Implicit
            (SelectArmArgs::Not { body: not_body }, SelectArmArgs::Implicit { body: imp_body }) => {
                cfg_block(&quote! { #not_body }, &quote! { #imp_body })
            }

            // Implicit + Not — use #[cfg] blocks to handle .await correctly
            (SelectArmArgs::Implicit { body: imp_body }, SelectArmArgs::Not { body: not_body }) => {
                cfg_block(&quote! { #not_body }, &quote! { #imp_body })
            }

            // Two Not
            (SelectArmArgs::Not { body: b0 }, SelectArmArgs::Not { body: b1 }) => {
                quote! { if cfg!(feature = "async") { #b0 } else { #b1 } }
            }
        }
    }
}

/// Generate a block that returns a value using #[cfg] gates.
/// `async_branch` is used when `feature = "async"` is enabled.
fn cfg_block(async_branch: &TokenStream2, sync_branch: &TokenStream2) -> TokenStream2 {
    quote! {{
        #[cfg(feature = "async")]
        { #async_branch }
        #[cfg(not(feature = "async"))]
        { #sync_branch }
    }}
}

/// Checks if the given string has the '!' (not) prefix.
/// This is used to denote negated feature names in select! arms.
fn has_not_prefix(s: &str) -> bool {
    s.starts_with('!')
}

/// Checks if the given token stream contains a `.await` expression.
///
/// This function traverses the token stream looking for the pattern `. await`,
/// which indicates an `.await` call in Rust syntax. It is used to determine
/// whether an implicit select arm contains async code, which influences how
/// the generated code handles the `cfg` blocks.
///
/// Returns `true` if `.await` is found, `false` otherwise.
fn token_stream_has_await(ts: &TokenStream2) -> bool {
    let mut tokens = ts.clone().into_iter();
    while let Some(token) = tokens.next() {
        if let TokenTree::Punct(p) = &token
            && p.as_char() == '.' && p.spacing() == Spacing::Alone
                && let Some(TokenTree::Ident(ident)) = tokens.next()
                    && ident == "await" {
                        return true;
                    }
    }
    false
}

/// Strips a trailing `.await` from a token stream.
///
/// This function is used when both arms of a `select!` macro contain `.await` expressions.
/// In that case, one arm's `.await` must be removed so that the generated `#[cfg]` blocks
/// produce valid code for both async and sync contexts.
///
/// # Examples
///
/// ```ignore
/// let ts: TokenStream2 = quote! { foo.bar().await };
/// let stripped = strip_await_from_tokens(&ts);
/// assert_eq!(stripped.to_string(), "foo . bar ()");
/// ```
///
/// If the token stream does not end with `.await`, the original stream is returned unchanged.
fn strip_await_from_tokens(ts: &TokenStream2) -> TokenStream2 {
    let tokens: Vec<_> = ts.clone().into_iter().collect();
    let len = tokens.len();
    if len >= 2
        && let TokenTree::Punct(p) = &tokens[len - 2]
            && p.as_char() == '.'
                && let TokenTree::Ident(ident) = &tokens[len - 1]
                    && ident == "await" {
                        return tokens[..len - 2].iter().cloned().collect();
                    }
    ts.clone()
}

#[cfg(test)]
mod tests {
    use crate::select::SelectArmArgs;
    use quote::ToTokens;

    /// Input: "async" => 100  → Explicit variant
    #[test]
    fn test_explicit_arm() {
        let input: proc_macro2::TokenStream = "\"async\" => 100".parse().unwrap();
        let arm: SelectArmArgs = syn::parse2(input).unwrap();
        match &arm {
            SelectArmArgs::Explicit { feat, body } => {
                assert_eq!(feat.value(), "async");
                assert_eq!(body.to_token_stream().to_string(), "100");
            }
            _ => panic!("expected Explicit variant"),
        }
    }

    /// Input: ! => 200  → Not variant
    #[test]
    fn test_not_arm() {
        let input: proc_macro2::TokenStream = "! => 200".parse().unwrap();
        let arm: SelectArmArgs = syn::parse2(input).unwrap();
        match &arm {
            SelectArmArgs::Not { body } => {
                assert_eq!(body.to_token_stream().to_string(), "200");
            }
            _ => panic!("expected Not variant"),
        }
    }

    /// Input: 1 + 2  → Implicit variant (no feature name)
    #[test]
    fn test_implicit_arm() {
        let input: proc_macro2::TokenStream = "1 + 2".parse().unwrap();
        let arm: SelectArmArgs = syn::parse2(input).unwrap();
        match &arm {
            SelectArmArgs::Implicit { body } => {
                assert_eq!(body.to_token_stream().to_string(), "1 + 2");
            }
            _ => panic!("expected Implicit variant"),
        }
    }
}

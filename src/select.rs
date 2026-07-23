use proc_macro::TokenStream;
use proc_macro2::{Spacing, TokenStream as TokenStream2, TokenTree};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, LitStr, Token, parse_macro_input};

// ─── SelectArm ──────────────────────────────────────────────────────────────────────────

/// A single arm inside `select!`.
pub enum SelectArm {
    /// "feat_name" => expr
    Explicit { feat: LitStr, body: Expr },
    /// ! => expr
    Not { body: Expr },
    /// expr (no feature name — auto-detect by .await)
    Implicit { body: Expr },
}

impl Parse for SelectArm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            let feat: LitStr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let body: Expr = input.parse()?;
            Ok(SelectArm::Explicit { feat, body })
        } else if input.peek(Token![!]) {
            let _not: Token![!] = input.parse()?;
            input.parse::<Token![=>]>()?;
            let body: Expr = input.parse()?;
            Ok(SelectArm::Not { body })
        } else {
            let body: Expr = input.parse()?;
            Ok(SelectArm::Implicit { body })
        }
    }
}

// ─── SelectInput ────────────────────────────────────────────────────────────────────────

struct SelectInput {
    arms: Vec<SelectArm>,
}

impl Parse for SelectInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let punctuated: Punctuated<SelectArm, Token![,]> = Punctuated::parse_terminated(input)?;

        if punctuated.len() != 2 {
            return Err(syn::Error::new(
                input.span(),
                "select! requires exactly 2 arms",
            ));
        }

        let arms: Vec<SelectArm> = punctuated.into_iter().collect();
        Ok(SelectInput { arms })
    }
}

// ─── Select macro ───────────────────────────────────────────────────────────────────────

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
                SelectArm::Explicit { feat: f0, body: b0 },
                SelectArm::Explicit { feat: f1, body: b1 },
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
            (SelectArm::Explicit { feat, body }, SelectArm::Not { body: not_body }) => {
                let feat_str = feat.value();
                if has_not_prefix(&feat_str) {
                    let inner = &feat_str[1..];
                    quote! { if cfg!(feature = #inner) { #not_body } else { #body } }
                } else {
                    quote! { if cfg!(feature = #feat_str) { #body } else { #not_body } }
                }
            }
            (SelectArm::Not { body: not_body }, SelectArm::Explicit { feat, body }) => {
                let feat_str = feat.value();
                if has_not_prefix(&feat_str) {
                    let inner = &feat_str[1..];
                    quote! { if cfg!(feature = #inner) { #body } else { #not_body } }
                } else {
                    quote! { if cfg!(feature = #feat_str) { #body } else { #not_body } }
                }
            }

            // Explicit + Implicit — cfg!() safe (arms have no .await from this context)
            (SelectArm::Explicit { feat, body }, SelectArm::Implicit { body: imp_body }) => {
                let feat_str = feat.value();
                if has_not_prefix(&feat_str) {
                    let inner = &feat_str[1..];
                    quote! { if cfg!(feature = #inner) { #imp_body } else { #body } }
                } else {
                    quote! { if cfg!(feature = #feat_str) { #body } else { #imp_body } }
                }
            }
            (SelectArm::Implicit { body: imp_body }, SelectArm::Explicit { feat, body }) => {
                let feat_str = feat.value();
                if has_not_prefix(&feat_str) {
                    let inner = &feat_str[1..];
                    quote! { if cfg!(feature = #inner) { #body } else { #imp_body } }
                } else {
                    quote! { if cfg!(feature = #feat_str) { #body } else { #imp_body } }
                }
            }

            // Both implicit — use #[cfg] blocks to handle .await correctly
            (SelectArm::Implicit { body: b0 }, SelectArm::Implicit { body: b1 }) => {
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
            (SelectArm::Not { body: not_body }, SelectArm::Implicit { body: imp_body }) => {
                cfg_block(&quote! { #not_body }, &quote! { #imp_body })
            }
            (SelectArm::Implicit { body: imp_body }, SelectArm::Not { body: not_body }) => {
                cfg_block(&quote! { #not_body }, &quote! { #imp_body })
            }

            // Two Not
            (SelectArm::Not { body: b0 }, SelectArm::Not { body: b1 }) => {
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

// ─── Helpers ─────────────────────────────────────────────────────────────────────────────

fn has_not_prefix(s: &str) -> bool {
    s.starts_with('!')
}

fn token_stream_has_await(ts: &TokenStream2) -> bool {
    let mut tokens = ts.clone().into_iter();
    while let Some(token) = tokens.next() {
        if let TokenTree::Punct(p) = &token {
            if p.as_char() == '.' && p.spacing() == Spacing::Alone {
                if let Some(TokenTree::Ident(ident)) = tokens.next() {
                    if ident == "await" {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn strip_await_from_tokens(ts: &TokenStream2) -> TokenStream2 {
    let tokens: Vec<_> = ts.clone().into_iter().collect();
    let len = tokens.len();
    if len >= 2 {
        if let TokenTree::Punct(p) = &tokens[len - 2] {
            if p.as_char() == '.' {
                if let TokenTree::Ident(ident) = &tokens[len - 1] {
                    if ident == "await" {
                        return tokens[..len - 2].iter().cloned().collect();
                    }
                }
            }
        }
    }
    ts.clone()
}

use crate::SynResult;
use crate::TokenStream2;
use crate::config::default_feature_name;
use proc_macro::TokenStream;
use proc_macro2::{Spacing, TokenTree};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, LitStr, Token, parse_macro_input};

#[doc = include_str!("../doc/args/select_arm.md")]
pub enum SelectArmArgs {
    /// "feat_name" => { expr }
    Explicit { feat: LitStr, body: Expr },

    /// ! => { expr }
    Not { body: Expr },

    /// { expr } (no feature name — auto-detect by .await)
    Implicit { body: Expr },
}

impl Parse for SelectArmArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        parse_one_arm(input)
    }
}

struct SelectInput {
    arm0: SelectArmArgs,
    arm1: SelectArmArgs,
}

impl Parse for SelectInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let arm0 = parse_one_arm(input)?;
        input.parse::<Token![else]>()?;
        let arm1 = parse_one_arm(input)?;
        Ok(SelectInput { arm0, arm1 })
    }
}

/// Parse one arm: either `"feat" => { expr }`, `! => { expr }`, or `{ expr }`.
pub fn parse_one_arm(input: ParseStream) -> SynResult<SelectArmArgs> {
    // Parse an explicit feature arm: "feat_name" => { expr }
    if input.peek(LitStr) {
        let feat: LitStr = input.parse()?;
        input.parse::<Token![=>]>()?;
        let body: Expr = input.parse()?;
        Ok(SelectArmArgs::Explicit { feat, body })
    }
    // Parse a negation arm: ! => { expr }
    else if input.peek(Token![!]) {
        input.parse::<Token![!]>()?;
        input.parse::<Token![=>]>()?;
        let body: Expr = input.parse()?;
        Ok(SelectArmArgs::Not { body })
    }
    // Parse an implicit arm: { expr } (no feature name — will auto-detect by .await)
    else {
        // Expect a block expression { ... }
        let body: Expr = input.parse()?;
        Ok(SelectArmArgs::Implicit { body })
    }
}

pub(crate) fn select(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SelectInput);
    let expanded = input.expand();
    TokenStream::from(expanded)
}

impl SelectInput {
    fn expand(&self) -> TokenStream2 {
        let arm0 = &self.arm0;
        let arm1 = &self.arm1;

        match (arm0, arm1) {
            // Both explicit — use cfg!() since no .await to worry about
            (
                SelectArmArgs::Explicit { feat: f0, body: b0 },
                SelectArmArgs::Explicit { feat: f1, body: b1 },
            ) => {
                let f0_str = f0.value();
                let f1_str = f1.value();

                if has_not_prefix(&f0_str) && has_not_prefix(&f1_str) {
                    cfg_block(&quote! { #b0 }, &quote! { #b1 })
                } else if has_not_prefix(&f0_str) {
                    let inner = &f0_str[1..];
                    cfg_block_with_feat(inner, &quote! { #b1 }, &quote! { #b0 })
                } else if has_not_prefix(&f1_str) {
                    cfg_block_with_feat(&f0_str, &quote! { #b0 }, &quote! { #b1 })
                } else {
                    both_explicit_block(&f0_str, &quote! { #b0 }, &f1_str, &quote! { #b1 })
                }
            }

            // Explicit + Not — cfg!() safe (no .await)
            (SelectArmArgs::Explicit { feat, body }, SelectArmArgs::Not { body: not_body }) => {
                let feat_str = feat.value();
                if has_not_prefix(&feat_str) {
                    let inner = &feat_str[1..];
                    cfg_block_with_feat(inner, &quote! { #not_body }, &quote! { #body })
                } else {
                    cfg_block_with_feat(&feat_str, &quote! { #body }, &quote! { #not_body })
                }
            }

            // Not + Explicit — cfg!() safe (no .await)
            (SelectArmArgs::Not { body: not_body }, SelectArmArgs::Explicit { feat, body }) => {
                let feat_str = feat.value();
                if has_not_prefix(&feat_str) {
                    let inner = &feat_str[1..];
                    cfg_block_with_feat(inner, &quote! { #body }, &quote! { #not_body })
                } else {
                    cfg_block_with_feat(&feat_str, &quote! { #body }, &quote! { #not_body })
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
                    cfg_block_with_feat(inner, &quote! { #imp_body }, &quote! { #body })
                } else {
                    cfg_block_with_feat(&feat_str, &quote! { #body }, &quote! { #imp_body })
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
                    cfg_block_with_feat(inner, &quote! { #body }, &quote! { #imp_body })
                } else {
                    cfg_block_with_feat(&feat_str, &quote! { #body }, &quote! { #imp_body })
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
                    (false, false) => cfg_block(&quote! { #b0 }, &quote! { #b1 }),
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
                cfg_block(&quote! { #b0 }, &quote! { #b1 })
            }
        }
    }
}

/// Generate a block that uses the default feature name.
///
/// This function creates a `#[cfg]` block that conditionally compiles one of two branches
/// based on whether the default feature (as returned by [`default_feature_name()`]) is enabled.
fn cfg_block(async_branch: &TokenStream2, sync_branch: &TokenStream2) -> TokenStream2 {
    let feat = default_feature_name();
    cfg_block_with_feat(feat, async_branch, sync_branch)
}

/// Generate a block using a specific feature name.
fn cfg_block_with_feat(
    feat: &str,
    async_branch: &TokenStream2,
    sync_branch: &TokenStream2,
) -> TokenStream2 {
    quote! {{
        #[cfg(feature = #feat)]
        { #async_branch }
        #[cfg(not(feature = #feat))]
        { #sync_branch }
    }}
}

/// Generate a block where each arm is gated by its own feature.
fn both_explicit_block(
    feat0: &str,
    branch0: &TokenStream2,
    feat1: &str,
    branch1: &TokenStream2,
) -> TokenStream2 {
    quote! {{
        #[cfg(feature = #feat0)]
        { #branch0 }
        #[cfg(feature = #feat1)]
        { #branch1 }
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
            && p.as_char() == '.'
            && p.spacing() == Spacing::Alone
            && let Some(TokenTree::Ident(ident)) = tokens.next()
            && ident == "await"
        {
            return true;
        }
    }
    false
}

/// Strips a trailing `.await` from a token stream.
fn strip_await_from_tokens(ts: &TokenStream2) -> TokenStream2 {
    let tokens: Vec<_> = ts.clone().into_iter().collect();
    let len = tokens.len();
    if len >= 2
        && let TokenTree::Punct(p) = &tokens[len - 2]
        && p.as_char() == '.'
        && let TokenTree::Ident(ident) = &tokens[len - 1]
        && ident == "await"
    {
        return tokens[..len - 2].iter().cloned().collect();
    }
    ts.clone()
}

#[cfg(test)]
mod tests {
    use crate::select::SelectArmArgs;
    use quote::ToTokens;

    #[test]
    fn test_explicit_arm() {
        let input: proc_macro2::TokenStream = "\"async\" => { 100 }".parse().unwrap();
        let arm: SelectArmArgs = syn::parse2(input).unwrap();
        match &arm {
            SelectArmArgs::Explicit { feat, body } => {
                assert_eq!(feat.value(), "async");
                let s = body.to_token_stream().to_string();
                assert!(s.contains("100"), "body should contain 100, got: {s}");
            }
            _ => panic!("expected Explicit variant"),
        }
    }

    #[test]
    fn test_not_arm() {
        let input: proc_macro2::TokenStream = "! => { 200 }".parse().unwrap();
        let arm: SelectArmArgs = syn::parse2(input).unwrap();
        match &arm {
            SelectArmArgs::Not { body } => {
                let s = body.to_token_stream().to_string();
                assert!(s.contains("200"), "body should contain 200, got: {s}");
            }
            _ => panic!("expected Not variant"),
        }
    }

    #[test]
    fn test_implicit_arm() {
        let input: proc_macro2::TokenStream = "{ 1 + 2 }".parse().unwrap();
        let arm: SelectArmArgs = syn::parse2(input).unwrap();
        match &arm {
            SelectArmArgs::Implicit { body } => {
                let s = body.to_token_stream().to_string();
                assert!(
                    s.contains("1 + 2") || s.contains("1+2"),
                    "body should contain 1 + 2, got: {s}"
                );
            }
            _ => panic!("expected Implicit variant"),
        }
    }

    #[test]
    fn test_cfg_block_generates_cfg_attributes() {
        use crate::select::cfg_block;
        let result = cfg_block(&quote::quote! { async_fn() }, &quote::quote! { sync_fn() });
        let s = result.to_string();
        assert!(
            s.contains("cfg (feature"),
            "expected cfg(feature in output, got: {s}"
        );
        assert!(
            s.contains("async_fn"),
            "expected async_fn in output, got: {s}"
        );
        assert!(
            s.contains("sync_fn"),
            "expected sync_fn in output, got: {s}"
        );
    }

    #[test]
    fn test_cfg_block_uses_default_feature() {
        use crate::config::default_feature_name;
        use crate::select::cfg_block;
        let result = cfg_block(&quote::quote! { a }, &quote::quote! { b });
        let s = result.to_string();
        let default_feat = default_feature_name();
        // TokenStream representation adds spaces: (feature = "foo_async")
        assert!(s.contains(&format!("feature = {:?}", default_feat)));
    }

    #[test]
    fn test_cfg_block_with_feat_uses_custom_feature() {
        use crate::select::cfg_block_with_feat;
        let result = cfg_block_with_feat("my_feat", &quote::quote! { a }, &quote::quote! { b });
        let s = result.to_string();
        // TokenStream inserts spaces around punctuation, so match the actual format
        assert!(
            s.contains("# [cfg (feature = \"my_feat\")]"),
            "expected #[cfg(feature = \"my_feat\")] in output, got: {s}"
        );
        assert!(
            s.contains("# [cfg (not (feature = \"my_feat\"))]"),
            "expected #[cfg(not(feature = \"my_feat\"))] in output, got: {s}"
        );
    }

    #[test]
    fn test_cfg_block_with_feat_branch_order() {
        use crate::select::cfg_block_with_feat;
        // The async_branch (first branch) goes under cfg(feature = ...)
        // The sync_branch (second branch) goes under cfg(not(...))
        let result = cfg_block_with_feat(
            "x",
            &quote::quote! { first_val },
            &quote::quote! { second_val },
        );
        let s = result.to_string();
        // TokenStream representation has spaces: # [cfg (feature = "x")]
        let feat_pos = s.find("# [cfg (feature = \"x\")]").unwrap();
        let first_pos = s.find("first_val").unwrap();
        // second_branch should be after cfg(not(...))
        let not_pos = s.find("# [cfg (not (feature = \"x\"))]").unwrap();
        let second_pos = s.find("second_val").unwrap();
        assert!(
            feat_pos < first_pos,
            "async_branch should follow #[cfg(feature)]"
        );
        assert!(
            not_pos < second_pos,
            "sync_branch should follow #[cfg(not)]"
        );
    }

    #[test]
    fn test_both_explicit_block_generates_two_cfg_gates() {
        use crate::select::both_explicit_block;
        let result = both_explicit_block(
            "a",
            &quote::quote! { branch_a },
            "b",
            &quote::quote! { branch_b },
        );
        let s = result.to_string();
        // TokenStream representation has spaces
        assert!(
            s.contains("# [cfg (feature = \"a\")]"),
            "expected #[cfg(feature = \"a\")] in output, got: {s}"
        );
        assert!(
            s.contains("# [cfg (feature = \"b\")]"),
            "expected #[cfg(feature = \"b\")] in output, got: {s}"
        );
        assert!(s.contains("branch_a"));
        assert!(s.contains("branch_b"));
    }

    #[test]
    fn test_both_explicit_block_no_not_cfg() {
        use crate::select::both_explicit_block;
        let result =
            both_explicit_block("x", &quote::quote! { x_val }, "y", &quote::quote! { y_val });
        let s = result.to_string();
        // Should NOT contain cfg(not(...))
        assert!(!s.contains("# [cfg (not"));
    }

    #[test]
    fn test_has_not_prefix_with_exclamation() {
        use crate::select::has_not_prefix;
        assert!(has_not_prefix("!feat"));
        assert!(has_not_prefix("!"));
        assert!(has_not_prefix("!abc"));
    }

    #[test]
    fn test_has_not_prefix_without_exclamation() {
        use crate::select::has_not_prefix;
        assert!(!has_not_prefix("feat"));
        assert!(!has_not_prefix(""));
        assert!(!has_not_prefix("abc!"));
        // "!!" starts with '!', so this is actually true — fix the assertion
        assert!(has_not_prefix("!!"));
    }

    #[test]
    fn test_token_stream_has_await_detects_await() {
        use crate::select::token_stream_has_await;
        let ts: proc_macro2::TokenStream = "foo.await".parse().unwrap();
        assert!(token_stream_has_await(&ts));
    }

    #[test]
    fn test_token_stream_has_await_no_await() {
        use crate::select::token_stream_has_await;
        let ts: proc_macro2::TokenStream = "foo.bar()".parse().unwrap();
        assert!(!token_stream_has_await(&ts));
    }

    #[test]
    fn test_token_stream_has_await_empty() {
        use crate::select::token_stream_has_await;
        let ts: proc_macro2::TokenStream = "".parse().unwrap();
        assert!(!token_stream_has_await(&ts));
    }

    #[test]
    fn test_token_stream_has_await_await_in_expr() {
        use crate::select::token_stream_has_await;
        // .await inside a larger expression
        let ts: proc_macro2::TokenStream = "let x = some_future.await;".parse().unwrap();
        assert!(token_stream_has_await(&ts));
    }

    #[test]
    fn test_token_stream_has_await_dot_only_not_await() {
        use crate::select::token_stream_has_await;
        // . followed by something else like .foo()
        let ts: proc_macro2::TokenStream = "x.foo()".parse().unwrap();
        assert!(!token_stream_has_await(&ts));
    }

    #[test]
    fn test_strip_await_from_tokens_removes_await() {
        use crate::select::strip_await_from_tokens;
        let ts: proc_macro2::TokenStream = "foo.await".parse().unwrap();
        let stripped = strip_await_from_tokens(&ts);
        let s = stripped.to_string();
        assert!(
            !s.contains("await"),
            "stripped output should not contain 'await', got: {s}"
        );
    }

    #[test]
    fn test_strip_await_from_tokens_preserves_prefix() {
        use crate::select::strip_await_from_tokens;
        let ts: proc_macro2::TokenStream = "some_future.await".parse().unwrap();
        let stripped = strip_await_from_tokens(&ts);
        let s = stripped.to_string();
        assert!(
            s.contains("some_future"),
            "should preserve 'some_future', got: {s}"
        );
    }

    #[test]
    fn test_strip_await_from_tokens_no_await_is_unchanged() {
        use crate::select::strip_await_from_tokens;
        let ts: proc_macro2::TokenStream = "simple_expr".parse().unwrap();
        let stripped = strip_await_from_tokens(&ts);
        assert_eq!(stripped.to_string(), "simple_expr");
    }

    #[test]
    fn test_strip_await_from_tokens_empty_is_unchanged() {
        use crate::select::strip_await_from_tokens;
        let ts: proc_macro2::TokenStream = "".parse().unwrap();
        let stripped = strip_await_from_tokens(&ts);
        assert_eq!(stripped.to_string(), "");
    }

    #[test]
    fn test_strip_await_from_tokens_complex_expr() {
        use crate::select::strip_await_from_tokens;
        let ts: proc_macro2::TokenStream = "async_fn().await".parse().unwrap();
        let stripped = strip_await_from_tokens(&ts);
        let s = stripped.to_string();
        assert!(
            s.contains("async_fn"),
            "should preserve 'async_fn', got: {s}"
        );
        assert!(!s.contains("await"), "should not contain 'await', got: {s}");
    }
}

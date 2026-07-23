use proc_macro::TokenStream;

mod args;
pub(crate) mod func;
pub(crate) mod invoke;
pub(crate) mod select;

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
#[proc_macro_attribute]
pub fn func(attr: TokenStream, item: TokenStream) -> TokenStream {
    func::func(attr, item)
}

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
#[proc_macro]
pub fn invoke(input: TokenStream) -> TokenStream {
    invoke::invoke(input)
}

/// Select between sync and async expressions based on a Cargo feature flag.
///
/// # Usage
///
/// ## Explicit mode (with feature names)
///
/// ```ignore
/// select!["async" => expr_async().await, "sync" => expr_sync()];
/// select!["async" => { expr_async().await }, "sync" => { expr_sync() }];
/// select!["async" => expr_async().await, !        => expr_sync()];
/// select![!        => expr_async().await, "sync" => expr_sync()];
/// ```
///
/// ## Implicit mode (auto-detect `.await`)
///
/// ```ignore
/// select![expr_async().await, expr_sync()];
/// select![{ expr_async().await }, { expr_sync() }];
/// ```
#[proc_macro]
pub fn select(input: TokenStream) -> TokenStream {
    select::select(input)
}

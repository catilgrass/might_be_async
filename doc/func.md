Attribute macro that generates both a sync and an async version of a function,
gated by a Cargo feature flag.

# Usage

```ignore
/// Doc comments are preserved
#[might_be_async::func]
pub fn my_function<T: Clone>(arg: T) -> ReturnType
where T: Debug
{
    // body — written as a regular (non-async) function
}
```

Expands to:

- `#[cfg(not(feature = "async"))] fn my_function(...)` — sync version
- `#[cfg(feature = "async")] async fn my_function(...)` — async version

An explicit feature name can be provided:

```ignore
#[might_be_async::func("tokio_rt")]
pub fn my_function() { ... }
```

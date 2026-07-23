Attribute macro that generates both a sync and an async version of a function,
gated by a Cargo feature flag.

The function is written as a regular (non-async) `fn`. The macro duplicates it
internally into two implementations:

- When the feature is **disabled**: the original `fn` is used as-is.
- When the feature is **enabled**: the function is promoted to `async fn`.

By default the feature name is `"async"`. A different name can be supplied as
an attribute argument, for example `#[func("tokio_rt")]` to gate on the feature
`"tokio_rt"` instead.

Attribute macro that generates both a sync and an async version of a function,
gated by a Cargo feature flag.

The function is written as a regular (non-async) `fn`. The macro duplicates it:

| Feature disabled | Feature enabled      |
| ---------------- | -------------------- |
| `fn name(...)`   | `async fn name(...)` |

By default the feature name is `"async"`. A custom name can be provided:
`#[func("tokio_rt")]`.

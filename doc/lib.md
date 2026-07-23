# might_be_async

A proc-macro crate that generates both synchronous and asynchronous versions of
functions, gated by a Cargo feature flag.

## Provided macros

| Macro     | Kind       | Purpose                                                   |
| --------- | ---------- | --------------------------------------------------------- |
| `#[func]` | attribute  | Generates sync + async versions of a function             |
| `invoke!` | procedural | Wraps a call, adding `.await` when the feature is enabled |
| `select!` | procedural | Chooses between two expressions based on the feature flag |

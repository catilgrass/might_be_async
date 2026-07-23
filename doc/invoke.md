Wraps a call expression, adding `.await` when the async feature is enabled.

When the feature is **disabled**, the expression is left as-is:

```rust
double(5)        // sync: call directly
```

When the feature is **enabled**, `.await` is appended:

```rust
double(5).await  // async: await the future
```

An explicit feature name can be provided: `invoke!("tokio_rt" => double(5))`.

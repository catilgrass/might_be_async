Wraps a call expression, adding `.await` when the async feature is enabled.

# Usage

```ignore
// Default feature name ("async"):
let result = might_be_async::invoke!(some_async_fn(args));

// Explicit feature name:
let result = might_be_async::invoke!("tokio_rt" => some_async_fn(args));
```

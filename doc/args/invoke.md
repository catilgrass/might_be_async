Arguments parsed by the `invoke!` macro.

Two forms:

| Variant    | Form                      | Behavior                           |
| ---------- | ------------------------- | ---------------------------------- |
| `Default`  | `invoke!(expr)`           | Feature name defaults to `"async"` |
| `Explicit` | `invoke!("feat" => expr)` | Uses the given feature name        |

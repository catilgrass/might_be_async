# might_be_async

A proc-macro crate that generates both synchronous and asynchronous versions of
functions, gated by a Cargo feature flag.

## Provided macros

| Macro     | Kind       | Purpose                                                   |
| --------- | ---------- | --------------------------------------------------------- |
| `#[func]` | attribute  | Generates sync + async versions of a function             |
| `invoke!` | procedural | Wraps a call, adding `.await` when the feature is enabled |
| `select!` | procedural | Chooses between two expressions based on the feature flag |

## Complete example

The three macros are designed to work together. `#[func]` keeps a set of
functions in sync: when the feature is off they are all plain `fn`, and when
it is on they are all `async fn`. `invoke!` calls between them automatically
add or omit `.await`, and `select!` picks the correct branch at compile time.

```rust
use might_be_async::{func, invoke, select};

/// A simple computation, gated by the feature flag.
#[func]
fn double(x: i32) -> i32 {
    x * 2
}

/// Calls `double` through `invoke!` — the call is plain or awaited
/// depending on whether the feature is active.
#[func]
fn add_then_double(a: i32, b: i32) -> i32 {
    let sum = a + b;
    invoke!(double(sum))
}

/// Uses `select!` to return a different value in each mode.
#[func]
fn pick(flag: bool) -> i32 {
    if flag {
        select!("async" => 100 else ! => 200)
    } else {
        0
    }
}

assert_eq!(add_then_double(3, 4), 14);
assert_eq!(pick(true), 200);
assert_eq!(pick(false), 0);
```

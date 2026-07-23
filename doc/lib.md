# might_be_async

A proc-macro crate that generates both synchronous and asynchronous versions of
functions, gated by a Cargo feature flag.

## Provided macros

### `#[func]` — Attribute macro

Generates both synchronous and asynchronous versions of a function, gated by a Cargo feature flag. When the feature is off, the function is a plain `fn`; when on, it becomes an `async fn`.

Accepts an optional string argument specifying the feature name (defaults to `"async"`).

```rust
// Uses default feature name "async"
#[might_be_async::func]
fn compute(x: i32) -> i32 { x }

// Custom feature name
#[might_be_async::func("foo_async")]
fn fetch(url: &str) -> String { "url".into() }
```

### `invoke!` — Proc macro

Wraps a function call, automatically adding `.await` when the feature is enabled. This lets you call `#[func]`-annotated functions without worrying about whether they are sync or async at the call site.

Accepts an optional feature name before `=>` (defaults to `"async"`).

```rust
use might_be_async::invoke;
# use std::env::args;
# #[might_be_async::func]
# fn entry() {
# let url = String::default();

// Uses default feature name "async"
invoke!(do_stuff(args()));

// Custom feature name
invoke!("foo_async" => fetch_data(url));
# }
# #[might_be_async::func]
# fn fetch_data(url: String) {}
# #[might_be_async::func]
# fn do_stuff(url: std::env::Args) {}
```

### `select!` — Proc macro

Chooses between two expressions at compile time based on whether the feature flag is active. The first branch (marked with `"async" =>`) is taken when the feature is enabled; the second branch (marked with `! =>`) is taken otherwise.

Supports three arm syntaxes:

```rust
use might_be_async::select;
# #[might_be_async::func("foo_async")]
# fn entry() {

// Explicit feature name arm and a negation arm
select!("foo_async" => { async_expr().await } else ! => { sync_expr() });

// Two explicit feature names (second can use ! prefix)
select!("foo_sync" => expr_a() else "foo_async" => expr_b().await);

// Implicit arms — auto-detects .await to decide async vs sync
select!({ expr_with_await().await } else { expr_without_await() });

# }
# async fn expr_with_await () {}
# fn expr_without_await () {}
# async fn async_expr () {}
# fn sync_expr () {}
# async fn expr_b () {}
# fn expr_a () {}
```

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
        select!("foo_async" => 100 else ! => 200)
    } else {
        0
    }
}

assert_eq!(add_then_double(3, 4), 14);
assert_eq!(pick(true), 200);
assert_eq!(pick(false), 0);
```

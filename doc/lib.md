# might_be_async

A proc-macro crate that generates both synchronous and asynchronous versions of
functions, gated by a Cargo feature flag.

## Provided macros

### `#[func]` — Attribute macro

Generates both synchronous and asynchronous versions of a function, gated by a Cargo feature flag. When the feature is off, the function is a plain `fn`; when on, it becomes an `async fn`.

Accepts an optional string argument specifying the feature name (defaults to `"async"`).

```rust
# use might_be_async::func;
// Uses default feature name "async"
#[func]
fn compute(x: i32) -> i32 { ... }

// Custom feature name
#[func("tokio")]
fn fetch(url: &str) -> String { ... }
```

### `invoke!` — Proc macro

Wraps a function call, automatically adding `.await` when the feature is enabled. This lets you call `#[func]`-annotated functions without worrying about whether they are sync or async at the call site.

Accepts an optional feature name before `=>` (defaults to `"async"`).

```rust
# use might_be_async::invoke;
// Uses default feature name "async"
invoke!(do_stuff(args))

// Custom feature name
invoke!("tokio" => fetch_data(url))
```

### `select!` — Proc macro

Chooses between two expressions at compile time based on whether the feature flag is active. The first branch (marked with `"async" =>`) is taken when the feature is enabled; the second branch (marked with `! =>`) is taken otherwise.

Supports three arm syntaxes:

```rust
# use might_be_async::select;
// Explicit feature name arm and a negation arm
select!("async" => { async_expr().await } else ! => { sync_expr() });

// Two explicit feature names (second can use ! prefix)
select!("sync" => { expr_a() } else "async" => { expr_b().await });

// Implicit arms — auto-detects .await to decide async vs sync
select!({ expr_with_await().await } else { expr_without_await() });

# async fn expr_with_await () {}
# fn expr_without_await () {}
# async fn async_expr () {}
# fn sync_expr () {}
# async fn expr_a () {}
# fn expr_b () {}
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
        select!("async" => 100 else ! => 200)
    } else {
        0
    }
}

assert_eq!(add_then_double(3, 4), 14);
assert_eq!(pick(true), 200);
assert_eq!(pick(false), 0);
```

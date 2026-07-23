# might_be_async

> A proc macro that gen both async & sync fn, toggle via feature flag.

## Installation

Add the dependency in `Cargo.toml`:

```toml
[dependencies]
might_be_async = "0.1.0"
```

Or use the `cargo add` command:

```bash
cargo add might_be_async
```

# Usage

First, choose a feature name for your async toggle. By default `might_be_async` uses the feature name `"async"`, but you can use any name you like.

```toml
# Cargo.toml

[package.metadata.might_be_async]
default_feature_name = "async" # Default

[dependencies]
might_be_async = "0.1.0"
```

Now you can annotate your functions with `#[func]`:

```rust
#[might_be_async::func]
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
```

When the `async` feature is **off**, `greet` remains a plain synchronous `fn`.
When the `async` feature is **on**, `greet` becomes an `async fn` — its signature changes to `async fn greet(name: &str) -> String`.

## Calling annotated functions

If one `#[func]` function calls another, use the `invoke!` macro to handle
the `.await` automatically:

```rust
#[might_be_async::func]
fn double(x: i32) -> i32 {
    x * 2
}

#[might_be_async::func]
fn compute(a: i32, b: i32) -> i32 {
    let sum = a + b;
    might_be_async::invoke!(double(sum))   // adds .await in async mode, does nothing in sync mode
}
```

## Feature-specific logic with `select!`

Use `select!` to write code that differs between sync and async modes:

```rust
#[might_be_async::func]
fn load_data() -> Vec<u8> {
    might_be_async::select!("async" => fetch_async().await else ! => fetch_sync())
}

async fn fetch_async() -> Vec<u8> {
    // ...
    vec![]
}

fn fetch_sync() -> Vec<u8> {
    // ...
    vec![]
}
```

When the feature `"async"` is active, the first branch runs. When it's not,
the second branch (marked with `!`) runs.

## Switching modes

Run in sync mode (default):

```bash
cargo build
cargo run
```

Run in async mode:

```bash
cargo build --features async
cargo run --features async
```

## License

MIT OR Apache-2.0"

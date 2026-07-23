//! Tests for the `#[func]` attribute macro.
//!
//! Run default (sync path):
//!     cargo test --test test_func
//!
//! Run async path:
//!     cargo test --features async --test test_func

#[might_be_async::func]
fn identity(x: i32) -> i32 {
    x
}

#[test]
fn test_identity() {
    assert_eq!(identity(42), 42);
}

// ─── With generics and where clause ─────────────────────────────────────────────────────

#[might_be_async::func]
fn first<T: PartialEq + Clone>(a: T, _b: T) -> T
where
    T: std::fmt::Debug,
{
    a.clone()
}

#[test]
fn test_first() {
    assert_eq!(first(1, 2), 1);
    assert_eq!(first("hello", "world"), "hello");
}

// ─── With explicit feature name ─────────────────────────────────────────────────────────

#[might_be_async::func("async")]
fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}

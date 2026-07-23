//! Tests for the `invoke!` macro.

/// A mock "async" function — in sync mode it just returns directly.
#[cfg(not(feature = "async"))]
fn compute(x: i32) -> i32 {
    x * 2
}

#[cfg(feature = "async")]
async fn compute(x: i32) -> i32 {
    x * 2
}

#[cfg(not(feature = "async"))]
#[test]
fn test_invoke_default() {
    let r = might_be_async::invoke!(compute(5));
    assert_eq!(r, 10);
}

#[cfg(feature = "async")]
#[test]
fn test_invoke_default() {
    let r = might_be_async::invoke!(compute(5));
    assert_eq!(r, 10);
}

// ─── Explicit feature name ──────────────────────────────────────────────────────────────

#[cfg(not(feature = "async"))]
fn square(x: i32) -> i32 {
    x * x
}

#[cfg(feature = "async")]
async fn square(x: i32) -> i32 {
    x * x
}

#[cfg(not(feature = "async"))]
#[test]
fn test_invoke_explicit() {
    let r = might_be_async::invoke!("async" => square(6));
    assert_eq!(r, 36);
}

#[cfg(feature = "async")]
#[test]
fn test_invoke_explicit() {
    let r = might_be_async::invoke!("async" => square(6));
    assert_eq!(r, 36);
}

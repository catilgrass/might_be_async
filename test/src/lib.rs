// ─── #[func] tests ─────────────────────────────────────────────────────────────────────

#[might_be_async::func]
fn identity(x: i32) -> i32 {
    x
}

#[cfg(not(feature = "async"))]
#[test]
fn test_func_identity() {
    assert_eq!(identity(42), 42);
}

#[might_be_async::func]
fn first<T: PartialEq + Clone>(a: T, _b: T) -> T
where
    T: std::fmt::Debug,
{
    a.clone()
}

#[cfg(not(feature = "async"))]
#[test]
fn test_func_generic() {
    assert_eq!(first(1, 2), 1);
    assert_eq!(first("a", "b"), "a");
}

// ─── invoke! tests ──────────────────────────────────────────────────────────────────────

#[cfg(not(feature = "async"))]
fn double(x: i32) -> i32 {
    x * 2
}

#[cfg(feature = "async")]
async fn double(x: i32) -> i32 {
    x * 2
}

#[cfg(not(feature = "async"))]
#[test]
fn test_invoke_default() {
    assert_eq!(might_be_async::invoke!(double(5)), 10);
}

#[cfg(feature = "async")]
#[test]
fn test_invoke_default() {
    assert_eq!(might_be_async::invoke!(double(5)), 10);
}

// ─── select! tests ──────────────────────────────────────────────────────────────────────

#[cfg(not(feature = "async"))]
#[test]
fn test_select_explicit() {
    let r = might_be_async::select! { "async" => 1, "sync" => 2 };
    assert_eq!(r, 2);
}

#[cfg(feature = "async")]
#[test]
fn test_select_explicit() {
    let r = might_be_async::select! { "async" => 1, "sync" => 2 };
    assert_eq!(r, 1);
}

#[cfg(not(feature = "async"))]
#[test]
fn test_select_not() {
    let r = might_be_async::select! { "async" => 10, ! => 20 };
    assert_eq!(r, 20);
}

#[cfg(feature = "async")]
#[test]
fn test_select_not() {
    let r = might_be_async::select! { "async" => 10, ! => 20 };
    assert_eq!(r, 10);
}

#[cfg(not(feature = "async"))]
#[test]
fn test_select_not_first() {
    let r = might_be_async::select! { ! => 30, "async" => 40 };
    assert_eq!(r, 30);
}

#[cfg(feature = "async")]
#[test]
fn test_select_not_first() {
    let r = might_be_async::select! { ! => 30, "async" => 40 };
    assert_eq!(r, 40);
}

#[cfg(not(feature = "async"))]
#[test]
fn test_select_implicit() {
    let r = might_be_async::select! { 1 + 2, 3 + 4 };
    assert_eq!(r, 7);
}

#[cfg(feature = "async")]
#[test]
fn test_select_implicit() {
    let r = might_be_async::select! { 1 + 2, 3 + 4 };
    assert_eq!(r, 3);
}

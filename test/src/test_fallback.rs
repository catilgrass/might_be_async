#[might_be_async::func]
fn fallback_id(x: i32) -> i32 {
    x
}

#[cfg(not(feature = "async"))]
#[test]
fn test_fallback_func_sync() {
    assert_eq!(fallback_id(42), 42);
}

#[cfg(feature = "async")]
#[test]
fn test_fallback_func_async() {
    let r = futures::executor::block_on(fallback_id(42));
    assert_eq!(r, 42);
}

#[cfg(not(feature = "async"))]
#[test]
fn test_fallback_select_implicit_sync() {
    let r = might_be_async::select! { { 1 } else { 2 } };
    assert_eq!(r, 2);
}

#[cfg(feature = "async")]
#[test]
fn test_fallback_select_implicit_async() {
    let r = might_be_async::select! { { 1 } else { 2 } };
    assert_eq!(r, 1);
}

#[cfg(not(feature = "async"))]
fn fallback_double(x: i32) -> i32 {
    x * 2
}

#[cfg(feature = "async")]
async fn fallback_double(x: i32) -> i32 {
    x * 2
}

#[cfg(not(feature = "async"))]
#[test]
fn test_fallback_invoke_sync() {
    assert_eq!(might_be_async::invoke!(fallback_double(5)), 10);
}

#[cfg(feature = "async")]
#[test]
fn test_fallback_invoke_async() {
    let r = futures::executor::block_on(async { might_be_async::invoke!(fallback_double(5)) });
    assert_eq!(r, 10);
}

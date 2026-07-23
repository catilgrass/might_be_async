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

#[cfg(not(feature = "metadata_async"))]
fn double(x: i32) -> i32 {
    x * 2
}

#[cfg(feature = "metadata_async")]
async fn double(x: i32) -> i32 {
    x * 2
}

#[cfg(not(feature = "metadata_async"))]
#[test]
fn test_invoke_default() {
    assert_eq!(might_be_async::invoke!(double(5)), 10);
}

#[cfg(feature = "metadata_async")]
#[test]
fn test_invoke_default() {
    let result = futures::executor::block_on(async { might_be_async::invoke!(double(5)) });
    assert_eq!(result, 10);
}

#[might_be_async::func]
fn square(x: i32) -> i32 {
    x * x
}

#[cfg(not(feature = "metadata_async"))]
#[test]
fn test_invoke_explicit() {
    assert_eq!(might_be_async::invoke!("custom_invoke" => square(6)), 36);
}

#[cfg(feature = "metadata_async")]
#[test]
fn test_invoke_explicit_sync() {
    let result = futures::executor::block_on(async {
        might_be_async::invoke!("custom_invoke" => square(6))
    });
    assert_eq!(result, 36);
}

#[cfg(not(feature = "metadata_async"))]
#[test]
fn test_invoke_explicit_async() {
    let result = might_be_async::invoke!("custom_invoke" => square(6));
    assert_eq!(result, 36);
}

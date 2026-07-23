#[might_be_async::func]
fn identity(x: i32) -> i32 {
    x
}

#[cfg(not(feature = "metadata_async"))]
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

#[cfg(not(feature = "metadata_async"))]
#[test]
fn test_func_generic() {
    assert_eq!(first(1, 2), 1);
    assert_eq!(first("a", "b"), "a");
}

#[might_be_async::func("custom_ft")]
fn triple(x: i32) -> i32 {
    x * 3
}

#[cfg(not(feature = "metadata_async"))]
#[test]
fn test_func_custom_feature() {
    assert_eq!(triple(3), 9);
}

#[cfg(feature = "metadata_async")]
#[test]
fn test_func_async_identity() {
    let result = futures::executor::block_on(async { identity(42).await });
    assert_eq!(result, 42);
}

#[cfg(feature = "metadata_async")]
#[test]
fn test_func_async_generic() {
    let r = futures::executor::block_on(async { first(10, 20).await });
    assert_eq!(r, 10);
}

#[might_be_async::func]
fn greet_async(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(not(feature = "metadata_async"))]
#[test]
fn test_func_sync_greet() {
    assert_eq!(greet_async("world"), "Hello, world!");
}

#[cfg(feature = "metadata_async")]
#[test]
fn test_func_async_greet() {
    let r = futures::executor::block_on(greet_async("world"));
    assert_eq!(r, "Hello, world!");
}

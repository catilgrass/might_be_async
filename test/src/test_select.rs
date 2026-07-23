// Tests for the `select!` macro (shared).

#[cfg(not(feature = "async"))]
#[test]
fn test_select_explicit() {
    let r = might_be_async::select! { "async" => { 1 } else "sync" => { 2 } };
    assert_eq!(r, 2);
}

#[cfg(feature = "async")]
#[test]
fn test_select_explicit() {
    let r = might_be_async::select! { "async" => { 1 } else "sync" => { 2 } };
    assert_eq!(r, 1);
}

#[cfg(not(feature = "async"))]
#[test]
fn test_select_not() {
    let r = might_be_async::select! { "async" => { 10 } else ! => { 20 } };
    assert_eq!(r, 20);
}

#[cfg(feature = "async")]
#[test]
fn test_select_not() {
    let r = might_be_async::select! { "async" => { 10 } else ! => { 20 } };
    assert_eq!(r, 10);
}

#[cfg(not(feature = "async"))]
#[test]
fn test_select_not_first() {
    let r = might_be_async::select! { ! => { 30 } else "async" => { 40 } };
    assert_eq!(r, 30);
}

#[cfg(feature = "async")]
#[test]
fn test_select_not_first() {
    let r = might_be_async::select! { ! => { 30 } else "async" => { 40 } };
    assert_eq!(r, 40);
}

#[cfg(not(feature = "async"))]
#[test]
fn test_select_implicit() {
    let r = might_be_async::select! { { 1 + 2 } else { 3 + 4 } };
    assert_eq!(r, 7);
}

#[cfg(feature = "async")]
#[test]
fn test_select_implicit() {
    let r = futures::executor::block_on(async {
        might_be_async::select! { { 1 + 2 } else { 3 + 4 } }
    });
    assert_eq!(r, 3);
}

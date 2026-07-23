#[cfg(not(feature = "metadata_async"))]
#[test]
fn test_select_explicit() {
    let r = might_be_async::select! { "metadata_async" => { 1 } else "sync" => { 2 } };
    assert_eq!(r, 2);
}

#[cfg(feature = "metadata_async")]
#[test]
fn test_select_explicit() {
    let r = might_be_async::select! { "metadata_async" => { 1 } else "sync" => { 2 } };
    assert_eq!(r, 1);
}

#[cfg(not(feature = "metadata_async"))]
#[test]
fn test_select_not() {
    let r = might_be_async::select! { "metadata_async" => { 10 } else ! => { 20 } };
    assert_eq!(r, 20);
}

#[cfg(feature = "metadata_async")]
#[test]
fn test_select_not() {
    let r = might_be_async::select! { "metadata_async" => { 10 } else ! => { 20 } };
    assert_eq!(r, 10);
}

#[cfg(not(feature = "metadata_async"))]
#[test]
fn test_select_not_first() {
    let r = might_be_async::select! { ! => { 30 } else "metadata_async" => { 40 } };
    assert_eq!(r, 30);
}

#[cfg(feature = "metadata_async")]
#[test]
fn test_select_not_first() {
    let r = might_be_async::select! { ! => { 30 } else "metadata_async" => { 40 } };
    assert_eq!(r, 40);
}

#[cfg(not(feature = "metadata_async"))]
#[test]
fn test_select_implicit() {
    let r = might_be_async::select! { { 1 + 2 } else { 3 + 4 } };
    assert_eq!(r, 7);
}

#[cfg(feature = "metadata_async")]
#[test]
fn test_select_implicit() {
    let r = futures::executor::block_on(async {
        might_be_async::select! { { 1 + 2 } else { 3 + 4 } }
    });
    assert_eq!(r, 3);
}

#[cfg(not(feature = "metadata_async"))]
#[test]
fn test_select_mixed_explicit_first() {
    let r = might_be_async::select! { "metadata_async" => { 100 } else { 200 } };
    assert_eq!(r, 200);
}

#[cfg(feature = "metadata_async")]
#[test]
fn test_select_mixed_explicit_first() {
    let r = might_be_async::select! { "metadata_async" => { 100 } else { 200 } };
    assert_eq!(r, 100);
}

#[cfg(not(feature = "metadata_async"))]
#[test]
fn test_select_metadata_default() {
    let r = might_be_async::select! { { 50 } else { 60 } };
    assert_eq!(r, 60);
}

#[cfg(not(feature = "metadata_async"))]
#[test]
fn test_select_metadata_two_not() {
    let r = might_be_async::select! { ! => { 70 } else ! => { 80 } };
    assert_eq!(r, 80);
}

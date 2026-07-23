//! Tests for the `select!` macro.

// ─── Explicit mode ──────────────────────────────────────────────────────────────────────

#[cfg(not(feature = "async"))]
#[test]
fn test_select_explicit_sync() {
    let r = might_be_async::select! {
        "async" => 100,
        "sync"  => 200
    };
    assert_eq!(r, 200);
}

#[cfg(feature = "async")]
#[test]
fn test_select_explicit_async() {
    let r = might_be_async::select! {
        "async" => 100,
        "sync"  => 200
    };
    assert_eq!(r, 100);
}

// ─── Explicit + Not ─────────────────────────────────────────────────────────────────────

#[cfg(not(feature = "async"))]
#[test]
fn test_select_not_sync() {
    let r = might_be_async::select! {
        "async" => 10,
        !        => 20
    };
    assert_eq!(r, 20);
}

#[cfg(feature = "async")]
#[test]
fn test_select_not_async() {
    let r = might_be_async::select! {
        "async" => 10,
        !        => 20
    };
    assert_eq!(r, 10);
}

// ─── Not + explicit ─────────────────────────────────────────────────────────────────────

#[cfg(not(feature = "async"))]
#[test]
fn test_select_not_first_sync() {
    let r = might_be_async::select! {
        !        => 30,
        "async"  => 40
    };
    assert_eq!(r, 30);
}

#[cfg(feature = "async")]
#[test]
fn test_select_not_first_async() {
    let r = might_be_async::select! {
        !        => 30,
        "async"  => 40
    };
    assert_eq!(r, 40);
}

// ─── Implicit mode: neither has .await ──────────────────────────────────────────────────

#[cfg(not(feature = "async"))]
#[test]
fn test_select_implicit_no_await_sync() {
    let r = might_be_async::select! { 10 + 20, 30 + 40 };
    assert_eq!(r, 70);
}

#[cfg(feature = "async")]
#[test]
fn test_select_implicit_no_await_async() {
    let r = might_be_async::select! { 10 + 20, 30 + 40 };
    assert_eq!(r, 30);
}

// ─── Implicit mode with block expressions ───────────────────────────────────────────────

#[cfg(not(feature = "async"))]
#[test]
fn test_select_implicit_block_sync() {
    let r = might_be_async::select! {
        { 50 },
        { 60 }
    };
    assert_eq!(r, 60);
}

#[cfg(feature = "async")]
#[test]
fn test_select_implicit_block_async() {
    let r = might_be_async::select! {
        { 50 },
        { 60 }
    };
    assert_eq!(r, 50);
}

1. 隐式，feature声明在 Cargo.toml 的 profile.metadata 中

```rust
#[might_be_async::func]
fn your_function(who_care: SomeFunc) -> ReturnValue {
    let r: ReturnValue = might_be_async::invoke!(your_another_function(who_care));
}

#[might_be_async::func]
fn your_another_function(who_care: SomeFunc) -> ReturnValue {
    might_be_async::select![
        might_be_async::Sync => sync_func(),
        might_be_async::Async => async_func().await
    ]
}

#[might_be_async::func(tokio::main)]
fn your_main() {

}

#[might_be_async::func(tokio::test)]
fn your_test() {

}
```

```toml
[package.metadata.might_be_async]
control.feature_name = "async"
```

若没有，就默认"async"

2. 显式

```rust
#[might_be_async::func("async")]
fn your_function(who_care: SomeFunc) -> ReturnValue {
    let r: ReturnValue = might_be_async::invoke!("async" => your_another_function(who_care));
}

#[might_be_async::func]
fn your_another_function(who_care: SomeFunc) -> ReturnValue {
    might_be_async::select!["async",
        Sync => sync_func(),
        Async => async_func().await
    ]
}

#[might_be_async::func("async", tokio::main)]
fn your_main() {

}

#[might_be_async::func("async", tokio::test)]
fn your_test() {

}
```

3. 更显式 （精细控制）

```rust
#[might_be_async::func("async" || "sync")]
fn your_function(who_care: SomeFunc) -> ReturnValue {
    let r: ReturnValue = might_be_async::invoke!("async" || "sync" => your_another_function(who_care));
}

#[might_be_async::func]
fn your_another_function(who_care: SomeFunc) -> ReturnValue {
    might_be_async::select!["async" || "sync",
        might_be_async::Sync => sync_func(),
        might_be_async::Async => async_func().await
    ]
}

#[might_be_async::func("async" || "sync", tokio::main || std::main)]
fn your_main() {

}

#[might_be_async::func("async" || "sync", tokio::test || std::test)]
fn your_test() {

}
```

关于 `select!`

展开后是

```rust
match {
    {
        #[cfg(feature = #feat_async)]
        {
            ::might_be_async::Async
        }
        #[cfg(feature = #feat_sync)]
        {
            ::might_be_async::Async
        }
    }
} {
    #expr
}
```

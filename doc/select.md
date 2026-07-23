Select between sync and async expressions based on a Cargo feature flag.

# Usage

## Explicit mode (with feature names)

```ignore
select!["async" => expr_async().await, "sync" => expr_sync()];
select!["async" => { expr_async().await }, "sync" => { expr_sync() }];
select!["async" => expr_async().await, !        => expr_sync()];
select![!        => expr_async().await, "sync" => expr_sync()];
```

## Implicit mode (auto-detect `.await`)

```ignore
select![expr_async().await, expr_sync()];
select![{ expr_async().await }, { expr_sync() }];
```

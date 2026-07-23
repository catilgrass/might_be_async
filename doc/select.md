Select between sync and async expressions based on a Cargo feature flag.

## Explicit mode

Provide feature names for each arm:

```rust
select!["async" => async_fn().await, "sync" => sync_fn()]
```

The `!` token negates a feature name:

```rust
select!["async" => async_fn().await, !        => sync_fn()]
```

## Implicit mode

Omit feature names; the macro auto-detects `.await`:

```rust
select![async_fn().await, sync_fn()]
```

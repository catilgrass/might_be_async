A single arm inside the `select!` macro.

| Variant    | Example           | Meaning                                               |
| ---------- | ----------------- | ----------------------------------------------------- |
| `Explicit` | `"async" => expr` | Arm guarded by `feature = "async"`                    |
| `Not`      | `! => expr`       | Arm guarded by `not(feature = <other_arm's_feature>)` |
| `Implicit` | `expr`            | Feature determined by `.await` detection              |

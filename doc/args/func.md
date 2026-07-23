Arguments parsed from the `#[func]` attribute.

If the attribute is empty (`#[func]`), defaults to feature name `"async"`.
When a string literal is provided (`#[func("tokio_rt")]`), that value is used.

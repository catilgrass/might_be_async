Procedural macro that wraps a call expression, adding `.await` when the async
feature is enabled, and leaving it as a plain call otherwise.

This macro is designed to be used **inside a `#[func]`-annotated function**.
`#[func]` ensures that the enclosing function is either sync or async depending
on the feature flag. `invoke!` mirrors that same gating on the call site:

- Feature disabled, enclosing fn is sync → `invoke!(callee(args))` expands to
  `callee(args)` — a plain synchronous call.
- Feature enabled, enclosing fn is async → `invoke!(callee(args))` expands to
  `callee(args).await` — an awaited asynchronous call.

Both the callee and the caller should be produced by `#[func]` so that their
sync/async nature changes in lockstep.

An explicit feature name can be provided with the `=>` syntax. This is useful
when different features control different parts of the pipeline.

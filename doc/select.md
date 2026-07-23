Procedural macro that chooses between two expressions based on whether the
async feature is enabled, producing a value.

## Explicit mode

Each arm is labelled with a feature name. When the named feature is enabled
that arm is selected; otherwise the other arm is used. Since only one feature
is ever active at a time, the arm labelled `"sync"` effectively acts as an else
branch.

The `!` token inverts the sense — it selects the arm when the corresponding
feature is **not** enabled.

## Implicit mode

Feature names are omitted; the macro inspects the token stream to decide which
expression is async. Whichever arm contains a `.await` call is treated as the
async branch.

When neither arm contains `.await`, the first expression is assigned to the
async branch and the second to sync. The two branches trade places during
expansion so the correct one is active in each mode.

#[func]
fn compute(x: i32) -> i32 {
    x * 2
}

#[func]
fn example() -> i32 {
    // invoke! macro should be used with #[func]
    invoke!(compute(5))
}

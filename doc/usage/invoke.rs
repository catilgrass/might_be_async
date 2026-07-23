#[func]
fn compute(x: i32) -> i32 {
    x * 2
}

#[func]
fn example() -> i32 {
    invoke!(compute(5))
}

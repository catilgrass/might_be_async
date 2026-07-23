fn compute(x: i32) -> i32 {
    x * 2
}
fn example() -> i32 {
    { { compute(5) } }
}

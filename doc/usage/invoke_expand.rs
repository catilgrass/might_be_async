async fn compute(x: i32) -> i32 {
    x * 2
}
async fn example() -> i32 {
    { { compute(5) } }
}

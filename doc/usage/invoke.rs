use might_be_async::invoke;

async fn compute(x: i32) -> i32 {
    x * 2
}

async fn example() -> i32 {
    invoke!(compute(5))
}

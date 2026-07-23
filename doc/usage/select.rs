use might_be_async::select;

fn example() -> i32 {
    select!["async" => 100, "sync" => 200]
}

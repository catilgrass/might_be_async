fn example() {
    select! {
        "async" => { 100 }
        else
        ! => { 200 }
    };
}

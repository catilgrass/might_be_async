use might_be_async::func;

#[func]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

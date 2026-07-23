use might_be_async::func;
pub fn greet(name: &str) -> String {
    ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("Hello, {0}!", name))
    })
}

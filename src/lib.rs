use proc_macro::TokenStream;

mod args;
pub(crate) mod func;
pub(crate) mod invoke;

#[proc_macro_attribute]
pub fn func(attr: TokenStream, item: TokenStream) -> TokenStream {
    func::func(attr, item)
}

#[proc_macro]
pub fn invoke(input: TokenStream) -> TokenStream {
    invoke::invoke(input)
}

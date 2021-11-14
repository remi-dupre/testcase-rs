mod derive;
mod hackercup;

// Note that modules actually use proc_macro2, conversion will be performed here.
use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

#[proc_macro_derive(TestCase, attributes(testcase))]
pub fn testcase_derive(input: TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).expect("Invalid token stream for TestCase");
    derive::impl_testcase_macro(&ast).into()
}

#[proc_macro_attribute]
pub fn hackercup(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);
    hackercup::generate(args, input).into()
}

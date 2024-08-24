// SPDX-License-Identifier: GPL-3.0

mod fib;

use proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn some_name(input: TokenStream) -> TokenStream {
    TokenStream::from(input)
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    TokenStream::from(input)
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#![recursion_limit = "256"]
extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;

mod active_query;
mod active_record;
mod command;
mod mysql;
mod query;
mod query_builder;

extern crate r2d2_mysql;

use std::io::ErrorKind;

type ARResult<T> = Result<T, ErrorKind>;


trait F{
    fn a(){
        println!("hello world");
    }
}

#[proc_macro_derive(ActiveRecord)]
pub fn active_record(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let expanded = quote! {
        impl  #name {
            fn a() -> Option<i32> {
                println!("hello world");
                Some(1)
            }
        }
    };
    TokenStream::from(expanded)
}
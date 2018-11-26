#![feature(extern_crate_item_prelude)]
extern crate proc_macro;
extern crate syn;
extern crate scraper;
#[macro_use]
extern crate quote;
extern crate unhtml;

#[cfg(test)]
mod test;
mod implement;

use proc_macro::TokenStream;

#[proc_macro_derive(UnHtml)]
pub fn un_html_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();
    implement::impl_un_html(&ast).parse().unwrap()
}
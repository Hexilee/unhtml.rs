#![feature(extern_crate_item_prelude)]
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
extern crate scraper;
#[macro_use]
extern crate quote;
extern crate unhtml;

#[cfg(test)]
mod test;
mod implement;
use syn::DeriveInput;

#[proc_macro_derive(UnHtml)]
pub fn un_html_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(implement::impl_un_html(&input))
}
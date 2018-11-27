#![feature(extern_crate_item_prelude)]
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
extern crate scraper;
#[macro_use]
extern crate quote;

extern crate cssparser;
extern crate selectors;

#[cfg(test)]
mod test;
mod implement;
use syn::ItemStruct;

#[proc_macro_attribute]
pub fn unhtml(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    proc_macro::TokenStream::from(implement::impl_un_html(&input))
}
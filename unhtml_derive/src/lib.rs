#![feature(extern_crate_item_prelude)]
extern crate proc_macro;
extern crate syn;
extern crate scraper;
#[macro_use]
extern crate quote;

#[cfg(test)]
mod test;
pub mod derives;
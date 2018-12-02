extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
extern crate scraper;
#[macro_use]
extern crate synstructure;
#[macro_use]
extern crate quote;
extern crate unhtml;
mod implement;

decl_derive!([FromHtml, attributes(html)] => unhtml_derive);

fn unhtml_derive(input: synstructure::Structure) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(implement::impl_un_html(&input))
}

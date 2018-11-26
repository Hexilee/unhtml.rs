use scraper::Selector;
use proc_macro::TokenStream;

#[proc_macro_derive(UnHtml)]
pub fn un_html_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();
    impl_un_html(&ast).parse().unwrap()
}

fn impl_un_html(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl UnHtml for #name {
            fn from_str(data: &str) -> Result<Box<Self>, cssparser::ParseError<SelectorParseErrorKind>> {
                Ok(Box::new(#name{})
            }
        }
    }
}
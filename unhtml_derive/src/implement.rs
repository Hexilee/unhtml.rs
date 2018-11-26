use scraper::Selector;
use unhtml::traits::UnHtml;

pub fn impl_un_html(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl UnHtml for # name {
            fn from_str(data: &str) -> Result<Box<Self>, cssparser::ParseError<SelectorParseErrorKind>> {
                Ok(Box::new(#name{}))
            }
        }
    }
}
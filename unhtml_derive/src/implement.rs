use scraper::Selector;
use syn::spanned::Spanned;
use syn::Data;
use syn::Fields;
use proc_macro2::TokenStream;

pub fn impl_un_html(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let result_recurse = match ast.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    fields.named.iter().map(|field| {
                        let name = &field.ident;
                        quote_spanned! { field.span() =>
                            #name: "Hello, World"
                        }
                    })
                }
                Fields::Unnamed(_) | Fields::Unit => unreachable!(),
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };
    quote! {
        impl UnHtml for #struct_name {
            fn from_str(data: &str) -> Result<Self, cssparser::ParseError<SelectorParseErrorKind>> {
                Ok(#struct_name{#(#result_recurse), *})
            }
        }
    }
}
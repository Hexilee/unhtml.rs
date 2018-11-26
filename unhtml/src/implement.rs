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
//                        if let Some(ref html_attr) = field.attrs.iter().find(|attr| attr.)
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
        #ast
        impl std::str::FromStr for #struct_name {
            type Err = Box<std::error::Error>;
            fn from_str(data: &str) -> Result<Self, Self::Err> {
                Ok(#struct_name{#(#result_recurse), *})
            }
        }
    }
}
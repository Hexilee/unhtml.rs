use crate::parse::try_parse;
use crate::Result;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub fn derive(input: proc_macro::TokenStream) -> Result<TokenStream> {
    let target = try_parse::<ItemStruct>(input)?;
    let (impl_generics, ty_generics, where_clause) = target.generics.split_for_impl();
    let struct_name = target.ident.clone();
    Ok(quote!(
        impl #impl_generics unhtml::FromText for #struct_name #ty_generics #where_clause {
            fn from_inner_text(select: unhtml::ElemIter) -> unhtml::Result<Self> {
                let first = select.next().ok_or(unhtml::HtmlError::SourceEmpty)?;
                let mut ret = String::new();
                for next_segment in first.text() {
                    ret += next_segment.trim();
                }
                Ok(ret.parse()?)
            }
            fn from_attr(select: unhtml::ElemIter, attr: &str) -> unhtml::Result<Self> {
                let first = select.next().ok_or(unhtml::HtmlError::SourceEmpty)?;
                let attr = first.value().attr(attr).ok_or(unhtml::HtmlError::SourceNotFound {
                    source_type: "attr".into(),
                    source_name: attr.into(),
                })?;
                Ok(attr.trim().parse()?)
            }
        }
    ))
}

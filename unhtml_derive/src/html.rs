use super::attr_meta::AttrMeta;
use super::parse::try_parse;
use crate::Result;
use proc_macro2::TokenStream;
use quote::quote;
use std::convert::TryInto;
use syn::{Attribute, Fields, ItemStruct};

const ATTR_INNER_TEXT: &str = "inner";

macro_rules! use_idents {
    ($($idents:ident),*) => {
        $(let $idents = quote!($idents);)*
    }
}

fn import() -> TokenStream {
    quote!(
        use unhtml::{scraper::Selector, Element, Text, Select};
    )
}

// TODO: confirm no lifetime in generics
pub fn derive(input: proc_macro::TokenStream) -> Result<TokenStream> {
    use_idents!(_select);
    let target = try_parse::<ItemStruct>(input)?;
    let (impl_generics, ty_generics, where_clause) = target.generics.split_for_impl();
    let struct_name = target.ident.clone();
    let attr_meta: AttrMeta = target.attrs.try_into()?;
    let import_statement = import();
    let define_elements_statement = define_elements(attr_meta.selector.as_ref());
    let struct_field_values = gen_struct_field_values(&target.fields)?;
    let struct_value = match &target.fields {
        Fields::Named(_) => quote!(Self{#struct_field_values}),
        Fields::Unnamed(_) => quote!(Self(#struct_field_values)),
        Fields::Unit => quote!(Self),
    };
    Ok(quote!(
        impl #impl_generics unhtml::FromHtml for #struct_name #ty_generics #where_clause {
            fn from_elements(#_select: unhtml::ElemIter) -> unhtml::Result<Self> {
                #import_statement
                #define_elements_statement
                Ok(#struct_value)
            }
        }
    ))
}

fn define_elements(selector: Option<&String>) -> TokenStream {
    use_idents!(_select, _elements);
    let current_select = match selector {
        Some(selector) => quote!(#_select.select_elements(&Selector::parse(#selector).unwrap())),
        None => quote!(#_select),
    };
    quote!(let #_elements: Vec<_> = #current_select.collect();)
}

fn gen_struct_field_values(fields: &Fields) -> Result<TokenStream> {
    let mut field_pairs = quote!();
    for field in fields {
        let value = gen_field_value(field.attrs.clone())?;
        let next_field = match field.ident.as_ref() {
            Some(ident) => quote!(#ident: #value),
            None => quote!(#value),
        };
        field_pairs = quote!(#field_pairs #next_field, );
    }
    Ok(field_pairs)
}

fn gen_field_value(attr: Vec<Attribute>) -> Result<TokenStream> {
    use_idents!(_elements);
    let meta: AttrMeta = attr.try_into()?;
    let current_select = quote!(#_elements.clone().into_iter());
    let new_select = match meta.selector.as_ref() {
        Some(selector) => quote!(
            #current_select.select_elements(&Selector::parse(#selector).unwrap())
        ),
        None => quote!(#current_select),
    };

    let result = match meta.attr.as_ref() {
        Some(attr) if attr == ATTR_INNER_TEXT => quote!(#new_select.inner_text()),
        Some(attr) => quote!(#new_select.attr(#attr)),
        None => quote!(#new_select.element()),
    };
    Ok(match meta.default {
        true => quote!(
            #result.unwrap_or(Default::default())
        ),
        false => quote!(#result?),
    })
}

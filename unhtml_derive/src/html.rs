use super::attr_meta::AttrMeta;
use super::parse::try_parse;
use proc_macro::{Diagnostic, Level};
use proc_macro2::TokenStream;
use quote::quote;
use std::convert::TryInto;
use syn::{ItemStruct, Lit};

const ATTR_INNER_TEXT: &str = "inner";

pub macro use_idents {
    ($($idents:ident),*) => {
        $(let $idents = quote!($idents);)*
    }
}

// TODO: confirm no lifetime in generics
pub fn derive(input: proc_macro::TokenStream) -> Result<TokenStream, Diagnostic> {
    let target = try_parse::<ItemStruct>(input)?;
    let (impl_generics, ty_generics, where_clause) = target.generics.split_for_impl();
    let struct_name = target.ident.clone();
    use_idents!(_data, _root_element_ref);
    let attr_meta: AttrMeta = target.attrs.try_into()?;
    //    let root_element_ref_define_block = match top_macro_attr.selector {
    //        Some(selector) => {
    //            check_selector(&selector);
    //            quote!(let #root_element_ref_ident = #data_ident.select(&unhtml::scraper::Selector::parse(#selector).unwrap()).next().ok_or(
    //                unhtml::DeserializeError::SourceNotFound { attr: "selector".to_string(), value: #selector.to_string(), html_fragment: #data_ident.html()}
    //            )?;)
    //        }
    //        None => quote!(
    //            let #root_element_ref_ident = data;
    //        ),
    //    };
    //    let result_recurse = match ast.data {
    //        syn::Data::Struct(ref data_struct) => data_struct
    //            .fields
    //            .iter()
    //            .map(get_field_token_stream(root_element_ref_ident.clone())),
    //        syn::Data::Enum(_) | syn::Data::Union(_) => unreachable!(),
    //    };
    Ok(quote!(
    //        impl #impl_generics unhtml::FromHtml for #struct_name #ty_generics #where_clause {
    //            fn from_html_ref(#data_ident: unhtml::scraper::ElementRef) -> Result<Self, unhtml::failure::Error> {
    //                #root_element_ref_define_block
    //                Ok(#struct_name{#(#result_recurse),*})
    //                unimplemented!()
    //            }
    //        }
        ))
}

//fn get_field_token_stream(
//    root_element_ref_ident: TokenStream,
//) -> impl Fn(&syn::Field) -> TokenStream {
//    move |field: &syn::Field| {
//        let name = &field.ident;
//        let macro_attr = get_macro_attr(&field.attrs);
//        let type_path = if let syn::Type::Path(ref path) = field.ty {
//            path
//        } else {
//            panic!("unsupported field type: {:?}", &field.ty);
//        };
//        let path_segment = type_path.path.segments.first().unwrap();
//        let type_ident = &path_segment.value().ident;
//        let type_arguments = &path_segment.value().arguments;
//        let default_value = macro_attr.default.clone();
//        let match_block_token_stream = get_match_block_token_stream(
//            type_ident,
//            get_result_token_stream(
//                &root_element_ref_ident,
//                macro_attr,
//                type_ident,
//                type_arguments,
//            ),
//            default_value,
//        );
//        quote!(#name: #match_block_token_stream)
//    }
//}

//fn get_match_block_token_stream(
//    type_ident: &syn::Ident,
//    result_token_stream: TokenStream,
//    default: Option<Lit>,
//) -> TokenStream {
//    match default {
//        Some(lit) => {
//            let lit_token_stream = if let Lit::Str(_) = lit {
//                quote!(#type_ident::from_html(#lit)?)
//            } else {
//                quote!(#type_ident::from(#lit))
//            };
//            quote!(
//                match #result_token_stream {
//                    Ok(final_result) => final_result,
//                    Err(_) => #lit_token_stream
//                }
//            )
//        }
//        None => quote!(#result_token_stream?),
//    }
//}

//fn get_result_token_stream(
//    root_element_ref_ident: &TokenStream,
//    macro_attr: MacroAttr,
//    type_ident: &syn::Ident,
//    type_arguments: &syn::PathArguments,
//) -> TokenStream {
//    match macro_attr.selector {
//        Some(selector_lit) => {
//            check_selector(&selector_lit);
//            match macro_attr.attr {
//                Some(attr_lit) => {
//                    let attr_value = get_lit_str_value(&attr_lit);
//                    if &attr_value == ATTR_INNER_TEXT {
//                        match get_vec_elem_type(type_ident, type_arguments) {
//                            Some(ty) => quote!(#type_ident::<#ty>::from_inner_text(#selector_lit, #root_element_ref_ident.clone())),
//                            None => quote!(#type_ident::from_selector_and_inner_text(#selector_lit, #root_element_ref_ident.clone())),
//                        }
//                    } else {
//                        match get_vec_elem_type(type_ident, type_arguments) {
//                            Some(ty) => quote!(#type_ident::<#ty>::from_attr(#selector_lit, #attr_lit, #root_element_ref_ident.clone())),
//                            None => quote!(#type_ident::from_selector_and_attr(#selector_lit, #attr_lit, #root_element_ref_ident.clone())),
//                        }
//                    }
//                }
//                None => match get_vec_elem_type(type_ident, type_arguments) {
//                    Some(ty) => quote!(#type_ident::<#ty>::from_html_ref(#selector_lit, #root_element_ref_ident.clone())),
//                    None => quote!(#type_ident::from_selector_and_html(#selector_lit, #root_element_ref_ident.clone())),
//                },
//            }
//        }
//        None => match get_vec_elem_type(type_ident, type_arguments) {
//            Some(_) => panic!("vec field must has selector!"),
//            None => match macro_attr.attr {
//                Some(attr_lit) => {
//                    let attr_value = get_lit_str_value(&attr_lit);
//                    if &attr_value == ATTR_INNER_TEXT {
//                        quote!(#type_ident::from_inner_text(#root_element_ref_ident.clone()))
//                    } else {
//                        quote!(#type_ident::from_attr(#attr_lit, #root_element_ref_ident.clone()))
//                    }
//                }
//                None => quote!(#type_ident::from_html_ref(#root_element_ref_ident.clone())),
//            },
//        },
//    }
//}

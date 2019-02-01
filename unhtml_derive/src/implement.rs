use syn::{Lit, Attribute};
use proc_macro2::TokenStream;
use unhtml::scraper::Selector;

const TYPE_VEC: &str = "Vec";
const HTML_IDENT: &str = "html";
const SELECTOR_IDENT: &str = "selector";
const ATTR_IDENT: &str = "attr";
const DEFAULT_IDENT: &str = "default";
const ATTR_INNER_TEXT: &str = "inner";

pub fn impl_un_html(structure: &synstructure::Structure) -> TokenStream {
    let ast = structure.ast();
    let struct_name = &ast.ident;
    let data_ident = quote!(data);
    let root_element_ref_ident = quote!(root_element_ref);
    let top_macro_attr = get_macro_attr(&ast.attrs);
    let root_element_ref_define_block = match top_macro_attr.selector {
        Some(selector) => {
            check_selector(&selector);
            quote!(let #root_element_ref_ident = #data_ident.select(&unhtml::scraper::Selector::parse(#selector).unwrap()).next().ok_or(
                unhtml::DeserializeError::SourceNotFound { attr: "selector".to_string(), value: #selector.to_string() }
            )?;)
        }
        None => quote!(
//            let mut optional_root_element_ref = None;
//            for child in #data_ident.children() {
//                if child.value().is_element() {
//                    optional_root_element_ref = Some(unhtml::scraper::ElementRef::wrap(child).unwrap());
//                }
//            };
            let #root_element_ref_ident = data;
        )
    };
    let result_recurse = match ast.data {
        syn::Data::Struct(ref data_struct) => data_struct.fields.iter().map(get_field_token_stream(root_element_ref_ident.clone())),
        syn::Data::Enum(_) | syn::Data::Union(_) => unreachable!(),
    };
    quote!(
        impl unhtml::FromHtml for #struct_name {
            fn from_html_ref(#data_ident: unhtml::scraper::ElementRef) -> Result<Self, unhtml::failure::Error> {
                #root_element_ref_define_block
                Ok(#struct_name{#(#result_recurse),*})
            }
        }
    )
}

fn get_field_token_stream(root_element_ref_ident: TokenStream) -> impl Fn(&syn::Field) -> TokenStream {
    move |field: &syn::Field| {
        let name = &field.ident;
        let macro_attr = get_macro_attr(&field.attrs);
        let type_path = if let syn::Type::Path(ref path) = field.ty {
            path
        } else {
            panic!("unsupported field type: {:?}", &field.ty);
        };
        let path_segment = type_path.path.segments.first().unwrap();
        let type_ident = &path_segment.value().ident;
        let type_arguments = &path_segment.value().arguments;
        let default_value = macro_attr.default.clone();
        let match_block_token_stream = get_match_block_token_stream(
            type_ident,
            get_result_token_stream(&root_element_ref_ident, macro_attr, type_ident, type_arguments),
            default_value,
        );
        quote!(#name: #match_block_token_stream)
    }
}

fn get_match_block_token_stream(type_ident: &syn::Ident, result_token_stream: TokenStream, default: Option<Lit>) -> TokenStream {
    match default {
        Some(lit) => {
            let lit_token_stream = if let Lit::Str(_) = lit {
                quote!(#type_ident::from_html(#lit)?)
            } else {
                quote!(#type_ident::from(#lit))
            };
            quote!(
                match # result_token_stream {
                    Ok(final_result) => final_result,
                    Err(_) => #lit_token_stream
                }
            )
        }
        None => quote!(#result_token_stream?)
    }
}

fn get_result_token_stream(root_element_ref_ident: &TokenStream,
                           macro_attr: MacroAttr, type_ident: &syn::Ident, type_arguments: &syn::PathArguments) -> TokenStream {
    check_type_arguments(type_ident, type_arguments);
    match macro_attr.selector {
        Some(selector_lit) => {
            check_selector(&selector_lit);
            match macro_attr.attr {
                Some(attr_lit) => {
                    let attr_value = get_lit_str_value(&attr_lit);
                    if &attr_value == ATTR_INNER_TEXT {
                        match get_vec_elem_type(type_ident, type_arguments) {
                            Some(ty) => quote!(#type_ident::<#ty>::from_inner_text(#selector_lit, #root_element_ref_ident.clone())),
                            None => quote!(#type_ident::from_selector_and_inner_text(#selector_lit, #root_element_ref_ident.clone()))
                        }
                    } else {
                        match get_vec_elem_type(type_ident, type_arguments) {
                            Some(ty) => quote!(#type_ident::<#ty>::from_attr(#selector_lit, #attr_lit, #root_element_ref_ident.clone())),
                            None => quote!(#type_ident::from_selector_and_attr(#selector_lit, #attr_lit, #root_element_ref_ident.clone()))
                        }
                    }
                }
                None => {
                    match get_vec_elem_type(type_ident, type_arguments) {
                        Some(ty) => quote!(#type_ident::<#ty>::from_html_ref(#selector_lit, #root_element_ref_ident.clone())),
                        None => quote!(#type_ident::from_selector_and_html(#selector_lit, #root_element_ref_ident.clone()))
                    }
                }
            }
        }
        None => {
            match get_vec_elem_type(type_ident, type_arguments) {
                Some(_) => panic!("vec field must has selector!"),
                None => match macro_attr.attr {
                    Some(attr_lit) => {
                        let attr_value = get_lit_str_value(&attr_lit);
                        if &attr_value == ATTR_INNER_TEXT {
                            quote!(#type_ident::from_inner_text(#root_element_ref_ident.clone()))
                        } else {
                            quote!(#type_ident::from_attr(#attr_lit, #root_element_ref_ident.clone()))
                        }
                    }
                    None => {
                        quote!(#type_ident::from_html_ref(#root_element_ref_ident.clone()))
                    }
                }
            }
        }
    }
}

fn check_type_arguments(type_ident: &syn::Ident, type_arguments: &syn::PathArguments) {
    if get_vec_elem_type(type_ident, type_arguments) == None && !type_arguments.is_empty() {
        panic!("field cannot be generic except for Vec<T>");
    }
}

fn get_vec_elem_type<'a>(type_ident: &syn::Ident, type_arguments: &'a syn::PathArguments) -> Option<&'a syn::Type> {
    if let &syn::PathArguments::AngleBracketed(ref angle_bracket) = type_arguments {
        if type_ident == TYPE_VEC && !type_arguments.is_empty() {
            if let syn::GenericArgument::Type(ty) = angle_bracket.args.first()?.value() {
                return Some(ty);
            }
        }
    }
    None
}

#[derive(Debug)]
struct MacroAttr {
    selector: Option<Lit>,
    attr: Option<Lit>,
    default: Option<Lit>,
}

fn get_macro_attr(attrs: &Vec<Attribute>) -> MacroAttr {
    let mut macro_attr = MacroAttr { selector: None, attr: None, default: None };
    for attr in attrs {
        if let Ok(meta) = attr.parse_meta() {
            if meta.name() == HTML_IDENT {
                if let syn::Meta::List(ref list) = meta {
                    for ref pair in list.nested.pairs() {
                        if let &&syn::NestedMeta::Meta(syn::Meta::NameValue(ref name_value)) = pair.value() {
                            if name_value.ident == SELECTOR_IDENT {
                                macro_attr.selector = Some(name_value.lit.to_owned());
                            } else if name_value.ident == ATTR_IDENT {
                                macro_attr.attr = Some(name_value.lit.to_owned());
                            } else if name_value.ident == DEFAULT_IDENT {
                                macro_attr.default = Some(name_value.lit.to_owned());
                            }
                        }
                    }
                }
            }
        }
    }
    macro_attr
}

fn check_selector(lit: &Lit) {
    if let &Lit::Str(ref str_lit) = lit {
        Selector::parse(&str_lit.value()).unwrap();
    } else {
        panic!("selector must be string")
    }
}

fn get_lit_str_value(lit: &Lit) -> String {
    if let &Lit::Str(ref str_lit) = lit {
        str_lit.value()
    } else {
        panic!("strlit must be string")
    }
}
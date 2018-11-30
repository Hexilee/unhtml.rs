use syn::{Fields, Lit, Attribute, ItemStruct};
use proc_macro2::TokenStream;
use scraper::Selector;
use unhtml_util::{HTML_IDENT, SELECTOR_IDENT, ATTR_IDENT, DEFAULT_IDENT};

const TYPE_STRING: &str = "String";
const TYPE_VEC: &str = "Vec";

pub fn impl_un_html(ast: &ItemStruct) -> TokenStream {
    let struct_name = &ast.ident;
    let data_ident = quote!(data);
    let root_element_ref_ident = quote!(root_element_ref);
    let top_macro_attr = get_macro_attr(&ast.attrs);
    let doc = quote!(Html::parse_fragment(#data_ident));
    let root_element_ref_define_block = match top_macro_attr.selector {
        Some(selector) => {
            check_selector(&selector);
            quote!(let #root_element_ref_ident = #doc.select(&Selector::parse(#selector).unwrap()).next().ok_or(
                ParseError::SelectOrAttrEmptyErr { attr: "selector".to_string(), value: #selector.to_string() }
            )?;)
        }
        None => quote!(let #root_element_ref_ident = #doc.root_element_ref();)
    };
    let result_recurse = match ast.fields {
        Fields::Named(ref fields) => fields.named.iter().map(get_field_token_stream(root_element_ref_ident.clone())),
        Fields::Unnamed(_) | Fields::Unit => unreachable!(),
    };
    quote!(
        #ast
        impl FromStr for #struct_name {
            type Err = failure::Error;
            fn from_str(#data_ident: &str) -> Result<Self, Self::Err> {
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
            let lit_token_stream = if type_ident == TYPE_STRING {
                // String::from_str() return String instead of Result<String, Self::Err>
                quote!(#type_ident::from(#lit))
            } else if let Lit::Str(_) = lit {
                quote!(#type_ident::from_str(lit)?)
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
    quote!(Err(()))
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
    }
}
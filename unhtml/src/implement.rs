use syn::{Fields, Lit, Attribute, ItemStruct};
use proc_macro2::TokenStream;
use scraper::Selector;
use unhtml_util::{HTML_IDENT, SELECTOR_IDENT, ATTR_IDENT, DEFAULT_IDENT};

pub fn impl_un_html(ast: &ItemStruct) -> TokenStream {
    let struct_name = &ast.ident;
    let data_ident = quote!(data);
    let select_ident = quote!(select);
    let top_macro_attr = get_macro_attr(&ast.attrs);
    println!("{:?}", &top_macro_attr);
    let doc = quote!(Html::parse_fragment(#data_ident));
    let select_define_block = match top_macro_attr.selector {
        Some(selector) => {
            check_selector(&selector);
            quote!(let #select_ident = #doc.select(&Selector::parse(#selector).unwrap()).next().ok_or(
                ParseError::SelectOrAttrEmptyErr { attr: "selector".to_string(), value: #selector.to_string() }
            )?;)
        },
        None => quote!(let #select_ident = #doc;)
    };
    let result_recurse = match ast.fields {
        Fields::Named(ref fields) => fields.named.iter().map(|field| -> TokenStream {
            let name = &field.ident;
            let macro_attr = get_macro_attr(&field.attrs);
            quote! {#name: "Hello, World"}
        }),
        Fields::Unnamed(_) | Fields::Unit => unreachable!(),
    };
    quote! {
        #ast
        impl std::str::FromStr for #struct_name {
            type Err = ParseError;
            fn from_str(#data_ident: &str) -> Result<Self, Self::Err> {
                #select_define_block
                Ok(#struct_name{#(#result_recurse),*})
            }
        }
    }
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
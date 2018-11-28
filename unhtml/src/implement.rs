use syn::{Fields, Lit, Attribute};
use proc_macro2::TokenStream;
use unhtml_util::{HTML_IDENT, SELECTOR_IDENT, ATTR_IDENT, DEFAULT_IDENT};

pub fn impl_un_html(ast: &syn::ItemStruct) -> TokenStream {
    let struct_name = &ast.ident;
    let result_recurse = match ast.fields {
        Fields::Named(ref fields) => {
            fields.named.iter().map(|field| -> TokenStream {
                let name = &field.ident;
                let macro_attr = get_macro_attr(&field.attrs);
                println!("{:?}", &macro_attr);
                quote! {#name: "Hello, World"}
            })
        }
        Fields::Unnamed(_) | Fields::Unit => unreachable!(),
    };

    quote! {
        #ast
        impl std::str::FromStr for #struct_name {
            type Err = ParseError;
            fn from_str(data: &str) -> Result<Self, Self::Err> {
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
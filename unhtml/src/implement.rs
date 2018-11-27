use scraper::Selector;
use syn::spanned::Spanned;
use syn::Fields;
use syn::AttrStyle;
use syn::Attribute;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use syn::ItemStruct;
use std::str::FromStr;
use proc_macro2::Literal;
use unhtml_util::*;

pub fn impl_un_html(ast: &syn::ItemStruct) -> TokenStream {
//    let a = scraper::Html::parse_document("").select(&scraper::Selector::parse("a").unwrap()).next().unwrap().inner_html();
    let struct_name = &ast.ident;
    let result_recurse = match ast.fields {
        Fields::Named(ref fields) => {
            fields.named.iter().map(|field| -> TokenStream {
//                println!("{:?}", field.ty);
                let name = &field.ident;
                let macro_attr = get_macro_attr(&field.attrs);
                println!("{:?}", &macro_attr);
                quote_spanned! { field.span() =>
                            #name: "Hello, World"
                        }
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
    selector: TokenTree,
    attr: TokenTree,
    default: Option<TokenTree>,
}

fn get_macro_attr(attrs: &Vec<Attribute>) -> MacroAttr {
    let mut macro_attr = MacroAttr { selector: TokenTree::Literal(Literal::string(ROOT_SELECTOR)), attr: TokenTree::Literal(Literal::string(ATTR_INNER_HTML)), default: None };
    if let Some(ref html_attr) = attrs.iter().find(|attr| attr.style == AttrStyle::Outer && attr.path.segments.first().unwrap().value().ident.to_string() == HTML_IDENT) {
        if let Some(ref token_tree) = html_attr.tts.to_owned().into_iter().find(|token_tree| if let TokenTree::Group(_) = *token_tree { true } else { false }) {
            if let TokenTree::Group(ref group) = *token_tree {
                let mut iter = group.stream().to_owned().into_iter();
                let iter_ref = &mut iter;
                while let Some(ref token_tree) = iter_ref.next() {
                    if let TokenTree::Ident(ref ident) = *token_tree {
                        if ident.eq(SELECTOR_IDENT) {
                            if let Some(TokenTree::Punct(ref punct)) = iter_ref.next() {
                                if punct.as_char() == EQUAL_PUNCT {
                                    if let Some(lit) = iter_ref.next() {
                                        macro_attr.selector = lit;
                                    }
                                }
                            }
                            continue;
                        }

                        if ident.eq(ATTR_IDENT) {
                            if let Some(TokenTree::Punct(ref punct)) = iter_ref.next() {
                                if punct.as_char() == EQUAL_PUNCT {
                                    if let Some(lit) = iter_ref.next() {
                                        macro_attr.attr = lit;
                                    }
                                }
                            }
                            continue;
                        }

                        if ident.eq(DEFAULT_IDENT) {
                            if let Some(TokenTree::Punct(ref punct)) = iter_ref.next() {
                                if punct.as_char() == EQUAL_PUNCT {
                                    macro_attr.default = iter_ref.next();
                                }
                            }
                            continue;
                        }
                    }
                }
            }
        }
    }
    macro_attr
}
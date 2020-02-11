use proc_macro2::Span;
use scraper::Selector;
use std::convert::TryFrom;
use std::fmt::Debug;
use syn::{parse, punctuated::Punctuated, Attribute, Error, Expr, Ident, LitStr, Result, Token};

const HTML_ATTR: &str = "html";
const SELECTOR_ATTR: &str = "selector";
const ATTR_ATTR: &str = "attr";
const DEFAULT_ATTR: &str = "default";

#[derive(Debug, Eq, PartialEq)]
pub enum DefaultAttr {
    None,
    DefaultImpl,
    Value(syn::Expr),
}

#[derive(Debug, Eq, PartialEq)]
pub struct AttrMeta {
    pub selector: Option<String>,
    pub attr: Option<String>,
    pub default: DefaultAttr,
}

impl Default for AttrMeta {
    fn default() -> Self {
        Self {
            selector: None,
            attr: None,
            default: DefaultAttr::None,
        }
    }
}

impl parse::Parse for AttrMeta {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let attrs: Punctuated<Attr, Token![,]> = input.parse_terminated(|input| input.parse())?;

        let mut meta = AttrMeta::default();
        for attr in attrs {
            match attr {
                Attr::Selector(lit_str) => meta.selector = Some(lit_str.value()),
                Attr::Attr(lit_str) => meta.attr = Some(lit_str.value()),
                Attr::Default(def) => meta.default = def,
            }
        }
        Ok(meta)
    }
}

#[derive(Debug)]
enum Attr {
    Selector(LitStr),
    Attr(LitStr),
    Default(DefaultAttr),
}

impl parse::Parse for Attr {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        match &*name.to_string() {
            // default = ...
            DEFAULT_ATTR if input.peek(Token![=]) => {
                let _: Token![=] = input.parse()?;
                let expr: Expr = input.parse()?;

                Ok(Attr::Default(DefaultAttr::Value(expr)))
            }
            // default
            DEFAULT_ATTR => Ok(Attr::Default(DefaultAttr::DefaultImpl)),
            // attr = "..."
            ATTR_ATTR if input.peek(Token![=]) && input.peek2(LitStr) => {
                let _: Token![=] = input.parse()?;
                let lit_str: LitStr = input.parse()?;

                Ok(Attr::Attr(lit_str))
            }
            ATTR_ATTR if input.peek(Token![=]) => {
                let _: Token![=] = input.parse()?;
                Err(input.error("expected string literal"))
            }
            // selector = "..."
            SELECTOR_ATTR if input.peek(Token![=]) && input.peek2(LitStr) => {
                let _: Token![=] = input.parse()?;
                let lit_str: LitStr = input.parse()?;
                check_selector(&lit_str.value())?;
                Ok(Attr::Selector(lit_str))
            }
            ATTR_ATTR | SELECTOR_ATTR if input.peek(Token![=]) => {
                let _: Token![=] = input.parse()?;
                Err(input.error("expected string literal"))
            }
            ATTR_ATTR | SELECTOR_ATTR => Err(input.error(format!(
                "missing '=', expected to find '{} = \"...\"'",
                name.to_string()
            ))),
            name => Err(input.error(format!("invalid `html` attribute: {}", name))),
        }
    }
}

impl TryFrom<Vec<Attribute>> for AttrMeta {
    type Error = Error;
    fn try_from(attrs: Vec<Attribute>) -> Result<Self> {
        let mut html_attrs = attrs
            .into_iter()
            .filter(|attr| attr.path.is_ident(HTML_ATTR));

        match (html_attrs.next(), html_attrs.next()) {
            (Some(ref only), None) if !only.tokens.is_empty() => only.parse_args(),
            // Either no attribute at all or attribute with empty contents
            (_, None) => Ok(Default::default()),
            (_, _) => Err(Error::new(
                Span::call_site(),
                "there cannot be multiple `html` attributes",
            )),
        }
    }
}

fn check_selector(selector: &str) -> Result<()> {
    Selector::parse(selector).map(|_| ()).map_err(|err| {
        Error::new(
            Span::call_site(),
            format!("invalid css selector `{}`: {:?}", selector, err),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{AttrMeta, DefaultAttr};
    use proc_macro2::TokenStream;
    use quote::quote;
    use std::convert::{TryFrom, TryInto};
    use syn::parse::{Parse, Parser};
    use syn::ItemStruct;

    pub fn parse<T: Parse>(token: TokenStream) -> T {
        <T as Parse>::parse.parse2(token).unwrap()
    }

    #[test]
    fn test_parse_meta_default() {
        assert_eq!(
            AttrMeta {
                selector: None,
                attr: None,
                default: DefaultAttr::None,
            },
            parse::<ItemStruct>(quote!(
                #[html]
                struct A;
            ))
            .attrs
            .try_into()
            .unwrap()
        );
    }

    #[test]
    fn test_parse_meta() {
        assert_eq!(
            AttrMeta {
                selector: Some("a".into()),
                attr: Some("href".into()),
                default: DefaultAttr::DefaultImpl,
            },
            parse::<ItemStruct>(quote!(
                #[html(selector = "a", attr = "href", default)]
                struct A;
            ))
            .attrs
            .try_into()
            .unwrap()
        );
    }

    #[test]
    fn test_parse_meta_default_fn_expr() {
        assert_eq!(
            AttrMeta {
                selector: Some("a".into()),
                attr: Some("href".into()),
                default: DefaultAttr::Value(syn::parse2(quote!(asdf())).unwrap()),
            },
            parse::<ItemStruct>(quote!(
                #[html(selector = "a", attr = "href", default = asdf())]
                struct A;
            ))
            .attrs
            .try_into()
            .unwrap()
        );
    }

    #[test]
    fn test_parse_meta_default_expr() {
        assert_eq!(
            AttrMeta {
                selector: Some("a".into()),
                attr: Some("href".into()),
                default: DefaultAttr::Value(syn::parse2(quote!(123)).unwrap()),
            },
            parse::<ItemStruct>(quote!(
                #[html(selector = "a", attr = "href", default = 123)]
                struct A;
            ))
            .attrs
            .try_into()
            .unwrap()
        );
    }

    #[test]
    fn test_parse_meta_invalid_selector() {
        let e = AttrMeta::try_from(
            parse::<ItemStruct>(quote!(
                #[html(selector = "->", attr = "href", default = 123)]
                struct A;
            ))
            .attrs,
        )
        .unwrap_err();

        assert!(e.to_string().contains("invalid css"));
    }

    #[test]
    fn test_parse_meta_bigger() {
        let meta = AttrMeta::try_from(
            parse::<ItemStruct>(quote!(
                struct A;
            ))
            .attrs,
        )
        .unwrap();

        assert_eq!(AttrMeta::default(), meta);
    }
}

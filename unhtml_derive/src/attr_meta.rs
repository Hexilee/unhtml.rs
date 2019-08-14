use crate::Result;
use proc_macro::{Diagnostic, Level};
use quote::quote;
use scraper::Selector;
use std::convert::TryFrom;
use syn::{Attribute, Lit, Meta, MetaList, NestedMeta};

const HTML_ATTR: &str = "html";
const SELECTOR_ATTR: &str = "selector";
const ATTR_ATTR: &str = "attr";
const DEFAULT_ATTR: &str = "default";

macro_rules! diagnostic_invalid_attribute {
    ($attr:expr) => {
        Diagnostic::new(Level::Error, format!("invalid `html` attribute: {}", $attr))
    };
}

#[derive(Debug, Eq, PartialEq)]
pub struct AttrMeta {
    pub selector: Option<String>,
    pub attr: Option<String>,
    pub default: bool,
}

impl Default for AttrMeta {
    fn default() -> Self {
        Self {
            selector: None,
            attr: None,
            default: false,
        }
    }
}

impl TryFrom<Vec<Attribute>> for AttrMeta {
    type Error = Diagnostic;
    fn try_from(attrs: Vec<Attribute>) -> Result<Self> {
        match filter_attrs(attrs)? {
            None | Some(Meta::Path(_)) => Ok(Default::default()),
            Some(Meta::NameValue(name_value)) => {
                Err(diagnostic_invalid_attribute!(quote!(#name_value)))
            }
            Some(Meta::List(list)) => Self::try_from_meta_list(list),
        }
    }
}

impl AttrMeta {
    fn try_from_meta_list(meta_list: MetaList) -> Result<Self> {
        let mut ret: AttrMeta = Default::default();
        for nested_meta in meta_list.nested.iter() {
            match nested_meta {
                NestedMeta::Lit(_) => {
                    return Err(diagnostic_invalid_attribute!(quote!(#meta_list)));
                }
                NestedMeta::Meta(inner_meta) => match inner_meta {
                    Meta::Path(path) if path.is_ident(DEFAULT_ATTR) => {
                        ret.default = true;
                    }
                    Meta::NameValue(named_value) => {
                        if named_value.path.is_ident(SELECTOR_ATTR) {
                            let selector_lit = get_lit_str_value(&named_value.lit).ok_or(
                                Diagnostic::new(Level::Error, "selector should be str literal"),
                            )?;
                            check_selector(&selector_lit)?;
                            ret.selector = Some(selector_lit);
                        } else if named_value.path.is_ident(ATTR_ATTR) {
                            let attr_lit = get_lit_str_value(&named_value.lit).ok_or(
                                Diagnostic::new(Level::Error, "attr should be str literal"),
                            )?;
                            ret.attr = Some(attr_lit);
                        }
                    }
                    _ => return Err(diagnostic_invalid_attribute!(quote!(#meta_list))),
                },
            }
        }
        Ok(ret)
    }
}

fn filter_attrs(attrs: Vec<Attribute>) -> Result<Option<Meta>> {
    let attrs: Vec<Attribute> = attrs
        .into_iter()
        .filter_map(|attr| {
            if attr.path.is_ident(HTML_ATTR) {
                Some(attr)
            } else {
                None
            }
        })
        .collect();
    if attrs.is_empty() {
        return Ok(None);
    }

    if attrs.len() > 1 {
        return Err(Diagnostic::new(
            Level::Error,
            "each derived target or field can only have one `html` attribute",
        ));
    }
    Ok(Some(
        attrs
            .into_iter()
            .next()
            .unwrap()
            .parse_meta()
            .map_err(|err| diagnostic_invalid_attribute!(err))?,
    ))
}

fn check_selector(selector: &str) -> Result<()> {
    Selector::parse(selector).map(|_| ()).map_err(|err| {
        Diagnostic::new(
            Level::Error,
            format!("invalid css selector `{}`: {:?}", selector, err),
        )
    })
}

fn get_lit_str_value(lit: &Lit) -> Option<String> {
    if let &Lit::Str(ref str_lit) = lit {
        Some(str_lit.value())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::AttrMeta;
    use proc_macro2::TokenStream;
    use quote::quote;
    use std::convert::TryInto;
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
                default: false,
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
                default: true,
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
}

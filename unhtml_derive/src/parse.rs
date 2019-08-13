use proc_macro::{Diagnostic, Level, TokenStream};
use syn::parse::{Parse, Parser};

pub fn try_parse<T: Parse>(token: TokenStream) -> Result<T, Diagnostic> {
    let copy = token.clone();
    <T as Parse>::parse.parse(token).map_err(|err| {
        Diagnostic::new(
            Level::Error,
            format!("parse token({}) fail: {}", copy.to_string(), err),
        )
    })
}

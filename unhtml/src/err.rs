use derive_more::{Display, From};

// TODO: SourceNotFound should contain selector info
#[derive(Display, Debug, From)]
pub enum Error {
    #[display(fmt = "source not found")]
    SourceNotFound,
    #[display(fmt = "attr(`{}`) is not found in `{}`", attr, src)]
    AttrNotFound { attr: String, src: String },
    #[display(fmt = "{} cannot be parsed as {}: {}", text, type_name, err)]
    TextParseError {
        text: String,
        type_name: String,
        err: String,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

use failure_derive::Fail;

#[derive(Fail, Debug)]
pub enum HtmlError {
    #[fail(display = "{}({}) get nothing", source_type, source_name)]
    SourceNotFound {
        source_type: String,
        source_name: String,
    },
    #[fail(display = "source is empty (selected nothing)")]
    SourceEmpty,
}

pub type Result<T> = std::result::Result<T, failure::Error>;

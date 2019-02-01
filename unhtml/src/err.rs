#[derive(Fail, Debug)]
pub enum DeserializeError {
    #[fail(display = "{}({}) get nothing", attr, value)]
    SourceNotFound {
        attr: String,
        value: String,
        html_fragment: String,
    },
    #[fail(display = "source({}) is empty", source)]
    SourceEmpty {
        source: String,
        html_fragment: String,
    }
}
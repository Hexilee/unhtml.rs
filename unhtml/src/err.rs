#[derive(Fail, Debug)]
pub enum ParseError {
    #[fail(display = "{}({}) get nothing", attr, value)]
    SelectOrAttrEmptyErr {
        attr: String,
        value: String
    }
}
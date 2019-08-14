use crate::{ElemIter, Elements, FromHtml, Result};

struct Link {
    href: String,
    text: String,
}

impl FromHtml for Link {
    fn from_elements(select: ElemIter) -> Result<Self> {
        unimplemented!()
    }
}

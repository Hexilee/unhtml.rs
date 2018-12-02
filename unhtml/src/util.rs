use scraper::{ElementRef, Selector};
use failure::Error;

pub fn vec_from<GetElemFun, T>(selector_str: &str, root_element_ref: ElementRef, get_elem_fun: GetElemFun) -> Result<Vec<T>, Error>
    where GetElemFun: Fn(ElementRef) -> Result<T, Error> {
    let selector = Selector::parse(selector_str).unwrap();
    let selects = root_element_ref.select(&selector);
    let mut list = Vec::new();
    for elem_ref in selects {
        list.push(get_elem_fun(elem_ref)?);
    }
    Ok(list)
}
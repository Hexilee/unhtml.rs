//use scraper::element_ref::ElementRef;
//use scraper::Html;
//use scraper::Node;
//use ego_tree::iter::Nodes;
//
//#[inline]
//pub fn get_select_by_html(nodes: &mut Nodes<Node>) -> Box<Iterator<Item=ElementRef>> {
//    Box::new(nodes.map(|node| ElementRef::wrap(node).unwrap()))
//}
//
//#[inline]
//pub fn get_elem_ref_by_html(nodes: &mut Nodes<Node>) -> Option<ElementRef> {
//    ElementRef::wrap(nodes.next()?)
//}
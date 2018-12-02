use scraper::Html;
use scraper::ElementRef;

/// polyfill for scraper::Html
pub trait RootElementRef {
    fn root_element(&self) -> ElementRef;
}

impl RootElementRef for Html {
    /// Get root element_ref `html`
    /// remove when new version of scraper published
    fn root_element(&self) -> ElementRef {
        let root_node = self.tree.root().first_child().unwrap();
        ElementRef::wrap(root_node).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use scraper::Html;
    use super::*;
    use scraper::Selector;
    #[test]
    fn test_html_root_element_ref() {
        let html = Html::parse_fragment(r#"<a href="http://github.com">1</a>"#);
        let root_ref = html.root_element();
        let href = root_ref.select(&Selector::parse("a").unwrap()).next().unwrap();
        assert_eq!(href.inner_html(), "1");
        assert_eq!(href.value().attr("href").unwrap(), "http://github.com");
    }
}
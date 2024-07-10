pub mod filters {
    use std::fmt;

    use pulldown_cmark;

    /// Renders text to HTML from Markdown.
    pub fn markdown_cmark(s: &dyn fmt::Display) -> ::askama::Result<String> {
        let text = &s.to_string();
        let parser = pulldown_cmark::Parser::new_ext(text, pulldown_cmark::Options::all());

        let mut buf = String::new();
        pulldown_cmark::html::push_html(&mut buf, parser);

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::filters::*;

    #[test]
    fn markdown() {
        assert_eq!(
            markdown_cmark(&"*Simple* ipsum").unwrap(),
            "<p><em>Simple</em> ipsum</p>\n"
        );
    }
}

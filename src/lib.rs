pub mod filters {
    use std::fmt;

    use pulldown_cmark;

    /// An `askama` filter to render `markdown` to html.
    ///
    /// ## Example:
    ///
    /// ```rust
    /// use askama::Template;
    /// use askama_markdown_cmark::filters;
    ///
    /// #[derive(Template)]
    /// #[template(source = "{{ content|markdown_cmark|safe }}", ext="html")]
    /// struct Example<'a> {
    ///     content: &'a str,
    /// }
    ///
    /// fn main() -> std::io::Result<()> {
    ///     println!("{}", Example { content: "## Hello world\n\nTesting ~~x~~ **y** _z_\n" }.render().unwrap());
    ///     Ok(())
    /// }
    /// ```
    #[askama::filter_fn]
    pub fn markdown_cmark<T: fmt::Display>(
        s: T,
        _: &dyn askama::Values,
    ) -> ::askama::Result<String> {
        let text = &s.to_string();
        let parser = pulldown_cmark::Parser::new_ext(text, pulldown_cmark::Options::all());

        let mut buf = String::new();
        pulldown_cmark::html::push_html(&mut buf, parser);

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::filters;
    use askama::Template;

    #[derive(Template)]
    #[template(source = "{{ s | markdown_cmark }}", ext = "md")]
    struct ExampleCmark<'a> {
        s: &'a str,
    }

    #[test]
    fn markdown() {
        let t = ExampleCmark {
            s: "*Simple* ipsum",
        };

        assert_eq!(t.render().unwrap(), "<p><em>Simple</em> ipsum</p>\n");
    }
}

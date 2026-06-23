![Crates.io Version](https://img.shields.io/crates/v/askama-markdown-cmark)
![Crates.io License](https://img.shields.io/crates/l/askama-markdown-cmark)
![Crates.io Total Downloads](https://img.shields.io/crates/d/askama-markdown-cmark)

An `askama` filter to render `markdown` to html.

## Example:

```rust
use askama::Template;
use askama_markdown_cmark::filters;

#[derive(Template)]
#[template(source = "{{ content|markdown_cmark|safe }}", ext="html")]
struct Example<'a> {
    content: &'a str,
}

fn main() -> std::io::Result<()> {
    println!("{}", Example { content: "## Hello world\n\nTesting ~~x~~ **y** _z_\n" }.render().unwrap());
    Ok(())
}
```

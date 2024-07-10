An `askama` filter to render `markdown` to html.

## Example

```rust
use askama::Template;

use askama_markdown_cmark::filters;

#[derive(Template)]
#[template(source = "{{ content|markdown_cmark|safe }}", ext="html")]
struct Example<'a> {
    content: &'a str,
}

fn main() -> Result<()> {
    println!("{}", Example { content: "## Hello world\n\nTesting ~~x~~ **y** _z_\n" }.render().unwrap());

    Ok(())
}
```

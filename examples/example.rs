use askama::Template;

use askama_markdown_cmark::filters;

#[derive(Template)]
#[template(path = "test.askama.html")]
struct Index<'a> {
    content: &'a str,
}

fn main() -> std::io::Result<()> {
    let rendered = Index {
        content: "**And I oop**",
    }
    .render()
    .expect("Static templates should never fail to render.");
    println!("{}", rendered);

    Ok(())
}

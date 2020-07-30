use pulldown_cmark as cmark;

pub fn render(content: String) -> String {
    let parser = cmark::Parser::new(&content);
    content
}

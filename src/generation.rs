use pulldown_cmark::Parser;
use pulldown_cmark::html;

pub struct PageGenerator {
    files: Vec<String>,
}

impl PageGenerator {
    pub fn new() -> Self {
        PageGenerator { files: vec![] }
    }
}

#[cfg(test)]
mod test {
    use pulldown_cmark::Parser;
    use pulldown_cmark::html;

    #[test]
    fn it_parses_markdown_to_html() {
        let markdown = "# Hello, pulldown-cmark!";
        let expected = "<h1>Hello, pulldown-cmark!</h1>";

        let mut parsed_markdown = String::with_capacity(markdown.len() * 3 / 2);
        let parser = Parser::new(&markdown);
        html::push_html(&mut parsed_markdown, parser);

        assert_eq!(expected, parsed_markdown.trim());
    }
}

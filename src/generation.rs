use std::io;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use pulldown_cmark::Parser;
use pulldown_cmark::html;

pub struct PageGenerator;

impl PageGenerator {
    pub fn new() -> Self {
        PageGenerator
    }

    pub fn md_to_html(&self, source: &str, destination: &str) -> Result<(), io::Error> {
        let mut file_contents = String::new();
        let mut input_md_file = try!(File::open(source));

        try!(input_md_file.read_to_string(&mut file_contents));

        let parser = Parser::new(&file_contents);

        let mut parsed_html = String::with_capacity(file_contents.len() * 3 / 2);
        html::push_html(&mut parsed_html, parser);

        let mut output_html_file = try!(OpenOptions::new()
                                            .read(true)
                                            .write(true)
                                            .create(true)
                                            .open(destination));

        try!(output_html_file.write_all(parsed_html.as_bytes().as_ref()));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;
    use std::io::prelude::*;
    use std::fs;
    use std::fs::File;
    use std::env::temp_dir;
    use std::error::Error;
    use pulldown_cmark::Parser;
    use pulldown_cmark::html;

    #[test]
    fn it_parses_a_valid_markdown_file_to_html() {
        // Set up mock file in temp_dir
        let temp_dir = String::from(temp_dir().to_str().unwrap());
        let md_file_name = temp_dir.clone() + "/test.md";
        let html_file_name = temp_dir.clone() + "/test.html";

        let mut file = match File::create(&md_file_name) {
            Ok(file) => file,
            Err(what) => panic!("{}", Error::description(&what))
        };

        match file.write(b"# This is a test") {
            Ok(_) => {},
            Err(what) => panic!("{}", Error::description(&what))
        };

        // Read mock md file to HTML
        let page_generator = PageGenerator::new();
        let result = page_generator.md_to_html(&md_file_name, &html_file_name);

        // Assert contents are as expected
        let expected = String::from("<h1>This is a test</h1>");
        let mut actual = String::new();

        let mut output_file = match File::open(&html_file_name) {
            Ok(file) => file,
            Err(what) => panic!("{}", Error::description(&what))
        };

        output_file.read_to_string(&mut actual);

        assert_eq!(expected, actual.trim());

        // Delete mock file in temp_dir
        fs::remove_file(&md_file_name);
        fs::remove_file(&html_file_name);
    }

    #[test]
    #[should_panic]
    fn it_panics_when_file_cannot_be_found() {
        // Setup
        let temp_dir = String::from(temp_dir().to_str().unwrap());
        let md_file_name = temp_dir.clone() + "/test.md";
        let html_file_name = temp_dir.clone() + "/test.html";

        // Attempt to parse - expect panic!
        let page_generator = PageGenerator::new();
        let result = match page_generator.md_to_html(&md_file_name, &html_file_name) {
            Ok(_) => {},
            Err(what) => panic!("{}", Error::description(&what))
        };
    }
}

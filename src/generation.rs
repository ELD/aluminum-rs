use std::io;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use pulldown_cmark::Parser;
use pulldown_cmark::html;

pub struct PageGenerator {
    input_file: String,
    output_file: String,
    wrap_html: bool,
}

impl PageGenerator {
    pub fn new() -> Self {
        PageGenerator {
            input_file: String::new(),
            output_file: String::new(),
            wrap_html: false,
        }
    }

    pub fn set_input_file<S: Into<String>>(&mut self, input_file: S) -> &mut Self {
        self.input_file = input_file.into();
        self
    }

    pub fn set_output_file<S: Into<String>>(&mut self, output_file: S) -> &mut Self {
        self.output_file = output_file.into();
        self
    }

    pub fn set_wrap(&mut self, wrap: bool) -> &mut Self {
        self.wrap_html = wrap;
        self
    }

    pub fn generate(&self) -> Result<(), io::Error> {
        let mut parsed_html = try!(self.md_to_html());

        if self.wrap_html {
            parsed_html = "<!DOCTYPE html>\n\
             <html>\n\
             <head>\n\
             <title>Aluminum Page</title>\n\
             </head>\n\
             <body>\n".to_string() + parsed_html.as_ref() +
                "</body>\n\
                 </html>"
        }

        let mut output_file = try!(OpenOptions::new()
                                            .read(true)
                                            .write(true)
                                            .create(true)
                                            .open(&self.output_file));

        try!(output_file.write_all(&parsed_html.as_bytes()));

        Ok(())
    }

    fn md_to_html(&self) -> Result<String, io::Error> {
        let mut file_contents = String::new();
        let mut input_md_file = try!(File::open(&self.input_file));

        try!(input_md_file.read_to_string(&mut file_contents));

        let parser = Parser::new(&file_contents);

        let mut parsed_html = String::with_capacity(file_contents.len() * 3 / 2);
        html::push_html(&mut parsed_html, parser);

        Ok(parsed_html)
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
        let md_file_name = temp_dir.clone() + "/test1.md";
        let html_file_name = temp_dir.clone() + "/test1.html";

        let mut file = match File::create(&md_file_name) {
            Ok(file) => file,
            Err(what) => panic!("{}", Error::description(&what))
        };

        match file.write(b"# This is a test") {
            Ok(_) => {},
            Err(what) => panic!("{}", Error::description(&what))
        };

        // Read mock md file to HTML
        let page_generator = PageGenerator::new()
                                .set_input_file(md_file_name.as_ref())
                                .set_output_file(html_file_name.as_ref())
                                .set_wrap(false)
                                .generate();

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
        let md_file_name = temp_dir.clone() + "/test2.md";
        let html_file_name = temp_dir.clone() + "/test2.html";

        // Attempt to parse - expect panic!
        let mut page_generator = PageGenerator::new();
        page_generator.set_input_file(md_file_name.as_ref())
                      .set_output_file(html_file_name.as_ref());

        let result = match page_generator.generate() {
            Ok(_) => {},
            Err(what) => panic!("{}", Error::description(&what))
        };
    }

    #[test]
    fn it_wraps_generated_md_in_well_formed_html_skeleton() {
        let temp_dir = String::from(temp_dir().to_str().unwrap());
        let md_file_name = temp_dir.clone() + "/test3.md";
        let html_file_name = temp_dir.clone() + "/test3.html";

        let mut file = match File::create(&md_file_name) {
            Ok(file) => file,
            Err(what) => panic!("{}", Error::description(&what))
        };

        file.write_all(b"# This is a test\nAnd some more text here...");

        let page_generator = PageGenerator::new()
                                .set_input_file(md_file_name.as_ref())
                                .set_output_file(html_file_name.as_ref())
                                .set_wrap(true)
                                .generate();

        let mut actual = String::new();

        let mut output_file = match File::open(&html_file_name) {
            Ok(file) => file,
            Err(what) => panic!("{}", Error::description(&what))
        };

        output_file.read_to_string(&mut actual);

        let expected = "<!DOCTYPE html>\n\
                            <html>\n\
                                <head>\n\
                                    <title>Aluminum Page</title>\n\
                                </head>\n\
                                <body>\n\
                                    <h1>This is a test</h1>\n\
                                    <p>And some more text here...</p>\n\
                                </body>\n\
                            </html>";

        assert_eq!(actual, expected);

        fs::remove_file(&md_file_name);
        fs::remove_file(&html_file_name);
    }
}

use std::io;
use std::io::prelude::{Write, Read};
use std::fs::File;
use std::fs::OpenOptions;
use std::default::Default;
use pulldown_cmark::{Parser, html, Options};

pub struct PageGenerator {
    input_file: String,
    output_file: String,
    parse_options: Options,
    wrap_html: bool,
}

impl PageGenerator {
    pub fn new() -> Self {
        Self::default()
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

    pub fn set_parse_options(&mut self, parse_options: Options) -> &mut Self {
        self.parse_options = parse_options;
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

        let parser = Parser::new_ext(&file_contents, self.parse_options);

        let mut parsed_html = String::with_capacity(file_contents.len() * 3 / 2);
        html::push_html(&mut parsed_html, parser);

        Ok(parsed_html)
    }
}

impl Default for PageGenerator {
    fn default() -> Self {
        PageGenerator {
            input_file: String::new(),
            output_file: String::new(),
            parse_options: Options::empty(),
            wrap_html: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::prelude::{Read, Write};
    use std::fs::File;
    use std::env::temp_dir;
    use tempdir::TempDir;

    #[test]
    fn it_parses_a_valid_markdown_file_to_html() {
        let temp_dir = TempDir::new("parse-valid-markdown").expect("Temp Dir");
        let md_file_name = temp_dir.path().join("test.md");
        let html_file_name = temp_dir.path().join("test.html");

        let mut file = File::create(&md_file_name).expect("Markdown File Create");

        writeln!(file, "# This is a test").expect("Write Markdown");

        PageGenerator::new()
            .set_input_file(md_file_name.to_str().expect("Input File"))
            .set_output_file(html_file_name.to_str().expect("Output File"))
            .set_wrap(false)
            .generate()
            .expect("Generate Pages");

        let expected = "<h1>This is a test</h1>".to_string();
        let mut actual = String::new();

        let mut output_file = File::open(&html_file_name).expect("Open HTML File");

        output_file.read_to_string(&mut actual).expect("Reading HTML File");

        assert_eq!(expected, actual.trim());
    }

    #[test]
    #[should_panic]
    fn it_panics_when_file_cannot_be_found() {
        let temp_dir = temp_dir().to_string_lossy().into_owned();
        let md_file_name = temp_dir.clone() + "/test2.md";
        let html_file_name = temp_dir.clone() + "/test2.html";

        let mut page_generator = PageGenerator::new();
        page_generator.set_input_file(md_file_name.as_str())
                      .set_output_file(html_file_name.as_str());

        page_generator.generate().expect("Generate Pages");
    }

    #[test]
    fn it_wraps_generated_md_in_well_formed_html_skeleton() {
        let temp_dir = TempDir::new("wrap-generated-html").expect("Temp Dir");
        let md_file_name = temp_dir.path().join("test.md");
        let html_file_name = temp_dir.path().join("test.html");

        let mut file = File::create(&md_file_name).expect("Create Markdown File");

        writeln!(file, "# This is a test\nAnd some more text here...").expect("Write Markdown");

        PageGenerator::new()
            .set_input_file(md_file_name.to_str().expect("Input File"))
            .set_output_file(html_file_name.to_str().expect("Output File"))
            .set_wrap(true)
            .generate()
            .expect("Generate Pages");

        let mut actual = String::new();

        let mut output_file = File::open(&html_file_name).expect("Open Output File");

        output_file.read_to_string(&mut actual).expect("Read Output");

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
    }
}

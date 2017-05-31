use std::io;
use std::io::prelude::Read;
use std::fs::File;
use std::default::Default;

use pulldown_cmark::{Parser, html, Options};

use regex::Regex;

use yaml_rust::YamlLoader;
use yaml_rust::yaml::Yaml;

lazy_static! {
    static ref FRONT_MATTER_REGEX: Regex = Regex::new(r"(?s)^(?:---)\s+(.*)\s+(?:---)\s+(.*)").unwrap();
}

pub struct Page {
    pub front_matter: Yaml,
    pub contents: String,
}

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

    pub fn parse_file(&self) -> Result<Page, io::Error> {
        let mut file_contents = String::new();
        File::open(&self.input_file)?.read_to_string(&mut file_contents)?;

        let (front_matter, contents) = if FRONT_MATTER_REGEX.is_match(&file_contents) {
            let captures = FRONT_MATTER_REGEX.captures(&file_contents).expect("Regex failed despite a match");
            (YamlLoader::load_from_str(&captures[1]).expect("Could not load YAML")[0].clone(), captures[2].to_string())
        } else {
            (Yaml::Null, file_contents)
        };

        Ok(Page {
            front_matter: front_matter,
            contents: self.md_to_html(&contents).expect("Could not generate Markdown")
        })
    }

    fn md_to_html(&self, file_contents: &str) -> Result<String, io::Error> {
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

    use std::io::prelude::Write;
    use std::fs::File;
    use std::env::temp_dir;
    use std::collections::BTreeMap;

    use tempdir::TempDir;

    #[test]
    fn it_parses_a_valid_markdown_file_to_html() {
        let temp_dir = TempDir::new("parse-valid-markdown").expect("Temp Dir");
        let md_file_name = temp_dir.path().join("test.md");
        let html_file_name = temp_dir.path().join("test.html");

        let mut file = File::create(&md_file_name).expect("Markdown File Create");

        writeln!(file, "# This is a test").expect("Write Markdown");

        let actual = PageGenerator::new()
            .set_input_file(md_file_name.to_str().expect("Input File"))
            .set_output_file(html_file_name.to_str().expect("Output File"))
            .set_wrap(false)
            .parse_file()
            .expect("Generate Pages");

        let expected = "<h1>This is a test</h1>".to_string();

        assert_eq!(Yaml::Null, actual.front_matter);
        assert_eq!(expected, actual.contents.trim());
    }

    #[test]
    fn it_parses_frontmatter_and_returns_a_page_object() {
        let temp_dir = TempDir::new("parse-front-matter").expect("Temp Dir");
        let md_file_name = temp_dir.path().join("test.md");
        let html_file_name = temp_dir.path().join("test.html");

        let mut file = File::create(&md_file_name).expect("Markdown file create");

        writeln!(file, "---\ntitle: My Page\ntags:\n  - one\n  - two\n---\n# This is a test!").expect("Write markdown");

        let page = PageGenerator::new()
            .set_input_file(md_file_name.to_str().expect("Input File"))
            .set_output_file(html_file_name.to_str().expect("Output File"))
            .set_wrap(true)
            .parse_file()
            .expect("Generate Page");

        let mut btree_map = BTreeMap::new();
        let tags = vec![Yaml::String("one".to_string()), Yaml::String("two".to_string())];

        btree_map.insert(Yaml::String("title".to_string()), Yaml::String("My Page".to_string()));
        btree_map.insert(Yaml::String("tags".to_string()), Yaml::Array(tags));

        let expected_frontmatter = Yaml::Hash(btree_map);

        let expected_html = "<h1>This is a test!</h1>".to_string();

        assert_eq!(expected_frontmatter, page.front_matter);
        assert_eq!(expected_html, page.contents.trim());
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

        page_generator.parse_file().expect("Generate Pages");
    }
}

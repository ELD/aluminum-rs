use std::io;
use std::fs;
use std::fs::DirBuilder;
use std::path::Path;
use super::generation::PageGenerator;

pub fn new_project(parent_dir: &str) -> Result<(), io::Error> {
    try!(DirBuilder::new().recursive(true).create(parent_dir));

    try!(DirBuilder::new().recursive(false).create(format!("{}/pages", parent_dir)));

    Ok(())
}

pub fn build_project() -> Result<(), io::Error> {
    let pages_path = "pages";
    let output_dir = "_site";
    let mut page_generator = PageGenerator::new();

    let directory_iterator = try!(Path::new(pages_path).read_dir());

    try!(DirBuilder::new().create(output_dir));

    for entry in directory_iterator {
        let file = try!(entry);
        let file_type = file.file_type().unwrap();
        let source_file = format!("{}/{}", pages_path, file.file_name().to_str().unwrap());
        let file_stem = String::from(file.path().file_stem().unwrap().to_str().unwrap());
        let destination_file = format!("{}/{}.html", output_dir, file_stem);

        if file_type.is_file() && file.file_name().to_str().unwrap().contains(".md") {
            try!(page_generator.set_input_file(source_file.as_ref())
                                .set_output_file(destination_file.as_ref())
                                .set_wrap(true)
                                .generate());
        }
    }

    Ok(())
}

pub fn clean_project() -> Result<(), io::Error> {
    try!(fs::remove_dir_all("_site"));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::remove_dir_all;
    use std::env;
    use std::path;
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn it_creates_default_directory_structure() {
        // Setup
        let temp_dir = env::temp_dir();
        let proj_dir = String::from(temp_dir.to_str().unwrap()) + "/test-project0";
        let page_dir = proj_dir.clone() + "/pages";

        new_project(proj_dir.as_ref());

        // Assert directories exists
        assert!(path::Path::new(&proj_dir).exists());
        assert!(path::Path::new(&page_dir).exists());

        // Teardown
        match remove_dir_all(proj_dir) {
            Ok(_) => {},
            Err(what) => panic!("{}", Error::description(&what))
        }
    }

    #[test]
    fn it_parses_md_files_to_html_in_the_project() {
        // Setup
        let temp_dir = env::temp_dir();
        let proj_dir = String::from(temp_dir.to_str().unwrap()) + "/test-project1";
        let page_dir = proj_dir.clone() + "/pages";
        let site_dir = proj_dir.clone() + "/_site";
        let test_file_name = page_dir.clone() + "/test.md";
        let test_file_compiled_name = "_site/test.html";

        new_project(&proj_dir);

        // Add markdown file
        let mut test_file = match File::create(&test_file_name) {
            Ok(file) => file,
            Err(what) => panic!("{}", Error::description(&what))
        };

        test_file.write_all(b"# This is a test");

        env::set_current_dir(&proj_dir);
        build_project();

        let mut test_output_file = match File::open(&test_file_compiled_name) {
            Ok(file) => file,
            Err(what) => panic!("{}", Error::description(&what))
        };

        let mut compiled_contents = String::new();

        test_output_file.read_to_string(&mut compiled_contents);

        let expected = "<!DOCTYPE html>\n\
                        <html>\n\
                        <head>\n\
                        <title>Aluminum Page</title>\n\
                        </head>\n\
                        <body>\n\
                        <h1>This is a test</h1>\n\
                        </body>\n\
                        </html>";

        assert!(path::Path::new(&test_file_compiled_name).exists());
        assert_eq!(expected, compiled_contents.trim());

        // Teardown
        match remove_dir_all(proj_dir) {
            Ok(_) => {},
            Err(what) => panic!("{}", Error::description(&what))
        }
    }

    #[test]
    fn it_deletes_the_site_directory_when_the_project_gets_cleaned() {
        let temp_dir = env::temp_dir();
        let proj_dir = String::from(temp_dir.to_str().unwrap()) + "/test-project2";
        let site_dir = proj_dir.clone() + "/_site";

        new_project(&proj_dir);

        env::set_current_dir(&proj_dir);
        build_project();

        assert!(path::Path::new(&site_dir).exists());

        clean_project();

        assert!(!path::Path::new(&site_dir).exists());
    }
}

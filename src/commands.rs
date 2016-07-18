use std::io;
use std::fs;
use std::fs::{DirBuilder, File};
use std::path::Path;
use super::generation::PageGenerator;

pub fn new_project(parent_dir: &str) -> Result<(), io::Error> {
    try!(DirBuilder::new().recursive(true).create(parent_dir));

    try!(DirBuilder::new().recursive(false).create(format!("{}/pages", parent_dir)));

    try!(File::create(format!("{}/_config.yml", parent_dir)));

    Ok(())
}

pub fn build_project() -> Result<(), io::Error> {
    let pages_path = "pages";
    let output_dir = "_site";
    let mut page_generator = PageGenerator::new();

    let directory_iterator = try!(Path::new(pages_path).read_dir());

    if !Path::new(output_dir).exists() {
        try!(DirBuilder::new().create(output_dir));
    }

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
    use std::env;
    use std::path;
    use std::fs::{create_dir, File};
    use std::io::prelude::*;
    use tempdir::TempDir;

    #[test]
    fn it_creates_default_directory_structure() {
        let temp_dir = TempDir::new("creates-default-directory").expect("Temp Dir");
        let proj_dir = temp_dir.path().join("test-project");
        let page_dir = proj_dir.join("pages");
        let site_config = proj_dir.join("_config.yml");

        new_project(proj_dir.to_str().unwrap()).expect("New Project");

        assert!(path::Path::new(&proj_dir).exists());
        assert!(path::Path::new(&page_dir).exists());
        assert!(path::Path::new(&site_config).exists());
    }

    #[test]
    fn it_parses_md_files_to_html_in_the_project() {
        let tmp_dir = TempDir::new("parse-md-project").expect("Temp Dir");

        let pages_dir_path = tmp_dir.path().join("pages");
        let site_dir_path = tmp_dir.path().join("_site");

        create_dir(&pages_dir_path).expect("Pages Dir");

        let md_file_path = pages_dir_path.join("test.md");
        let html_file_path = site_dir_path.join("test.html");

        let mut md_file_handle = File::create(md_file_path).expect("Markdown file");

        writeln!(md_file_handle, "# This is a test").expect("Write Markdown");

        env::set_current_dir(tmp_dir.path()).expect("Set Working Dir");

        build_project().ok().expect("Build Project");

        let mut output_file = File::open(html_file_path).expect("HTML file");

        let mut compiled_contents = String::new();

        output_file.read_to_string(&mut compiled_contents).expect("Read Output File");

        let expected = "<!DOCTYPE html>\n\
                        <html>\n\
                        <head>\n\
                        <title>Aluminum Page</title>\n\
                        </head>\n\
                        <body>\n\
                        <h1>This is a test</h1>\n\
                        </body>\n\
                        </html>";

        assert_eq!(expected, compiled_contents.trim());
    }

    #[test]
    fn it_deletes_the_site_directory_when_the_project_gets_cleaned() {
        let tmp_dir = TempDir::new("clean-project").expect("Temp Dir");
        let page_dir_path = tmp_dir.path().join("pages");
        let site_dir_path = tmp_dir.path().join("_site");

        create_dir(&page_dir_path).expect("Page Dir");
        create_dir(&site_dir_path).expect("Site Dir");

        assert!(site_dir_path.exists());

        env::set_current_dir(&tmp_dir.path()).expect("Set Working Directory");
        clean_project().expect("Clean Project");

        assert!(!site_dir_path.exists());
    }
}

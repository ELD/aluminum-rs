use std::io;
use std::fs::DirBuilder;
use std::path::Path;
use std::error::Error;
use super::generation::PageGenerator;

pub fn new_project(parent_dir: &str) {
    match DirBuilder::new().recursive(true).create(parent_dir) {
        Ok(_) => {},
        Err(what) => println!("{}", Error::description(&what))
    }

    match DirBuilder::new().recursive(false).create(format!("{}/pages", parent_dir)) {
        Ok(_) => {},
        Err(what) => println!("{}", Error::description(&what))
    }
}

pub fn build_project() -> Result<(), io::Error> {
    let pages_path = "pages";
    let output_dir = "_site";
    let page_generator = PageGenerator::new();

    let directory_iterator = try!(Path::new(pages_path).read_dir());

    DirBuilder::new().create(format!("{}", output_dir));

    for entry in directory_iterator {
        let file = try!(entry);
        let file_type = file.file_type().unwrap();
        let source_file = format!("{}/{}", pages_path, file.file_name().to_str().unwrap());
        let file_stem = String::from(file.path().file_stem().unwrap().to_str().unwrap());
        let destination_file = format!("{}/{}.html", output_dir, file_stem);

        if file_type.is_file() && file.file_name().to_str().unwrap().contains(".md") {
            page_generator.md_to_html(&source_file, &destination_file);
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::remove_dir_all;
    use std::env;
    use std::path;
    use std::error::Error;

    #[test]
    fn it_creates_default_directory_structure() {
        // Setup
        let temp_dir = env::temp_dir();
        let proj_dir = String::from(temp_dir.to_str().unwrap()) + "/test-project";
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
}

use std::fs::DirBuilder;
use std::error::Error;

pub fn new_project(parent_dir: &str) {
    match DirBuilder::new().recursive(true).create(parent_dir) {
        Ok(_) => {},
        Err(what) => println!("{}", Error::description(&what))
    }

    match DirBuilder::new().recursive(false).create(format!("{}/pages", parent_dir)) {
        Ok(_) => {},
        Err(what) => println!("{}", Error::description(&what))
    }

    match DirBuilder::new().recursive(false).create(format!("{}/_site", parent_dir)) {
        Ok(_) => {},
        Err(what) => println!("{}", Error::description(&what))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::{DirBuilder, remove_dir_all};
    use std::env;
    use std::path;
    use std::error::Error;

    #[test]
    fn it_creates_default_directory_structure() {
        // Setup
        let temp_dir = env::temp_dir();
        let proj_dir = String::from(temp_dir.to_str().unwrap()) + "/test-project";
        let site_dir = proj_dir.clone() + "/_site";
        let page_dir = proj_dir.clone() + "/pages";

         new_project(proj_dir.as_ref());

        // Assert directories exists
        assert!(path::Path::new(&proj_dir).exists());
        assert!(path::Path::new(&site_dir).exists());
        assert!(path::Path::new(&page_dir).exists());

        // Teardown
        match remove_dir_all(proj_dir) {
            Ok(_) => {},
            Err(what) => panic!("{}", Error::description(&what))
        }
    }
}

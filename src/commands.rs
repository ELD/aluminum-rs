pub fn new_project(parent_dir: &str) {
    println!("Creating new project with name {}!", parent_dir);
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

        // TODO: Replace this logic with created directories
         new_project(proj_dir.as_ref());

        // Assert equal
        assert!(path::Path::new(&proj_dir).exists());

        // Teardown
        match remove_dir_all(proj_dir) {
            Ok(_) => {},
            Err(what) => panic!("{}", Error::description(&what))
        }
    }
}

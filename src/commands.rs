use std::io;
use std::fs;
use std::fs::{DirBuilder, File};
use std::path::Path;
use super::generation::PageGenerator;
use super::config::Config;

pub fn new_project(parent_dir: &str) -> Result<(), io::Error> {
    try!(DirBuilder::new().recursive(true).create(parent_dir));

    try!(DirBuilder::new().recursive(false).create(format!("{}/pages", parent_dir)));

    try!(File::create(format!("{}/_config.yml", parent_dir)));

    Ok(())
}

pub fn build_project(config: Config) -> Result<(), io::Error> {
    let pages_path = &config.source_dir;
    let output_dir = &config.output_dir;
    let mut page_generator = PageGenerator::new();

    let directory_iterator = try!(Path::new(pages_path).read_dir());

    if !Path::new(output_dir).exists() {
        try!(DirBuilder::new().create(output_dir));
    }

    for entry in directory_iterator {
        let file = try!(entry);
        let file_type = try!(file.file_type());

        let file_name = match file.file_name().to_str() {
            Some(file_name) => file_name.to_string(),
            None => continue
        };

        let source_file = format!("{}/{}", pages_path, file_name);

        let file_stem = match file.path().file_stem() {
            Some(file_stem) => file_stem.to_string_lossy().into_owned(),
            None => continue
        };

        let destination_file = format!("{}/{}.html", output_dir, file_stem);

        let file_ext = match file.file_name().to_str() {
            Some(file_ext) => file_ext.to_string(),
            None => continue
        };

        if file_type.is_file() && file_ext.contains(".md") {
            try!(page_generator.set_input_file(source_file.as_ref())
                     .set_output_file(destination_file.as_ref())
                     .set_wrap(true)
                     .generate());
        }
    }

    Ok(())
}

pub fn clean_project(config: Config) -> Result<(), io::Error> {
    try!(fs::remove_dir_all(config.output_dir));

    Ok(())
}

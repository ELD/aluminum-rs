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

        let file_name = file.file_name().into_string().expect("File Name");

        let source_file = format!("{}/{}", pages_path, file_name);

        let file_stem = file.path().file_stem().expect("File Stem").to_string_lossy().into_owned();

        let destination_file = format!("{}/{}.html", output_dir, file_stem);

        if file_type.is_file() && file_name.contains(".md") {
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

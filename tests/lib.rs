extern crate aluminum;

use std::path::Path;
use std::fs::{remove_dir_all, create_dir};

use aluminum::commands;
use aluminum::config;

#[test]
fn it_creates_a_new_project() {
    let proj_dir = "tests/tmp/new-project";
    let pages_dir = "tests/tmp/new-project/pages";
    let config_path = "tests/tmp/new-project/_config.yml";

    commands::new_project(&proj_dir).expect("New Project");

    assert!(Path::new(&proj_dir).exists());
    assert!(Path::new(&pages_dir).exists());
    assert!(Path::new(&config_path).exists());

    remove_dir_all(proj_dir).expect("Clean Up");
}

#[test]
fn it_builds_a_default_project() {

}

#[test]
fn it_deletes_the_built_site_on_clean() {
    let dir_to_clean = "tests/tmp/clean-project/_site";
    let mut config = config::Config::default();

    config.output_dir = "tests/tmp/clean-project/_site".to_string();

    assert!(Path::new(dir_to_clean).exists());

    commands::clean_project(config).expect("Clean Project");

    assert!(!Path::new(dir_to_clean).exists());

    create_dir(dir_to_clean).expect("Clean Up");
}

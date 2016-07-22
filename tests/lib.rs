extern crate aluminum;

use std::io::Read;
use std::path::Path;
use std::fs::{File, remove_dir_all, create_dir};

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
    let test_proj_dir = "tests/tmp/default-project";
    let source_dir = format!("{}/pages", test_proj_dir);
    let output_dir = format!("{}/_site", test_proj_dir);
    let actual_output_file_path = format!("{}/test.html", output_dir);

    let fixture_proj_dir = "tests/fixtures/default-project";
    let fixture_output_file_path = format!("{}/_site/test.html", fixture_proj_dir);

    let mut config = config::Config::default();

    config.source_dir = source_dir.clone();
    config.output_dir = output_dir.clone();

    commands::build_project(config).expect("Build Project");

    let mut fixture_contents = String::new();
    let mut fixture_output_file = File::open(fixture_output_file_path).expect("Fixture File");

    fixture_output_file.read_to_string(&mut fixture_contents).expect("Read Fixture File");

    let mut actual_contents = String::new();
    let mut actual_output_file = File::open(actual_output_file_path).expect("Actual File");

    actual_output_file.read_to_string(&mut actual_contents).expect("Read Actual File");

    assert_eq!(fixture_contents.trim(), actual_contents.trim());

    remove_dir_all(output_dir).expect("Clean Up");
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
    File::create(format!("{}/.gitkeep", dir_to_clean)).expect("Clean Up");
}

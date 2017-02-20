extern crate aluminum;
extern crate hyper;
extern crate walkdir;
extern crate tempdir;
#[macro_use(assert_diff)]
extern crate difference;

use std::io;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::thread;

use hyper::Client;
use walkdir::WalkDir;
use tempdir::TempDir;

use aluminum::commands;
use aluminum::config;

fn run_create_tests(test_name: &str) -> Result<(), io::Error> {
    let target = format!("tests/target/{}/", test_name);

    let tempdir = TempDir::new(test_name).expect("Couldn't create temporary directory");

    let result = commands::new_project(tempdir.path().to_str().expect("Couldn't convert path to str"));

    if result.is_ok() {
        let walkdir = WalkDir::new(&target).into_iter().filter_map(|e| e.ok());

        for entry in walkdir {
            if entry.file_type().is_file() {
                let file_name = entry.path().strip_prefix(&target).expect("Couldn't get file name");

                let mut expected = String::new();
                File::open(&entry.path()).expect("Couldn't open expected file").read_to_string(&mut expected).expect("Couldn't read expected file");

                let mut actual = String::new();
                File::open(&tempdir.path().join(file_name)).expect("Couldn't open expected file").read_to_string(&mut actual).expect("Couldn't read expected file");

                assert_diff!(&expected, &actual, " ", 0);
            } else if entry.file_type().is_dir() {
                let dir_name = entry.path().strip_prefix(&target).expect("Couldn't get directory name.");

                assert!(tempdir.path().join(dir_name).exists());
            }
        }
    }

    tempdir.close().expect("Couldn't clean up temporary directory");

    result
}

fn run_build_tests(test_name: &str, config_options: Vec<String>) -> Result<(), io::Error> {
    let target = format!("tests/target/{}/", test_name);

    let mut config = config::Config::default();
    let tempdir = TempDir::new(test_name).expect("Failed to create temporary directory under test");

    config.source_dir = format!("tests/fixtures/{}/pages", test_name);
    config.output_dir = tempdir.path().to_str().expect("Can't convert to string").to_string();
    config.markdown_options = config_options;

    let result = commands::build_project(&config);

    if result.is_ok() {
        let target_files = WalkDir::new(&target)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file());

        for file in target_files {
            let tmp_path = file.path().strip_prefix(&target).expect("Couldn't get tmp_file path");

            let mut expected = String::new();
            File::open(&file.path()).expect("Couldn't open expected file").read_to_string(&mut expected).expect("Couldn't read to string.");

            let mut actual = String::new();
            File::open(&Path::new(&config.output_dir).join(tmp_path)).expect("Couldn't open actual file").read_to_string(&mut actual).expect("Couldn't read to string.");

            assert_diff!(&expected, &actual, " ", 0);
        }
    }

    tempdir.close().ok().expect("Failed to close temp dir");

    result
}

fn run_serve_tests(test_name: &str, mut config: config::Config, expected_status: hyper::status::StatusCode) -> Result<(), io::Error> {
    let target = format!("tests/target/{}/", test_name);
    let tempdir = TempDir::new(test_name).expect("Failed to create temporary directory under test");
    let port = config.port.clone();

    config.source_dir = format!("tests/fixtures/{}/pages", test_name);
    config.output_dir = tempdir.path().to_str().expect("Can't convert tempdir path to string").to_string();

    thread::spawn(move || {
        commands::serve(&config).expect("Serve Command");
    });

    // Walk through _site dir, check file contents against HTTP served body
    let target_files = WalkDir::new(&target)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file());

    for file in target_files {
        let file_url = file.path().strip_prefix(&target).expect("Couldn't get file URL").to_str().expect("Could not convert to string");

        let mut expected = String::new();
        File::open(
            &Path::new(&target).join(file_url)
        ).expect("Couldn't open actual file")
            .read_to_string(&mut expected)
            .expect("Couldn't read to string");

        let client = Client::new();
        let mut response = client.get(&format!("http://localhost:{}/{}", port, file_url)).send().expect("Sending Client Request");

        let mut response_body = String::new();
        response.read_to_string(&mut response_body).expect("Response Body");


        assert_diff!(&expected, &response_body, " ", 0);
        assert_eq!(expected_status, response.status);
    }

    Ok(())
}

#[test]
fn it_creates_a_new_project() {
    run_create_tests("new-project").expect("Project creation error");
}

#[test]
fn it_builds_a_default_project() {
    run_build_tests("default-project", Vec::new()).expect("Failed to build a default project");
}

#[test]
fn it_builds_a_project_with_footnote_and_table_support() {
    run_build_tests("enhanced-project", vec!["tables".to_string(), "footnotes".to_string()]).expect("Failed to build a project with footnote and table support");
}

#[test]
fn it_deletes_the_built_site_on_clean() {
    // Setup
    let fixture_dir = "tests/fixtures/clean";
    let tempdir = TempDir::new("clean_test").expect("Unable to create temporary directory");
    let mut config = config::Config::default();
    config.source_dir = fixture_dir.to_string();
    config.output_dir = format!("{}/_site", tempdir.path().to_str().expect("Couldn't convert path to string"));

    commands::build_project(&config).expect("Building site for clean test");

    // Setup assertion
    assert!(Path::exists(&tempdir.path().join("_site/test.html")));

    // Act
    commands::clean_project(&config).expect("Cleaning site for clean test");

    // Assert
    assert!(!Path::exists(&tempdir.path().join("_site")));

    tempdir.close().ok().expect("Failed to clean up temporary directory");
}

#[test]
fn it_spins_up_a_web_server_on_serve_command() {
    run_serve_tests("serve-simple-project-built", config::Config::default(), hyper::Ok).expect("Failed to serve simple project");
}

#[test]
fn the_port_number_can_be_changed() {
    let mut config = config::Config::default();
    config.port = "4001".to_string();

    run_serve_tests("serve-simple-project", config, hyper::Ok).expect("Could not change the port number");
}

#[test]
fn it_returns_a_404_when_the_route_is_invalid() {
    let mut config = config::Config::default();
    config.port = "4002".to_string();

    run_serve_tests("serve-bad-route", config, hyper::NotFound).expect("Did not error out on a bad request");
}

#[test]
fn it_hits_every_route_in_the_pages_directory() {
    let mut config = config::Config::default();
    config.port = "4003".to_string();

    run_serve_tests("serve-project-multiple-pages", config, hyper::Ok).expect("Could not serve multiple pages");
}

#[test]
fn it_builds_the_project_before_serving_the_site() {
    let base_dir = "tests/fixtures/serve-project-non-built".to_string();
    let pages_path = "pages";
    let site_path = "_site";
    let port = "4004".to_string();
    let mut config = config::Config::default();
    config.source_dir = format!("{}/{}", base_dir, pages_path);
    config.output_dir = format!("{}/{}", base_dir, site_path);
    config.port = port.clone();

    assert!(!Path::new(&format!("{}/{}", base_dir, site_path)).exists());

    thread::spawn(move || {
        commands::serve(&config).expect("Serve");
    });

    let server_addr = format!("http://localhost:{}/index.html", port);
    let client = Client::new();
    let response = client.get(server_addr.as_str()).send().expect("Sending Client Request");

    assert_eq!(hyper::Ok, response.status);

    let mut clean_config = config::Config::default();
    clean_config.output_dir = format!("{}/{}", base_dir, site_path);

    commands::clean_project(&clean_config).expect("Clean Up");
    assert!(!Path::new(&format!("{}/{}", base_dir, site_path)).exists());
}

#[test]
fn it_returns_400_on_a_bad_request() {
    let base_dir = "tests/fixtures/serve-simple-project-built".to_string();
    let site_dir = "_site".to_string();
    let page_dir = "pages".to_string();
    let port = "4005".to_string();

    let mut config = config::Config::default();
    config.source_dir = format!("{}/{}", base_dir, page_dir);
    config.output_dir = format!("{}/{}", base_dir, site_dir);
    config.port = port.clone();

    thread::spawn(move || {
        commands::serve(&config).expect("Serve");
    });

    let server_addr = format!("http://localhost:{}", port);
    let client = Client::new();

    let response = client.post(server_addr.as_str()).send().expect("Send Bad Request");

    assert_eq!(hyper::BadRequest, response.status);
}

#[test]
#[should_panic]
fn it_panics_on_invalid_server_connection() {
    let mut config = config::Config::default();
    config.source_dir = "tests/target/serve-simple-project-built/pages".to_string();
    config.output_dir = "tests/target/serve-simple-project-built/_site".to_string();
    config.port = "65536".to_string();
    commands::serve(&config).expect("Serve");
}

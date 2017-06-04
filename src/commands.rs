use super::generation::PageGenerator;
use super::config::Config;

use std::io;
use std::io::{Read, Write};
use std::fs;
use std::fs::{DirBuilder, File};
use std::path::Path;
use std::error::Error;
use std::fs::OpenOptions;

use hyper::server::{Request, Response, Server};
use hyper::status::StatusCode;
use hyper::uri::RequestUri;
use hyper::method::Method;

use walkdir::WalkDir;

use pulldown_cmark::{Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

const DEFAULT_CONFIG_FILE: &'static str = "\
source: pages
output: _site
port: 4000
markdown_options:
  - tables
  - footnotes
";

const BAD_REQUEST: &'static str = "\
<h1>400 Bad Request</h1>
";

const NOT_FOUND: &'static str = "\
<h1>404 Not Found</h1>
";

pub fn new_project(parent_dir: &str) -> Result<(), io::Error> {
    DirBuilder::new().recursive(true).create(parent_dir)?;

    DirBuilder::new().recursive(false).create(format!("{}/pages", parent_dir))?;

    let mut config_file = File::create(format!("{}/_config.yml", parent_dir))?;

    config_file.write_all(DEFAULT_CONFIG_FILE.as_bytes())?;

    Ok(())
}

pub fn build_project(config: &Config) -> Result<(), io::Error> {
    let pages_path = &*config.source_dir;
    let output_dir = &*config.output_dir;
    let mut markdown_options = Options::empty();

    if config.markdown_options.contains(&"footnotes".to_string()) {
        markdown_options.insert(OPTION_ENABLE_FOOTNOTES);
    }

    if config.markdown_options.contains(&"tables".to_string()) {
        markdown_options.insert(OPTION_ENABLE_TABLES);
    }

    let mut page_generator = PageGenerator::new();

    let directory_iterator = WalkDir::new(pages_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() &&
            !e.file_name().to_str().unwrap().starts_with("_") &&
            !(e.path().to_str().unwrap().contains("/_") || e.path().to_str().unwrap().contains("\\_")));

    if !Path::new(output_dir).exists() {
        DirBuilder::new().create(output_dir)?;
    }

    for file in directory_iterator {
        // Name of the file?
        let file_name = file.file_name().to_str().unwrap().to_string();

        let destination_file = format!("{}/{}", output_dir, file
            .path()
            .strip_prefix(pages_path)
            .unwrap()
            .with_extension("html")
            .display()
        );

        fs::create_dir_all(Path::new(&destination_file).parent().unwrap()).unwrap();

        if file_name.contains(".md") {
            let parsed = page_generator.set_input_file(file.path().to_str().expect("Couldn't convert for some reason"))
                .set_output_file(destination_file.as_str())
                .set_wrap(true)
                .set_parse_options(markdown_options.clone())
                .parse_file()?
                .render_to_string()?;

            let mut output_file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(destination_file.as_str())?;

            output_file.write_all(&parsed.as_bytes())?;
        } else if file_name.contains(".html") {
            let output_file_name = format!("{}/{}", config.output_dir, file_name);
            let output_file_path = Path::new(&output_file_name);

            fs::copy(file.path(), output_file_path).unwrap();
        }
    }

    Ok(())
}

pub fn clean_project(config: &Config) -> Result<(), io::Error> {
    fs::remove_dir_all(&*config.output_dir)?;

    Ok(())
}

pub fn serve(config: &Config) -> Result<(), io::Error> {
    build_project(&config)?;

    let server_addr = format!("127.0.0.1:{}", &*config.port);
    let server = match Server::http(server_addr.as_str()) {
        Ok(server) => server,
        Err(what) => panic!("{}", Error::description(&what))
    };

    let serve_dir = config.output_dir.clone();
    match server.handle(move |request: Request, response: Response| {
        match handle_static_file(&serve_dir, request, response) {
            Ok(_) => {}
            Err(what) => panic!("{}", Error::description(&what))
        }
    }) {
        Ok(_) => {}
        Err(what) => panic!("{}", Error::description(&what))
    }

    Ok(())
}

fn handle_static_file(page_dir: &str, request: Request, mut response: Response) -> Result<(), io::Error> {
    let path = match request.uri {
        RequestUri::AbsolutePath(ref uri) if request.method == Method::Get => {
            if uri == "/" {
                "/index.html"
            } else {
                uri
            }
        },
        _ => {
            *response.status_mut() = StatusCode::BadRequest;
            let body = BAD_REQUEST.as_bytes();
            response.send(body)?;
            return Ok(())
        }
    };

    let file_path = Path::new(page_dir).join(&path[1..]);

    if file_path.exists() && file_path.is_file() {
        let mut file = File::open(file_path)?;
        let mut file_contents = String::new();

        file.read_to_string(&mut file_contents)?;

        *response.status_mut() = StatusCode::Ok;
        response.send(&file_contents.into_bytes())?;
        return Ok(())
    } else {
        *response.status_mut() = StatusCode::NotFound;
        let body = NOT_FOUND.as_bytes();
        response.send(body)?;
        return Ok(())
    }
}

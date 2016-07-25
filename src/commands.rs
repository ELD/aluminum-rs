use std::io;
use std::io::{Read, Write};
use std::fs;
use std::fs::{DirBuilder, File};
use std::path::Path;
use std::error::Error;
use std::process;
use super::generation::PageGenerator;
use super::config::Config;
use hyper::server::{Request, Response, Server};
use hyper::status::StatusCode;
use hyper::uri::RequestUri;

pub fn new_project(parent_dir: &str) -> Result<(), io::Error> {
    try!(DirBuilder::new().recursive(true).create(parent_dir));

    try!(DirBuilder::new().recursive(false).create(format!("{}/pages", parent_dir)));

    let mut config_file = try!(File::create(format!("{}/_config.yml", parent_dir)));

    try!(config_file.write_all(b"source: pages\noutput: _site\n\n"));

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

pub fn serve(config: Config) -> Result<(), io::Error> {
    let mut server = match Server::http("127.0.0.1:4000") {
        Ok(server) => server,
        Err(what) => {
            try!(writeln!(io::stderr(), "{}", Error::description(&what)));
            process::exit(1);
        }
    };

    server.handle(move |request: Request, response: Response| {
        handle_static_file(request, response);
    });

    Ok(())
}

fn handle_static_file(request: Request, mut response: Response) -> Result<(), io::Error> {
    let output_dir = "tests/tmp/serve-project-built/_site";

    let path = match request.uri {
        RequestUri::AbsolutePath(uri) => uri,
        _ => {
            *response.status_mut() = StatusCode::BadRequest;
            let body = b"<h1>400 Bad Request</h1>";
            try!(response.send(body));
            return Ok(())
        }
    };

    let file_path = Path::new(output_dir).join(&path[1..]);

    if file_path.exists() && file_path.is_file() {
        let mut file = try!(File::open(file_path));
        let mut file_contents = String::new();

        file.read_to_string(&mut file_contents);

        *response.status_mut() = StatusCode::Ok;
        try!(response.send(&file_contents.into_bytes()));
        return Ok(())
    } else {
        *response.status_mut() = StatusCode::NotFound;
        let body = b"<h1>404: Not Found</h1>";
        try!(response.send(body));
        return Ok(())
    }

    println!("{:?}", file_path);

    Ok(())
}

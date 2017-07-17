extern crate clap;
extern crate aluminum;

use std::io;
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use clap::{App, Arg, AppSettings, SubCommand};
use aluminum::commands;
use aluminum::config::Config;

const VERSION_NUMBER: &'static str = "0.3.0";

fn main() {
    let matches = App::new("Aluminum")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(VERSION_NUMBER)
        .about("Static site generator")
        .subcommand(SubCommand::with_name("new").arg(Arg::with_name("project name")
                                                         .index(1)
                                                         .required(true)))
        .subcommand(SubCommand::with_name("build"))
        .subcommand(SubCommand::with_name("clean"))
        .subcommand(SubCommand::with_name("serve"))
        .get_matches();


    if let ("new", Some(new)) = matches.subcommand() {
        let project_name = if new.is_present("project name") {
            new.value_of("project name").unwrap()
        } else {
            ""
        };

        commands::new_project(project_name).unwrap();
    } else if matches.is_present("build") {
        let mut config_file_contents = String::new();
        let mut config_file = match File::open("_config.yml") {
            Ok(file) => file,
            Err(what) => panic!("No config file present: {}", Error::description(&what))
        };

        match config_file.read_to_string(&mut config_file_contents) {
            Ok(_) => {},
            Err(what) => panic!("Unable to read config file: {}", Error::description(&what))
        }

        println!("Building project...");
        let config = Config::from_string(config_file_contents);
        match commands::build_project(&config) {
            Ok(_) => {},
            Err(what) => {
                writeln!(io::stderr(), "Error: {}", Error::description(&what)).expect("Print Error");
            }
        }
    } else if matches.is_present("clean") {
        let mut config_file_contents = String::new();
        let mut config_file = match File::open("_config.yml") {
            Ok(file) => file,
            Err(what) => panic!("No config file present: {}", Error::description(&what))
        };

        match config_file.read_to_string(&mut config_file_contents) {
            Ok(_) => {},
            Err(what) => panic!("Unable to read config file: {}", Error::description(&what))
        }

        println!("Cleaning project...");
        let config = Config::from_string(config_file_contents);
        match commands::clean_project(&config) {
            Ok(_) | Err(_) => {},
        }
    } else if matches.is_present("serve") {
        let mut config_file_contents = String::new();
        let mut config_file = match File::open("_config.yml") {
            Ok(file) => file,
            Err(what) => panic!("No config file present: {}", Error::description(&what))
        };

        match config_file.read_to_string(&mut config_file_contents) {
            Ok(_) => {},
            Err(what) => panic!("Unable to read config file: {}", Error::description(&what))
        }

        println!("Serving project");
        let config = Config::from_string(config_file_contents);
        commands::serve(&config).expect("Serve");
    }
}

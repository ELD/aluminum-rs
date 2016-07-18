extern crate clap;
extern crate aluminum;

use std::io;
use std::io::prelude::*;
use std::error::Error;
use clap::{App, Arg, AppSettings, SubCommand};
use aluminum::commands;

const VERSION_NUMBER: &'static str = "0.1.0";

fn main() {
    let matches = App::new("Rukyll")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(VERSION_NUMBER)
        .about("Static site generator")
        .subcommand(SubCommand::with_name("new").arg(Arg::with_name("project name")
                                                         .index(1)
                                                         .required(true)))
        .subcommand(SubCommand::with_name("build"))
        .subcommand(SubCommand::with_name("clean"))
        .get_matches();


    if let ("new", Some(new)) = matches.subcommand() {
        let project_name = if new.is_present("project name") {
            new.value_of("project name").unwrap()
        } else {
            ""
        };

        commands::new_project(project_name).unwrap();
    } else if matches.is_present("build") {
        println!("Building project...");
        match commands::build_project() {
            Ok(_) => {},
            Err(what) => {
                writeln!(io::stderr(), "Error: {}", Error::description(&what)).expect("Print Error");
            }
        }
    } else if matches.is_present("clean") {
        println!("Cleaning project...");
        match commands::clean_project() {
            Ok(_) | Err(_) => {},
        }
    }
}

extern crate clap;
extern crate aluminum;

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
        .get_matches();


    if let ("new", Some(new)) = matches.subcommand() {
        // TODO: Better error handling - Propagate errors up the stack to this method
        let project_name = if new.is_present("project name") {
            new.value_of("project name").unwrap()
        } else {
            ""
        };

        commands::new_project(project_name).unwrap();
    } else if let ("build", Some(build)) = matches.subcommand() {
        // Do building of project/markdown files
        println!("Building project...");
        commands::build_project().unwrap();
    }
}

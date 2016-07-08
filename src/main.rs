extern crate clap;
extern crate rukyll;

use clap::{App, Arg, AppSettings, SubCommand};
use rukyll::commands;

const VERSION_NUMBER: &'static str = "0.1.0";

fn main() {
    let matches = App::new("Rukyll")
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .version(VERSION_NUMBER)
                        .about("Static site generator")
                        .subcommand(SubCommand::with_name("new")
                            .arg(Arg::with_name("project name")
                            .index(1)
                            .required(true)
                            )
                        )
                        .get_matches();

    match matches.subcommand() {
        ("new", Some(new)) => {
            // TODO: Better error handling
            let project_name = if new.is_present("project name") {
                new.value_of("project name").unwrap()
            } else {
                ""
            };

            commands::new_project(&project_name);
        },
        _ => {}
    }
}

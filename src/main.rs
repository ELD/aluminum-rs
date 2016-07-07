extern crate clap;
extern crate rukyll;

use clap::{Arg, App, SubCommand};
use rukyll::commands::{new_project};

fn main() {
    let matches = App::new("Rukyll")
                        .version("0.1.0")
                        .about("Static site generator")
                        .subcommand(SubCommand::with_name("new"))
                        .get_matches();

    match matches.subcommand_name() {
        Some("new") => {
            new_project();
        },
        Some(_) => {},
        None => {}
    }
}

extern crate clap;
extern crate term_size;

use clap::{App, Arg};

mod core;

use crate::core::Core;

#[derive(Debug)]
pub struct Options {
    display_all: bool,
    display_long: bool,
}

fn main() {
    let app = App::new("l")
        .version("1.0.0")
        .author("lome")
        .about("this is a l")
        .arg(Arg::with_name("all")
            .short("a")
            .help("Show all file and dir"))
        .arg(Arg::with_name("long")
            .short("l")
            .help("show file all info"))
        .arg(Arg::with_name("FILE")
            .multiple(true).default_value("."))
        .get_matches();

    let option = Options {
        display_all: app.is_present("all"),
        display_long: app.is_present("long")
    };

    let files: Vec<&str> = app
        .values_of("FILE")
        .expect("not get any file")
        .collect();
    let core = Core::new(&option);
    core.run(files);
}
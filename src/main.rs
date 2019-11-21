extern crate clap;

use clap::{App, Arg};
use std::fs::read_dir;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

#[derive(Debug)]
pub struct Option {
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

    let option = Option {
        display_all: app.is_present("all"),
        display_long: app.is_present("long")
    };

    let files: Vec<&str> = app
        .values_of("FILE")
        .expect("not get any file")
        .collect();

    for file in files {
        let path = Path::new(file);
        if path.is_dir() {
            println!("{}: ", file);
            let somes = read_dir(path).expect("not get this path");
            for some in somes {
                if let Ok(ref entry) = some {
                    let name = entry.file_name().to_str().expect("file name convert str fail").to_string();
                    let data = entry.metadata().expect("no metadata");
                    if option.display_long {
                        if option.display_all || !name.starts_with(".") {
                            let permission = data.permissions().mode();
                            let systime = data.modified().expect("get system time error");
                            println!("    {} Permission: {:o}  Systime: {:?}", name, permission, systime);
                        }
                    } else {
                        if option.display_all || !name.starts_with(".") {
                            println!("    {}", name);
                        }
                    }
                }
            }
        }
    }
}
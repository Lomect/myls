use std::fs::read_dir;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

use crate::Options;

pub struct Core<'a> {
    options: &'a Options
}

impl<'a> Core<'a> {
    pub fn new(options: &'a Options) -> Self {
        Core {
            options,
        }
    }

    pub fn run(&self, files: Vec<&str>) {
        for file in files {
            let path = Path::new(file);
            if path.is_dir() {
                println!("{}: ", file);
                let somes = read_dir(path).expect("not get this path");
                for some in somes {
                    if let Ok(ref entry) = some {
                        let name = entry.file_name().to_str().expect("file name convert str fail").to_string();
                        let data = entry.metadata().expect("no metadata");
                        if self.options.display_long {
                            if self.options.display_all || !name.starts_with(".") {
                                let permission = data.permissions().mode();
                                let systime = data.modified().expect("get system time error");
                                println!("    {} Permission: {:o}  Systime: {:?}", name, permission, systime);
                            }
                        } else {
                            if self.options.display_all || !name.starts_with(".") {
                                println!("    {}", name);
                            }
                        }
                    }
                }
            }
        }
    }
}
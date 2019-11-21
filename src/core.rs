use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;

use term_size;

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
        let mut origin_files= Vec::new();
        let mut origin_dirs = Vec::new();
        for file in files {
            let path = Path::new(file);
            if path.is_dir() {
                origin_dirs.push(path);
            } else {
                origin_files.push(path.to_path_buf());
            }
        }

        origin_files.sort_unstable();
        origin_dirs.sort_unstable();

        if !self.options.display_long {
            if !origin_files.is_empty() {
                self.print_short(&origin_files);
            }

            for dir in origin_dirs {
                println!("{}: ", dir.display().to_string());
                let paths = self.list_folder(dir);
                self.print_short(&paths);
            }
        }
    }

    fn print_short(&self, files: &Vec<PathBuf>) {
        let width = match term_size::dimensions() {
            Some((width, _)) => width,
            None => panic!("cannot get terminal size"),
        };

        let mut names = Vec::new();
        let mut name_max_size  = 0;
        for file in files {
            let name = file.file_name()
                .expect(format!("Path: {}, Get file name fail Path", file.display().to_string()).as_str())
                .to_str()
                .expect(format!("Os str convert to str fail Path: {}", file.display().to_string()).as_str());
            if name.len() > name_max_size {
                name_max_size = name.len();
            }
            names.push(name);
        }

        let max_num_pow = width/(name_max_size+2);
        let mut content = String::new();
        let mut per_value = 0;

        for name in names {
            if self.options.display_all || !name.starts_with(".") {
                content += "  ";
                per_value += 1;
                content += name;

                if name.len() < name_max_size {
                    for _ in 0..(name_max_size - name.len()) {
                        content.push(' ');
                    }
                }

                if per_value == max_num_pow {
                    println!("{}", content);
                    content.clear();
                    per_value = 0;
                }
            }
        }

        println!("{}", content);
    }

    fn list_folder(&self, path: &Path) -> Vec<PathBuf> {
        // 这里返回`PathBuf`是因为如果返回Vec<Path>的话，由于
        // Path不是Sized的类型，所以不能作为返回值，如果返回
        // Vec<&Path>， 由于新产生的Path是在该函数作用范围内的，如果
        // 函数返回后该Path不在作用范围内，是错误的，这里还可以使用生命周期，
        // 使得输入参数Path和输出Path生命周期相同
        // fn list_folder(&self, path: &'a Path) -> Vec<&'a Path> {}

        let mut inner_folder = Vec::new();

        let folders = read_dir(path).expect("read dir fail");
        for folder in folders {
            if let Ok(entry) = folder {
                let folder_path = entry.path();
                inner_folder.push(folder_path);
            }
        }
        inner_folder
    }
}
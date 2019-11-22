use std::fs::read_dir;
use std::path::{Path, PathBuf};

use term_size;

use crate::Options;
use crate::meta::Meta;

pub struct Core<'a> {
    options: &'a Options
}

#[derive(Default)]
pub struct MaxInfo {
    pub name: usize,
    pub user: usize,
    pub filesize: usize,
    pub sizeuint: usize,
    pub group: usize
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
                self.print_short(origin_files);
            }

            for dir in origin_dirs {
                println!("{}: ", dir.display().to_string());
                let paths = self.list_folder(dir);
                self.print_short(paths);
            }
        } else {
            if !origin_files.is_empty() {
                let mut dirinfo = MaxInfo::default();
                let mut metas = self.path_to_metas(origin_files, &mut dirinfo);
                self.print_long(&mut metas, &mut dirinfo);
            }

            for dir in origin_dirs {
                println!("{}: ", dir.display().to_string());
                let mut dirinfo = MaxInfo::default();
                let paths = self.list_folder(dir);
                let mut metas = self.path_to_metas(paths, &mut dirinfo);
                self.print_long(&mut metas, &mut dirinfo);
            }

        }
    }

    fn print_short(&self, files: Vec<PathBuf>) {
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
                .expect(format!("Os str convert to str fail Path: {}", file.display().to_string()).as_str())
                .to_string();

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
                content.push_str(name.as_str());

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

    fn print_long(&self, metas: &mut Vec<Meta>, dirinfo: &mut MaxInfo) {
        for meta in metas {
            meta.format_meta(dirinfo);
            println!("{} {} {} {} {} {} {}", meta.permission,
                     meta.group,
                     meta.user,
                     meta.filesize,
                     meta.size_uint,
                     meta.time,
                     meta.name);
        }
    }

    fn list_folder(&self, path: &Path) -> Vec<PathBuf> {
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

    fn path_to_metas(&self, paths: Vec<PathBuf>, dirinfo: &mut MaxInfo) -> Vec<Meta> {
        let mut metas = Vec::new();
        for path in paths {
            let meta = match Meta::from_path(path.as_path()) {
                Ok(meta) => {
                    max(&meta, dirinfo);
                    meta
                },
                Err(e) => panic!(e),
            };
            metas.push(meta);
        }
        metas
    }
}

fn max(meta: &Meta, dirinfo: &mut MaxInfo) {
    if meta.filesize.len() > dirinfo.filesize {
        dirinfo.filesize = meta.filesize.len();
    }

    if meta.group.len() > dirinfo.group {
        dirinfo.group = meta.group.len();
    }

    if meta.user.len() > dirinfo.user {
        dirinfo.user = meta.user.len();
    }

    if meta.size_uint.len() > dirinfo.sizeuint {
        dirinfo.sizeuint = meta.size_uint.len();
    }
}

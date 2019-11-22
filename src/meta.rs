use std::path::{PathBuf, Path};
use std::fs::{Metadata, read_link};
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;

use users::{get_user_by_uid, get_group_by_gid};
use size::{Size, Base, Style};

use crate::core::MaxInfo;
use std::time::UNIX_EPOCH;
use time::Timespec;

quick_error!(
    #[derive(Debug)]
    pub enum MetaError {
        UnableReadName(path: String) {
            from(path)
        }

        Encoding(path: String) {
            from(path)
        }

        UnableReadMate(path: String) {
            from(path)
        }
    }
);


pub struct Meta {
    pub path: PathBuf,
    pub name: String,
    pub permission: String,
    pub group: String,
    pub user: String,
    pub meta: Metadata,
    pub symlink: Option<String>,
    pub filesize: String,
    pub size_uint: String,
    pub time: String,
}

impl Meta {
    pub fn from_path(path: &Path) -> Result<Self, MetaError> {
        let name = match path.file_name() {
            Some(name) => match name.to_str() {
                Some(name) => name.to_string(),
                None => return Err(MetaError::Encoding(format!("Os str convert str fail. Path: {}", path.display().to_string())))
            },
            None => return Err(MetaError::UnableReadName(format!("Get file name error. Path: {}", path.display().to_string())))
        };

        let (meta, symlink) = match read_link(path) {
            Ok(path) => {
                let meta = path.symlink_metadata().expect("Fail to read symlink");
                let symlink = path.to_str().expect("Fail to convert pathbuf to str");
                (meta, Some(symlink.to_string()))
            },
            _ => {
                let meta = match path.metadata() {
                    Ok(meta) => meta,
                    _ => return Err(MetaError::UnableReadMate(format!("Get file metadata error. Path: {}", path.display().to_string())))
                };
                (meta, None)
            }
        };
        let user = get_user_by_uid(meta.uid())
            .expect("Get user by uid error")
            .name()
            .to_str()
            .expect("User name os str convert to str fail")
            .to_string();

        let group = get_group_by_gid(meta.gid())
            .expect("Get group by uid error")
            .name()
            .to_str()
            .expect("Group name os str convert to str fail")
            .to_string();

        let filesize = Size::Bytes(meta.len()).to_string(Base::Base10, Style::Abbreviated);
        let filesize_parts: Vec<_> = filesize.split(" ").collect();

        let modified = meta.modified().expect("Get modified time fail");
        let dur = modified.duration_since(UNIX_EPOCH).expect("Get The modified duration fail");
        let modified_time = time::at(Timespec::new(
            dur.as_secs() as i64,
                dur.subsec_nanos() as i32,));
        let modified = modified_time
            .strftime("%m月 %d %H:%M")
            .expect("format time error");

        Ok(Meta{
            path: path.to_path_buf(),
            name,
            user,
            group,
            permission: String::new(),
            meta,
            symlink,
            filesize: filesize_parts[0].to_string(),
            size_uint: filesize_parts[1].to_string(),
            time: modified.to_string(),
        })
    }

    fn format_group(&mut self, maxinfo: &MaxInfo) {
        if self.group.len() < maxinfo.group {
            for _ in 0..(maxinfo.group - self.group.len()) {
                self.group.push(' ');
            }
        }
    }

    fn format_user(&mut self, maxinfo: &MaxInfo) {
        if self.user.len() < maxinfo.user {
            for _ in 0..(maxinfo.user - self.user.len()) {
                self.user.push(' ');
            }
        }
    }

    fn format_filesize(&mut self, maxinfo: &MaxInfo) {
        if self.filesize.len() < maxinfo.filesize {
            for _ in 0..(maxinfo.filesize - self.filesize.len()) {
                self.filesize.push(' ');
            }
        }
    }

    fn format_uint(&mut self, maxinfo: &MaxInfo) {
        if self.size_uint.len() < maxinfo.sizeuint {
            for _ in 0..(maxinfo.sizeuint - self.size_uint.len()) {
                self.size_uint.push(' ');
            }
        }
    }

    fn format_symlink(&mut self) {
        match self.symlink {
            Some(ref link) => {
                self.name.push_str(" => ");
                self.name.push_str(link.as_str());
            },
            None => {}
        }
    }

    fn format_permission(&mut self) {
        let unix_permission = "rwxrwxrwx".to_string();
        let mut permission = String::new();
        if self.path.is_dir() {
            permission.push('d');
        } else {
            match self.symlink {
                Some(_) =>  permission.push('l'),
                None =>  permission.push('-')
            }
        }

        let mode = self.meta.permissions().mode();
        for (key, value) in unix_permission.chars().enumerate() {
            if mode & 0b100000000>>key == 0 {
                permission.push('-')
            } else {
                permission.push(value);
            }
        }
        self.permission = permission;
    }

    pub fn format_meta(&mut self, dirinfo: &MaxInfo) {
        self.format_permission();
        self.format_symlink();
        self.format_group(dirinfo);
        self.format_user(dirinfo);
        self.format_filesize(dirinfo);
        self.format_uint(dirinfo);
    }
}

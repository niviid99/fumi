use std::fs;

pub struct BasicConf {
    pub hidden_files: bool,
}

impl Default for BasicConf {
    fn default() -> Self {
        Self {
            hidden_files: false,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Copy)]
pub enum FileType {
    File,
    Symlink,
    Dir,
    Unknown,
}

#[derive(Clone)]
pub struct File {
    pub path: String,
    pub name: String,
    pub filetype: FileType,
}

impl Default for File {
    fn default() -> Self {
        Self {
            path: "".to_string(),
            name: "".to_string(),
            filetype: FileType::Unknown,
        }
    }
}

pub fn ls_to_arr(array: &mut Vec<File>, path: &str, conf: BasicConf) -> Result<(), ()> {
    array.clear();

    let farr = fs::read_dir(path);
    if farr.is_err() {
        return Err(());
    }

    let ets = farr.unwrap();
    for e in ets {
        if e.is_err() {
            continue;
        }

        let mut final_name = String::new();

        let x = e.unwrap();
        let xfn = x.file_name().to_string_lossy().to_string();

        final_name.push_str(&xfn);

        let mut ft = FileType::Unknown;
        let xt = x.file_type();
        if xt.is_ok() {
            let xt = xt.unwrap();
            if xt.is_symlink() {
                final_name.push_str(" \u{f0c1}");
                ft = FileType::Symlink;
            }
            if xt.is_dir() {
                final_name.push_str(" \u{ea83}");
                ft = FileType::Dir;
            }
            if xt.is_file() {
                final_name.push_str(" \u{ea7b}");
                ft = FileType::File;
            }
        }

        if !conf.hidden_files && final_name.starts_with('.') {
            continue;
        }

        let mut out = File::default();
        out.name = final_name;
        out.path = x.path().to_string_lossy().to_string();
        out.filetype = ft;

        array.push(out);
    }

    Ok(())
}

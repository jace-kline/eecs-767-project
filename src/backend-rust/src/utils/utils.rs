use std::path::{Path, PathBuf};

pub fn file_name(p: &str) -> Option<String> {
    let s = PathBuf::from(p)
        .as_path()
        .file_name()?
        .to_str()?
        .to_string();
    Some(s)
}

pub fn file_extension(p: &str) -> Option<String> {
    let s = PathBuf::from(p)
        .as_path()
        .extension()?
        .to_str()?
        .to_string();
    Some(s)
}
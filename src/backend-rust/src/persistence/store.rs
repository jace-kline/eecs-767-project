use crate::utils::types::{FileMap, FileInfo};
use std::path::Path;
use std::error::Error;
use std::fmt::Display;
use std::io::BufWriter;
use std::io::Write;
use std::fs::File;

impl FileInfo {
    pub fn write_to_file<P>(path: &P, map: &FileMap<FileInfo>) -> Result<(), Box<dyn Error>> 
    where P: AsRef<Path>,
    {
        let file = File::create(path)?;
        let mut file = BufWriter::new(file);

        for (path, attrs) in map {
            file.write_all(FileInfo::format_line(path, attrs).as_bytes())?;
        }
        Ok(())
    }

    fn format_line<P>(path: &P, attrs: &FileInfo) -> String
    where P: AsRef<Path> + Display,
    {
        let indexed = if attrs.indexed {1} else {0};
        format!("{},{},{},{}\n", path, attrs.size, attrs.modified, indexed)
    }
}
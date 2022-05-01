use std::{fmt::{Debug}, path::Path};
use filetime::{FileTime};
use walkdir::{WalkDir, DirEntry};
use crate::types::{FileInfo, FileMap, FilePath};

fn mk_fileinfo(entry: DirEntry) -> Option<(FilePath, FileInfo)> {

    // get file information

    // get metadata
    let metadata: std::fs::Metadata = entry.metadata().ok()?;
    // convert path to canonical form (from root)
    let pathbuf = entry.into_path().canonicalize().ok()?;
    let fname = pathbuf.as_path().file_name()?.to_str()?.to_string();
    let pathstr = pathbuf.as_os_str().to_str()?;
    let path = String::from(pathstr);

    // get file size
    let size = metadata.len();
    // get modification time (seconds from epoch)
    let modified = FileTime::from_last_modification_time(&metadata).unix_seconds();

    // checks to occur before creation

    // 1. does path exist?
    let exists: bool = pathbuf.exists();
    // 2. is regular file?
    let is_regular: bool = pathbuf.is_file();

    (exists && is_regular).then(||
        (path,
            FileInfo {
                fname,
                size,
                modified
            }
        )
    )

}

pub fn scrape_files<P>(root: P) -> FileMap<FileInfo> 
where P: AsRef<Path>
{

    fn filter_file_entry(entry: walkdir::Result<DirEntry>) -> Option<(FilePath, FileInfo)> {
        entry.ok().and_then(|entry| mk_fileinfo(entry))
    }

    WalkDir::new(root)
        .into_iter()
        .filter_map(filter_file_entry)
        .collect()
}
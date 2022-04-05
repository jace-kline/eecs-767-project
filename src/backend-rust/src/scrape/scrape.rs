use std::fmt::{Debug};
use filetime::{FileTime};
use walkdir::{WalkDir, DirEntry};
use crate::utils::types::{FileMap, FilePath};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct FileInfo {
    pub size: u64,
    pub modified: i64,
    pub indexed: bool, // assume this is true to start with
}

impl FileInfo {
    fn try_create(entry: DirEntry) -> Option<(FilePath, FileInfo)> {

        // get file information

        // get metadata
        let metadata: std::fs::Metadata = entry.metadata().ok()?;
        // convert path to canonical form (from root)
        let pathbuf = entry.into_path().canonicalize().ok()?;
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
                    size,
                    modified,
                    indexed: true
                }
            )
        )

    }

    // public accessor functions
    pub fn size(&self) -> u64 { return self.size }
    pub fn modified(&self) -> i64 { return self.modified }
    pub fn indexed(&self) -> bool { return self.indexed }

    // set whether this file is being indexed
    pub fn set_indexed(&mut self, b: bool) { self.indexed = b }
}

pub fn scrape_files(root: &str) -> FileMap<FileInfo> {

    fn filter_file_entry(entry: walkdir::Result<DirEntry>) -> Option<(FilePath, FileInfo)> {
        entry.ok().and_then(|entry| FileInfo::try_create(entry))
    }

    WalkDir::new(root)
        .into_iter()
        .filter_map(filter_file_entry)
        .collect()
}
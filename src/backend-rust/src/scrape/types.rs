use crate::types::{FilePath};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub fname: String,
    pub size: u64,
    pub modified: i64
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum ScrapeTag {
    New,   // for new files that should be parsed/indexed OR previously ignored files that have been modified
    Indexed, // for files that match previously stored files
    IndexedRemoved, // for stored files that have been moved / no longer exist
    IndexedModified, // for stored files that match stored path, but not attrs
    Ignored, // for stored files that we have identified should not be parsed/indexed based on stored ignored list
    IgnoredRemoved
}

#[derive(Debug, Clone)]
pub struct ScrapeDiffRecord {
    pub tag: ScrapeTag, 
    pub path: FilePath, 
    pub info: FileInfo
}

impl ScrapeDiffRecord {
    pub fn new(tag: ScrapeTag, path: FilePath, info: FileInfo) -> Self {
        Self {
            tag,
            path,
            info
        }
    }
}
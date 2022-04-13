pub use super::index::Index;
pub use super::frequency_index::FrequencyIndex;
use crate::types::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum IndexTag {
    Indexed,
    Ignored
}

pub type StoredFileInfo = (IndexTag, FileInfo);

#[derive(Debug, PartialEq, Eq)]
pub enum IndexUpdate {
    AddIndexed(FilePath, FileInfo, TermMap<Frequency>),
    AddIgnored(FilePath, FileInfo),
    ReplaceIndexed(FilePath, FileInfo, TermMap<Frequency>),
    ReplaceIgnored(FilePath, FileInfo),
    Remove(FilePath)
}

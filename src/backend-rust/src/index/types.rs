pub use super::index::Index;
use crate::types::{FileMap, TermMap, Frequency, FileInfo};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub enum IndexTag {
    Indexed,
    Ignored
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StoredFileInfo(pub IndexTag, pub FileInfo);

pub use super::index::Index;
use crate::types::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum IndexTag {
    Indexed,
    Ignored
}

pub type StoredFileInfo = (IndexTag, FileInfo);

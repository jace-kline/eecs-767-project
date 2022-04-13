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

pub use super::index::Index;
use crate::types::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum IndexTag {
    Indexed,
    Ignored
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct StoredFileInfo(pub IndexTag, pub FileInfo);

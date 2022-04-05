
pub type FilePath = String;
pub type Term = String;

pub type FileMap<V> = BTreeMap<FilePath, V>;
pub type TermMap<V> = BTreeMap<Term, V>;

pub type Frequency = u32;
pub type Score = f32;

use std::collections::BTreeMap;

// re-export scrape types
pub use crate::scrape::{FileInfo};


pub type FilePath = String;
pub type Term = String;

pub type FileMap<V> = BTreeMap<FilePath, V>;
pub type TermMap<V> = BTreeMap<Term, V>;

use std::collections::BTreeMap;

// re-export scrape types
pub use crate::scrape::{FileInfo};

use std::collections::BTreeMap;

pub type FilePath = String;
pub type Term = String;

pub type FileMap<V> = BTreeMap<FilePath, V>;
pub type TermMap<V> = BTreeMap<Term, V>;

pub type Frequency = usize;
pub type Score = f64;
pub type Weight = f64;

pub use super::index::types::*;
// pub use super::persistence::types::*;
pub use super::score::types::*;
pub use super::scrape::types::*;
// pub use super::text::types::*;
pub use super::utils::types::*;
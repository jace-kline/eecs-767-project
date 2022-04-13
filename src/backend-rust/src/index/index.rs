use std::collections::BTreeMap;
use crate::types::*;
use crate::types::IndexTag::*;
use serde::{Serialize, Deserialize};
use std::io::{BufReader};
use std::{path::Path, fs::File};
use crate::utils::io::overwrite_file;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Index {
    pub file_info_index: FileMap<StoredFileInfo>,
    pub frequency_index: FrequencyIndex
}

impl Index {

    pub fn new() -> Self {
        Self {
            file_info_index: BTreeMap::new(),
            frequency_index: FrequencyIndex::new()
        }
    }

    // add a [term->freq] map corresponding to a new text-processed document
    pub fn add_indexed(&mut self, path: &str, info: FileInfo, term_freq_map: TermMap<Frequency>) -> Option<()>
    {
        // if the path was already present -> failure
        // path must be removed before re-inserting
        let res = self.file_info_index.insert(path.to_string(), (Indexed, info));
        if res.is_some() { return None; }

        self.frequency_index.add(path, term_freq_map)
        
    }

    pub fn add_ignored(&mut self, path: &str, info: FileInfo) -> Option<()> {
        // if the path was already present -> failure
        // path must be removed before re-inserting
        let res = self.file_info_index.insert(path.to_string(), (Ignored, info));
        if res.is_some() { None } else { Some(()) }
    }

    // remove a document from the index
    pub fn remove(&mut self, path: &str) -> Option<()> {

        let (tag, _) = self.file_info_index.get(path)?;
        
        if let Indexed = tag {
            self.frequency_index.remove(path)?;
        }

        self.file_info_index.remove(path)?;
        Some(())
    }

    // persistence - read from file
    pub fn from_file<P>(stored_index_path: P) -> Option<Self>
    where P: AsRef<Path> {
        let file = File::open(stored_index_path).ok()?;
        let reader = BufReader::new(file);
        // serde_json::from_reader(reader).ok()
        bson::from_reader(reader).ok()
    }

    // persistence - store to file
    pub fn to_file<P>(&self, stored_index_path: P) -> Option<()>
    where P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr> + Copy {
        // let output = serde_json::to_string(index).ok()?;
        let output = bson::to_vec(self).ok()?;
        overwrite_file(stored_index_path, &output).ok()?;
        Some(())
    }
}

use std::collections::BTreeMap;
use crate::types::*;
use crate::types::IndexTag::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Index {
    pub file_info_index: FileMap<StoredFileInfo>,
    pub term_file_index: TermMap<FileMap<Frequency>>,
    pub file_term_index: FileMap<TermMap<Frequency>>
}

impl Index {

    pub fn new() -> Self {
        Self {
            file_info_index: BTreeMap::new(),
            term_file_index: BTreeMap::new(), 
            file_term_index: BTreeMap::new() 
        }
    }

    // add a [term->freq] map corresponding to a new text-processed document
    pub fn add_indexed(&mut self, path: &str, info: FileInfo, term_freq_map: TermMap<Frequency>) -> Option<()>
    {
        // if the path was already present -> failure
        // path must be removed before re-inserting
        let res = self.file_info_index.insert(path.to_string(), (Indexed, info));
        if res.is_some() { return None; }

        for (term, freq) in &term_freq_map {
            // insert the [term->file->freq] mapping in the term_file_index
            let file_freq_map_ref = self.term_file_index
                .entry(term.to_string())
                .or_insert(BTreeMap::new());

            // insert/update the frequency
            *(file_freq_map_ref.entry(path.to_string()).or_insert(0)) += freq;
        }

        // add the passed-in term_freq_map to the file_term_index
        self.file_term_index.insert(path.to_string(), term_freq_map);

        Some(())
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
            for (term, _) in self.file_term_index.get(path)? {
                let file_freq_map_ref = self.term_file_index.get_mut(term)?;
                file_freq_map_ref.remove(path);
    
                // if no more term instances, then remove term
                if file_freq_map_ref.is_empty() {
                    self.term_file_index.remove(term);
                }
            }

            self.file_term_index.remove(path);
        }

        self.file_info_index.remove(path);

        Some(())
    }

    pub fn num_documents(&self) -> usize {
        self.file_term_index.len()
    }

    pub fn num_terms(&self) -> usize {
        self.term_file_index.len()
    }

    // how many times term appears in document
    pub fn tf(&self, term: &str, path: &str) -> Frequency {
        self.term_file_index
            .get(term)
            .and_then(|map| 
                map.get(path)
                .map(|n| *n)
            )
            .unwrap_or(0)
    }

    // how many documents a term appears in
    pub fn df(&self, term: &str) -> Option<Frequency> {
        Some(
            self.term_file_index
            .get(term)?
            .len()
        )
    }
}

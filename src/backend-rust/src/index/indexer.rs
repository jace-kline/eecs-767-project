use std::collections::BTreeMap;
use crate::utils::types::{Term, FilePath, FileMap, TermMap, Frequency};

pub struct Indexer {
    pub term_file_index: TermMap<FileMap<Frequency>>,
    pub file_term_index: FileMap<TermMap<Frequency>>
}

impl Indexer {

    pub fn new() -> Self {
        Self { 
            term_file_index: BTreeMap::new(), 
            file_term_index: BTreeMap::new() 
        }
    }

    // add a [term->freq] map corresponding to a new text-processed document
    pub fn add(&mut self, path: &str, term_freq_map: TermMap<Frequency>) -> Option<()>
    {

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

    // remove a document from the index
    pub fn remove(&mut self, path: &str) -> Option<()> {
        
        for (term, _) in self.file_term_index.get(path)? {
            let file_freq_map_ref = self.term_file_index.get_mut(term)?;
            file_freq_map_ref.remove(path);

            // if no more term instances, then remove term
            if file_freq_map_ref.is_empty() {
                self.term_file_index.remove(term);
            }
        }

        self.file_term_index.remove(path);

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

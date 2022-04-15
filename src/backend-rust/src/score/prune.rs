use crate::types::{FilePath, Index, TermMap, Frequency};

// filters the documents by which to score a query
// selects documents that appear in at least one term's posting list
pub fn prune(index: &Index, term_freq_map: TermMap<Frequency>) -> Vec<FilePath> {
    term_freq_map
    .keys()
    .filter_map(|term| index.frequency_index.term_file_index.get(term))
    .map(|file_freq_map| file_freq_map.keys())
    .flatten()
    .map(|path| path.to_string())
    .collect()
}
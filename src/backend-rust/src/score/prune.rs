use crate::utils::types::{TermMap, Frequency, FilePath};
use crate::index::indexer::Indexer;
use std::collections::BTreeSet;

// filters the documents by which to score a query
// selects documents that appear in at least one term's posting list
pub fn prune(indexer: &Indexer, term_freq_map: TermMap<Frequency>) -> BTreeSet<FilePath> {
    term_freq_map
    .keys()
    .filter_map(|term| indexer.term_file_index.get(term))
    .map(|file_freq_map| file_freq_map.keys())
    .flatten()
    .fold(BTreeSet::new(), |mut docs, doc| {
        docs.insert(doc.to_string());
        docs
    })
}
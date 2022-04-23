use crate::types::*;

pub trait Scorer {
    fn score_query_doc(&self, term_freq_map: &TermMap<Frequency>, doc: &str) -> Score;

    fn score_doc_doc(&self, doc1: &str, doc2: &str) -> Score;
}

use crate::types::*;

pub trait Scorer {

    // score a processed query against a document in the index
    fn score_query_doc(&self, index: &Index, term_freq_map: &TermMap<Frequency>, doc: &str) -> Score;

    // score 2 documents against each other
    fn score_doc_doc(&self, index: &Index, doc1: &str, doc2: &str) -> Score;

    // rank the documents by score in descending order
    fn rank(
        &self,
        index: &Index,
        processed_query: &TermMap<Frequency>,
        num_results: usize
    ) -> Vec<RankResult>;

    // given some "relevant" feedback documents, adjust and re-rank
    fn rank_feedback(
        &self,
        index: &Index,
        processed_query: &TermMap<Frequency>,
        num_results: usize,
        feedback: &[FilePath]
    ) -> Vec<RankResult>;

}

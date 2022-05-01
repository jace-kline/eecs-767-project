use crate::types::*;
use super::document_vector;

pub struct VectorModelScorer {
    pub document_vectors: FileMap<DocumentVector>
}

impl VectorModelScorer {
    pub fn new(index: &Index) -> Self {

        // compute & store document vectors
        let document_vectors = index.frequency_index.file_term_index.iter().map(|(doc, term_freq_map)| {
            let dv = VectorModelScorer::make_document_vector(&index.frequency_index, term_freq_map);
            (doc.to_string(), dv)
        })
        .collect::<FileMap<DocumentVector>>();

        Self {
            document_vectors
        }
    }

    fn make_document_vector(index: &FrequencyIndex, term_freq_map: &TermMap<Frequency>) -> DocumentVector {
        DocumentVector::new(
            term_freq_map.iter().map(|(term, tf)| {
                let df = index.df(&term).unwrap_or(0);
                (term.to_string(), tf_idf_formula(*tf, df, index.num_documents()))
            })
            .collect::<TermMap<Weight>>()
        )
    }

    fn score_document_vector(&self, qv: &DocumentVector, doc: &str) -> Score {
        let dv = self.document_vectors.get(doc);
        // let qv = VectorModelScorer::make_document_vector(&self.index.frequency_index, &term_freq_map);

        if let Some(dv) = dv {
            dv.cosine_similarity(&qv)
        } else {
            0.0
        }
    }
}

impl Scorer for VectorModelScorer {

    fn score_query_doc(&self, index: &Index, term_freq_map: &TermMap<Frequency>, doc: &str) -> Score {
        let qv = VectorModelScorer::make_document_vector(&index.frequency_index, &term_freq_map);

        self.score_document_vector(&qv, doc)
    }

    fn score_doc_doc(&self, index: &Index, doc1: &str, doc2: &str) -> Score {
        let dv1 = self.document_vectors.get(doc1);
        let dv2 = self.document_vectors.get(doc2);

        if let (Some(dv1), Some(dv2)) = (dv1, dv2) {
            dv1.cosine_similarity(dv2)
        } else {
            0.0
        }
    }

    fn rank(
        &self,
        index: &Index,
        processed_query: &TermMap<Frequency>,
        num_results: usize
    ) -> Vec<RankResult> {
        crate::score::rank::rank(
            self,
            index,
            crate::score::prune::prune,
            processed_query,
            num_results
        )
    }

    fn rank_feedback(
        &self,
        index: &Index,
        processed_query: &TermMap<Frequency>, 
        num_results: usize, 
        relevant: &[FilePath]
    ) -> Vec<RankResult> {
        use crate::score::prune::prune;
        use crate::score::rank::rank_truncate_scored;

        let qv = VectorModelScorer::make_document_vector(&index.frequency_index, processed_query);

        // get the document vectors for each "relevant" path
        // simply ignore any unfound path keys
        let relevant_dvs = relevant.iter().filter_map(|path| {
            self.document_vectors.get(path)
        })
        .collect::<Vec<&DocumentVector>>();

        // perform Rocchio vector adjustment on the query vector
        let qv_feedback = document_vector::rocchio(&qv, &relevant_dvs);

        let scored = prune(index, processed_query)
        .into_iter()
        .map(|path| {
            let score = self.score_document_vector(&qv_feedback, &path);
            (path, score)
        })
        .collect::<Vec<(FilePath, Score)>>();

        rank_truncate_scored(scored, num_results)
        .into_iter()
        .map(|(path, score)| RankResult {
            path,
            score
        })
        .collect()

    }


}

pub fn tf_idf_formula(tf: usize, df: usize, n: usize) -> Weight {
    tf as f64 * idf_formula(df, n)
}

pub fn idf_formula(df: usize, n: usize) -> f64 {
    if df == 0 { 0.0 } else { f64::log10(n as f64 / df as f64) }
}
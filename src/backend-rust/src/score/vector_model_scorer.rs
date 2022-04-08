use crate::utils::types::{FileMap, TermMap, Weight, Score, Frequency};
use crate::index::indexer::Indexer;
use super::document_vector::DocumentVector;


pub struct VectorModelScorer<'a> {
    indexer: &'a Indexer,
    document_vectors: FileMap<DocumentVector>
}

impl<'a> VectorModelScorer<'a> {
    pub fn new(indexer: &'a Indexer) -> Self {

        // compute document vectors
        let document_vectors = indexer.file_term_index.iter().map(|(doc, term_freq_map)| {
            let dv = VectorModelScorer::make_document_vector(indexer, term_freq_map);
            (doc.to_string(), dv)
        })
        .collect::<FileMap<DocumentVector>>();

        Self {
            indexer,
            document_vectors
        }
    }

    fn make_document_vector(indexer: &Indexer, term_freq_map: &TermMap<Frequency>) -> DocumentVector {
        DocumentVector::new(
            term_freq_map.iter().map(|(term, tf)| {
                let df = indexer.df(&term).unwrap_or(0);
                (term.to_string(), tf_idf_formula(*tf, df, indexer.num_documents()))
            })
            .collect::<TermMap<Weight>>()
        )
    }

    pub fn score_query_doc(&self, term_freq_map: &TermMap<Frequency>, doc: &str) -> Score {
        let dv = self.document_vectors.get(doc);
        let qv = VectorModelScorer::make_document_vector(self.indexer, &term_freq_map);

        if let Some(dv) = dv {
            dv.cosine_similarity(&qv)
        } else {
            0.0
        }
    }

    pub fn score_doc_doc(&self, doc1: &str, doc2: &str) -> Score {
        let dv1 = self.document_vectors.get(doc1);
        let dv2 = self.document_vectors.get(doc2);

        if let (Some(dv1), Some(dv2)) = (dv1, dv2) {
            dv1.cosine_similarity(dv2)
        } else {
            0.0
        }
    }
}

pub fn tf_idf_formula(tf: usize, df: usize, n: usize) -> Weight {
    tf as f64 * idf_formula(df, n)
}

pub fn idf_formula(df: usize, n: usize) -> f64 {
    if df == 0 { 0.0 } else { f64::log10(n as f64 / df as f64) }
}
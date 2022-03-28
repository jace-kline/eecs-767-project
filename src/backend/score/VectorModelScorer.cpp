#include "VectorModelScorer.h"

VectorModelScorer::VectorModelScorer(const Index& idx) 
: ScorerInterface(idx), tf_formula(TF_IDF::tf_standard), idf_formula(TF_IDF::idf_standard) {
    compute_document_vectors();
}

VectorModelScorer::VectorModelScorer(
    const Index& idx,

    // function pointer for how to compute tf
    std::function<double(frequency_t tf)> tf_func,

    // function pointer for how to compute idf
    std::function<double(size_t N, frequency_t df)> idf_func
) : ScorerInterface(idx), tf_formula(tf_func), idf_formula(idf_func) {
    compute_document_vectors();
}

void VectorModelScorer::compute_document_vectors() {
    // clear document vectors map
    document_vectors.clear();

    // for each [document -> term] mapping...
    for(const auto& doc_terms_pair : index.document_index) {
        
        document_t doc = doc_terms_pair.first;

        // create a map to store [term -> weight] mapping
        std::map<term_t, weight_t> term_weight_map;

        // for each [term -> record] mapping in document_index...
        for(const auto& term_record_pair : doc_terms_pair.second) {

            term_t term = term_record_pair.first;
            const doc_term_record_t& record = term_record_pair.second;

            // compute tf-idf weight of (term, doc)
            frequency_t _tf = record.tf;
            double _df = record.posting_list_ref.df();
            double _w = tf_formula(_tf) * idf_formula(index.num_documents(), _df);

            // store tf-idf weight in the term->weight map
            term_weight_map.insert(term_weight_map.end(), {term, _w});
        }

        // insert doc->DocumentVector mapping
        document_vectors.insert(document_vectors.end(), { doc, DocumentVector(std::move(term_weight_map)) });
    }
}

double VectorModelScorer::score(const std::map<term_t, frequency_t>& query_term_freqs, document_t d) {

    // find document's stored DocumentVector
    // if not found, return 0
    auto it = document_vectors.find(d);
    if(it == document_vectors.end()) return 0.0;

    const DocumentVector& dv = it->second;

    // create a map to store [term -> weight] mapping
    std::map<term_t, weight_t> term_weight_map;

    for(const auto& term_freq_pair : query_term_freqs) {
        term_t term = term_freq_pair.first;
        frequency_t _tf = term_freq_pair.second;

        // compute tf-idf weight of (term, doc)
        double _w = tf_formula(_tf) * idf_formula(index.num_documents(), index.df(term));

        // store tf-idf weight in the term->weight map
        term_weight_map.insert(term_weight_map.end(), {term, _w});
    }

    // construct query's DocumentVector from map
    DocumentVector qv(std::move(term_weight_map));

    return dv.cosine_similarity(qv);
}
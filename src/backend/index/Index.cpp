#include "Index.h"
#include <math.h>

Index::Index() {}

size_t Index::num_documents() const {
    return document_index.size();
}

size_t Index::num_terms() const {
    return term_index.size();
}

frequency_t Index::df(term_t term) const {
    auto it = term_index.find(term);
    return (it == term_index.end() ? 0 : it->second.df());
}

frequency_t Index::tf(term_t term) const {
    auto it = term_index.find(term);
    return (it == term_index.end() ? 0 : it->second.tf());
}

frequency_t Index::tf(term_t term, document_t doc) const {
    auto it = term_index.find(term);
    return (it == term_index.end() ? 0 : it->second.tf(doc));
}

// // could throw div by 0 error if df=0
// double idf_expr(size_t N, frequency_t df) {
//     return log10(N / df);
// }

// // could throw div by 0 error if term doesn't exist
// double Index::idf(term_t term) const {
//     return idf_expr(num_documents(), df(term));
// }

// weight_t Index::w(term_t term, document_t doc) const {
//     // return tf(term, doc) * idf(term);
//     auto it = term_index.find(term);
//     bool term_found = it != term_index.end();

//     // if term not found in inverted index, weight is 0
//     if (!term_found) return 0;

//     // otherwise -> ensures df(term) > 0
//     frequency_t _tf = it->second.tf(doc);
//     frequency_t _df = it->second.df();

//     return ((double) _tf) * idf_expr(num_documents(), _df);
// }

bool Index::update(document_t doc, std::map<term_t, frequency_t>&& term_freq_map) {

    // // move term_record_map into document_index map
    // auto it = document_index.insert(document_index.end(), { doc, std::move(term_record_map) });
    // std::map<term_t, frequency_t>& _map = it->second; // alias

    // set up new map for document in the document_index
    auto doc_index_it = document_index.insert(document_index.end(), { doc, std::map<term_t, doc_term_record_t>() });

    auto help_iter = term_index.begin();

    for(auto pair : term_freq_map) {
        term_t term = pair.first;
        frequency_t freq = pair.second;

        // try to find term in the index
        auto it = term_index.find(term);

        // if term is not in index
        if(it == term_index.end()) {

            // insert the (term, posting list) pair and update the iterator
            it = term_index.insert(help_iter, { term, PostingList() });

            // update the document_term_index
            (doc_index_it->second).insert(doc_index_it->second.end(), { doc, { freq, it->second } });

            // update the help iterator to the spot of the latest term
            help_iter = it;
        }

        // insert the [file -> record] mapping into the posting list map
        (it->second).update(doc, freq);
    }

    return true;
}

// std::map<document_t, DocumentVector> Index::make_document_vectors() const {
//     std::map<document_t, DocumentVector> document_vectors;
//     // IMPLEMENT THIS
// }

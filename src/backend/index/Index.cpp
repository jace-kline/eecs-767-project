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

double Index::idf(term_t term) const {
    return log10(num_documents() / df(term));
}

bool Index::update(document_t doc, std::map<term_t, frequency_t>&& term_record_map) {

    // move term_record_map into document_index map
    auto it = document_index.insert(document_index.end(), { doc, std::move(term_record_map) });
    std::map<term_t, frequency_t>& _map = it->second; // alias

    auto help_iter = term_index.begin();

    for(auto pair : _map) {
        term_t term = pair.first;
        frequency_t freq = pair.second;

        // try to find term in the index
        auto it = term_index.find(term);

        // if term is not in index
        if(it == term_index.end()) {

            // insert the (term, posting list) pair and update the iterator
            it = term_index.insert(help_iter, { term, PostingList() });

            // update the help iterator to the spot of the latest term
            help_iter = it;
        }

        // insert the [file -> record] mapping into the posting list map
        (it->second).update(doc, freq);
    }

    return true;
}


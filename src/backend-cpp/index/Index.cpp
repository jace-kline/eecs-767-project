#include "Index.h"

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
            (doc_index_it->second).insert(doc_index_it->second.end(), { term, { freq, it->second } });

            // update the help iterator to the spot of the latest term
            help_iter = it;
        }

        // insert the [file -> record] mapping into the posting list map
        (it->second).update(doc, freq);
    }

    return true;
}

std::optional<std::set<term_t>> Index::shared_terms(
    const std::set<term_t>& term_set,
    document_t doc
) const {
    
    if(auto _opt = document_terms(doc)) {
        // perform set intersection to get shared terms
        return utils::set_intersection(term_set, _opt.value());
    }

    return std::nullopt;

}

std::set<term_t> Index::all_terms() const {
    return utils::keys(term_index);
}

std::set<document_t> Index::all_documents() const {
    return utils::keys(document_index);
}

std::optional<std::set<term_t>> Index::document_terms(document_t doc) const {
    auto it = document_index.find(doc);
    if(it == document_index.end()) return std::nullopt;

    return utils::keys(it->second);
}

std::optional<std::set<term_t>> Index::term_documents(term_t term) const {
    auto it = term_index.find(term);
    if(it == term_index.end()) return std::nullopt;

    return utils::keys(it->second.document_freq_map);
}

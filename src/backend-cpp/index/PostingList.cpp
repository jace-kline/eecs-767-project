#include "PostingList.h"

PostingList::PostingList() : term_freq(0) {}

frequency_t PostingList::tf() const {
    return this->term_freq;
}

frequency_t PostingList::tf(document_t doc) const {
    auto res = this->document_freq_map.find(doc);

    return (res == this->document_freq_map.end())
    ? 0
    : res->second;
}

frequency_t PostingList::df() const {
    return this->document_freq_map.size();
}

bool PostingList::update(document_t doc, frequency_t freq) {
    // insert [path -> record] into map
    document_freq_map.insert(document_freq_map.end(), { doc, freq });

    // update document frequency and term frequency
    this->term_freq += freq;

    return true;
}
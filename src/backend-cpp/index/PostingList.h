#ifndef POSTING_LIST_H
#define POSTING_LIST_H

#include "../shared/types.h"

class PostingList {
    private:
        frequency_t term_freq;
        std::map<document_t, frequency_t> document_freq_map; // files associated with term

        // allow Index class to access private members
        friend class Index;

    public:
        PostingList();

        // get total term frequency (all documents)
        frequency_t tf() const;

        // get term frequency in a target document
        frequency_t tf(document_t) const;

        // get number of documents that contain term
        frequency_t df() const;

        // add a document-term record to the postings
        bool update(document_t, frequency_t);
};

#endif
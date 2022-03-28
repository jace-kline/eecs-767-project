#ifndef INDEX_H
#define INDEX_H

#include "../shared/types.h"
#include "../shared/utils.h"
#include "PostingList.h"
#include <vector>

// store this in the document_index
// gives us a "shortcut" to posting to access df info
// useful for creating document-term weight vectors
struct doc_term_record_t {
    frequency_t tf;
    const PostingList& posting_list_ref;
};

class Index {

    private:

        // indexes
        std::map<term_t, PostingList> term_index;
        std::map<document_t, std::map<term_t, doc_term_record_t>> document_index;

        // allow scorer modules to directly access indexes
        friend class ScorerInterface;
        friend class VectorModelScorer;

    public:
        Index();

        // number of total docs
        size_t num_documents() const;

        // number of unique terms (normalized)
        size_t num_terms() const;

        // number of documents a given term appears in
        frequency_t df(term_t term) const;

        // number of instances of a term across all docs combined
        frequency_t tf(term_t term) const;

        // number of times a given term appears in a given doc
        frequency_t tf(term_t term, document_t doc) const;

        // // inverse document frequency of given term
        // double idf(term_t term) const;

        // // tf-idf weight of given (term, doc) pair
        // // w(t,d) = tf(t,d) * idf(t)
        // double w(term_t term, document_t doc) const;

        // provide document name, and map of { term -> freq }
        // the map is MOVED to this index and therefore no longer usable by provider (move semantics)
        // return true if update successful, otherwise false
        bool update(document_t, std::map<term_t, frequency_t>&&);

        // // don't call this until finished indexing
        // // produces a map of {document -> document vector}
        // // to be used in similarity calculations/scoring
        // std::map<document_t, DocumentVector> make_document_vectors() const;
};

#endif
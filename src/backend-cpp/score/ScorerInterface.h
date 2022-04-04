#ifndef SCORER_INTERFACE_H
#define SCORER_INTERFACE_H

#include "../shared/types.h"
#include "../index/Index.h"

// Abstract class that defines what a "Scorer" module should implement
class ScorerInterface {
    // accessible by subclasses
    protected:
    const Index& index;

    public:
    // invoked by subclasses with const ref to Index
    ScorerInterface(const Index& idx): index(idx) {}

    // given a parsed query map and document name, return score
    virtual double score(
        const std::map<term_t, frequency_t>& query_term_freqs,
        document_t d
    ) = 0;

    // given two [term->freq] maps, return a score between 2 docs
    virtual double score(
        const std::map<term_t, frequency_t>& lhs_term_freqs,
        const std::map<term_t, frequency_t>& rhs_term_freqs
    ) = 0;

    // // return vector (in best->worst order) of documents
    // // based on similarity of the query to the documents
    // virtual RankingList rank(
    //     const std::map<term_t, frequency_t>& query_term_freqs 
    // ) = 0;
};

#endif
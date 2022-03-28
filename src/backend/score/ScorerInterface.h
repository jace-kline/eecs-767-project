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

    // every scorer module must implement the score() method
    // given a parsed query map and document name, return score
    virtual double score(const std::map<term_t, frequency_t>& query_term_freqs, document_t d) = 0;
};

#endif
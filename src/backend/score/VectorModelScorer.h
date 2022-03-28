#ifndef VECTOR_MODEL_SCORER_H
#define VECTOR_MODEL_SCORER_H

#include "ScorerInterface.h"
#include "TF_IDF.h"
#include "DocumentVector.h"
#include <functional>

class VectorModelScorer : private ScorerInterface {
    private:
    // const Index& index;
    std::map<document_t, DocumentVector> document_vectors;

    protected:
    // helper function to make doc vectors from index
    // called in constructor
    void compute_document_vectors();

    // how to compute tf, idf given params
    std::function<double(frequency_t tf)> tf_formula;
    std::function<double(size_t N, frequency_t df)> idf_formula;

    public:
    // compute the document vectors inside the constructor
    VectorModelScorer(
        const Index& idx,

        // function/lambda for how to compute tf
        std::function<double(frequency_t tf)> tf_func,

        // function/lambda for how to compute idf
        std::function<double(size_t N, frequency_t df)> idf_func
    );

    // if tf, idf formula are assumed to be standard definitions
    VectorModelScorer(const Index& idx);

    // cosine similarity score
    double score(const std::map<term_t, frequency_t>& query_term_freqs, document_t d);

    // // cosine similarity score
    // Ranking ranking(query_t q);
};

#endif
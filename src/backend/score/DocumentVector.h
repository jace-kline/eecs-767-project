#ifndef DOCUMENT_VECTOR_H
#define DOCUMENT_VECTOR_H

#include "../shared/types.h"
#include "../shared/utils.h"
#include <map>
#include <math.h>

class DocumentVector {
    private:
    std::map<term_t, weight_t> term_weight_map;
    double mag;

    protected:
    // called in constructor
    // magnitude = sqrt(sum of squares of all weights)
    double compute_magnitude() const;
    
    public:
    // compute magnitude within constructor
    DocumentVector(std::map<term_t, weight_t>&& term_weights);

    // // vector addition
    // DocumentVector operator+(const DocumentVector& rhs) const;

    // // vector subtraction
    // DocumentVector operator-(const DocumentVector& rhs) const;

    // get the magnitude of the vector
    double magnitude() const;

    // vector dot product
    double dot(const DocumentVector& rhs) const;

    // cosine similarity
    double cosine_similarity(const DocumentVector& rhs) const;
};

#endif
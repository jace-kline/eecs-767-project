#include "DocumentVector.h"

DocumentVector::DocumentVector(std::map<term_t, weight_t>&& term_weights) : term_weight_map(term_weights) {
    magnitude = compute_magnitude();
}

double DocumentVector::compute_magnitude() const {
    unsigned int sum_squares = 0;
    for(const auto& pair : term_weight_map) {
        sum_squares += pow(pair.second, 2);
    }
    
    return sqrt((double) sum_squares);
}

double DocumentVector::operator*(const DocumentVector& rhs) const {
    // start iterators at beginning of each map
    auto it_lhs = this->term_weight_map.begin();
    auto it_rhs = rhs.term_weight_map.begin();

    double agg = 0;
    // while neither iterator has hit the end...
    while(it_lhs != this->term_weight_map.end() && it_rhs != rhs.term_weight_map.end()) {

        // if terms are equal, multiply the weights and add to running sum
        if(it_lhs->first == it_rhs->first) {
            agg += it_lhs->second * it_rhs->second;
            it_lhs++;
            it_rhs++;
        } 
        // if lhs term < rhs term, increment lhs iterator
        else if (it_lhs->first < it_rhs->first) {
            it_lhs++;
        } 
        // otherwise, increment rhs iterator
        else {
            it_rhs++;
        }
    }

    return agg;
}

double DocumentVector::cosine_similarity(const DocumentVector& rhs) const {
    return (*this * rhs) / (this->magnitude * rhs.magnitude);
}
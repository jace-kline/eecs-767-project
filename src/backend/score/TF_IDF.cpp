#include "TF_IDF.h"

double TF_IDF::tf_standard(frequency_t tf) {
    return (double) tf;
}

double TF_IDF::idf_standard(size_t N, frequency_t df) {
    return log10((double) N / (double) df);
}
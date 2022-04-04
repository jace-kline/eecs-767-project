#ifndef TF_IDF_H
#define TF_IDF_H

#include "../shared/types.h"
#include <math.h>

namespace TF_IDF {

double tf_standard(frequency_t tf);

double idf_standard(size_t N, frequency_t df);

}

#endif
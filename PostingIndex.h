#ifndef POSTING_INDEX_H
#define POSTING_INDEX_H

#include "types.h"

class PostingIndex {
private:
    std::map<term_t, posting_t> term_postings_map;

public:
    PostingIndex() {}

    bool indexFile(path_t path);
};

#endif
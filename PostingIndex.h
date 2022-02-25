#ifndef POSTING_INDEX_H
#define POSTING_INDEX_H

#include "types.h"

#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <filesystem>
#include <bits/stdc++.h>

class PostingIndex {
private:
    // private member variables
    std::map<term_t, posting_t> term_postings_map;

public:
    PostingIndex();

    // index a file
    bool postFile(path_t path);
};

#endif
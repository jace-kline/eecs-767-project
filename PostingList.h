#ifndef POSTING_LIST_H
#define POSTING_LIST_H

#include "types.h"

#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <filesystem>
#include <bits/stdc++.h>

class PostingList {
private:
    // private member variables
    std::map<term_t, posting_t> term_postings_map;

public:
    PostingList();

    // update the posting list with term mappings from a document
    bool update(std::map<term_t, file_term_record_t> &term_file_map);
};

#endif
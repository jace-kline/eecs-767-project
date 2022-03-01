#ifndef INVERTED_INDEX_H
#define INVERTED_INDEX_H

#include "types.h"
#include "filesystem.h"
#include "file_processing.h"

// all the information that a given term is mapped to
// includes document frequency (df), term frequency (tf), and a map of file paths to records
struct posting_list_t {
    unsigned int df; // document frequency
    unsigned int tf; // term frequency
    std::map<path_t, file_term_record_t> file_record_map; // files associated with term

    bool update(path_t path, file_term_record_t record);
};

struct inverted_index_t {
    // maps terms to posting lists
    std::map<term_t, posting_list_t> index;

    bool update(path_t path, std::map<term_t, file_term_record_t> &term_file_map);
};

inverted_index_t buildInvertedIndex(path_t rootPath);

#endif
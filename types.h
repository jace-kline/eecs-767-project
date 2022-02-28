#ifndef TYPES_H
#define TYPES_H

#include <string>

typedef std::string term_t;
typedef std::string path_t;

// information about an instance of a term located within a file
struct term_instance_t {
    term_t term;
    unsigned int loc;
};

// information about a term, its frequency, and its locations within a file
struct file_term_record_t {
    unsigned int freq;
    // std::vector<unsigned int> locs;
};

// all the information that a given term is mapped to
// includes document frequency (df), term frequency (tf), and a map of file paths to records
struct posting_t {
    unsigned int df; // document frequency
    unsigned int tf; // term frequency
    std::map<path_t, file_term_record_t> file_record_map; // files associated with term
};

#endif


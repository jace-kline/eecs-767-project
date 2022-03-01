#ifndef TYPES_H
#define TYPES_H

#include <string>
#include <map>

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

#endif


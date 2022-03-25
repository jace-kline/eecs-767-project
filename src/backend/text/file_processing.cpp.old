#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <filesystem>
#include <bits/stdc++.h>

#include "types.h"
#include "filesystem.h"

// bool isAsciiTextFile(std::string path) {
//     std::ifstream fs(path);

//     if(!fs) return false; // If can't create stream, return false

//     char c;
//     while((fs.get(c), fs.eof()) == false) { // Iterate over each character in stream
//         if(c < 9 || c > 127) { // If read char is not within ASCII range 9-127, then assume file is not ASCII text
//             fs.close(); 
//             return false; 
//         }
//     }

//     fs.close();
//     return true;
// }

bool isToken(char c) {
    return ((c >= 48 && c <= 57) // c in {0..9}
        || (c >= 65 && c <= 90) // c in {A..Z}
        || (c >= 97 && c <= 122)); // c in {a..z}  
}

std::string normalize(std::string term) {
    std::transform(term.begin(), term.end(), term.begin(),
        [](unsigned char c){ return std::tolower(c); });
    return term;
}

std::vector<term_instance_t> collectTerms(std::ifstream &fs) {

    // collect terms
    std::vector<term_instance_t> terminstances;

    char c;
    std::string term;
    unsigned loc = 0;

    while((fs.get(c), fs.eof()) == false) { // Iterate over each character in stream
        if(isToken(c)) {
            term.push_back(c);
        } else if (!term.empty()) {
            terminstances.push_back({ normalize(term), loc });
            term.clear();
        }
        loc++;
    }
    fs.close();

    // if all pass, then return (move semantics)
    return terminstances;
}

std::map<std::string, file_term_record_t> buildFileTermRecords(std::vector<term_instance_t>& terminstances) {
    std::map<std::string, file_term_record_t> term_file_map;
    for(term_instance_t t : terminstances) {
        auto iter = term_file_map.find(t.term);
        if(iter != term_file_map.end()) {
            (iter->second).freq++;
        } else {
            file_term_record_t record = { 1 };
            term_file_map.insert(std::pair{ t.term, record });
        }
    }

    return term_file_map;
}

std::optional<std::map<term_t, file_term_record_t>> processFile(std::string path) {

    // try to create input file stream for provided path
    std::ifstream fs(path);
    if(!fs) return std::nullopt;

    // collect term instances
    std::vector<term_instance_t> terminstances = collectTerms(fs);

    // check distribution of terms to see if its a likely text file?

    // build records for each term in this file & return
    return buildFileTermRecords(terminstances);
}

class FileIterator {
    private:
    std::filesystem::recursive_directory_iterator file_iter;

    public:
    FileIterator(path_t rootPath) 
    : file_iter(std::filesystem::recursive_directory_iterator(rootPath)) {}

    // fetches next file
    // returns nullopt if end of recursion
    std::optional<path_t> next() {
        if(file_iter == end(file_iter)) return std::nullopt;
        while(!file_iter->is_regular_file()) {
            if(file_iter == end(file_iter)) return std::nullopt;
            file_iter++;
        }
        return file_iter->path();
    }
};
#include "FileIndexer.h"
#include "utils.h"

// return false if file is not readable OR not to be considered in indexing
// must supply a path to the file and a reference to a vector to store the terms (strings + locations)
bool FileIndexer::collectFileTermInstances() {
    std::ifstream fs(path);

    if(!fs) return false; // If can't create stream, return false

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
    return true;
}

void FileIndexer::buildFileTermRecords() {
    for(term_instance_t t : terminstances) {
        auto iter = term_file_map.find(t.term);
        if(iter != term_file_map.end()) {
            (iter->second).freq++;
        } else {
            file_term_record_t record = { 1 };
            term_file_map.insert(std::pair{ t.term, record });
        }
    }
}

std::optional<file_record_map_t&> FileIndexer::indexFile() {
    if(collectFileTermInstances()) {
        buildFileTermRecords();
        return term_file_map;
    }

    return std::nullopt;
}
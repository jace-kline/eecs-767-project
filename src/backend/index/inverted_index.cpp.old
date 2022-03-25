#include "inverted_index.h"

bool posting_list_t::update(path_t path, file_term_record_t record) {
    // insert [path -> record] into map
    file_record_map.insert(file_record_map.end(), { path, record });

    // update document frequency and term frequency
    df++;
    tf += record.freq;

    return true;
}


bool inverted_index_t::update(path_t path, std::map<term_t, file_term_record_t> &term_file_map) {

    std::map<term_t, posting_list_t>::iterator help_iter = index.begin();

    for(auto pair : term_file_map) {
        term_t term = pair.first;
        file_term_record_t record = pair.second;

        // try to find term in the index
        auto it = index.find(term);

        // if term is not in index
        if(it == index.end()) {
            // create new posting list for the term
            posting_list_t posting_list = {
                0, // df
                0, // tf
                std::map<path_t, file_term_record_t>() // empty map
            };

            // insert the term and update the iterator
            it = index.insert(help_iter, { term, posting_list });

            // update the help iterator to the spot of the latest term
            help_iter = it;
        }

        // insert the [file -> record] mapping into the posting list map
        // give the iterator "hint" as the end of the map
        (it->second).update(path, record);
    }

    return true;
}

inverted_index_t indexFilesystem(path_t rootPath) {

    // recurse over filesystem and gather all file paths (ordered by path string)
    std::set<path_t> paths = scrapeFilePaths(rootPath);

    return buildInvertedIndex(paths);

}

inverted_index_t buildInvertedIndex(std::set<path_t> &paths) {
    // initialize an empty inverted index
    inverted_index_t index;

    for(path_t path : paths) {
        std::optional<std::map<term_t, file_term_record_t>> valid_result;
        if(valid_result = processFile(path)) {
            std::map<term_t, file_term_record_t> term_file_map = valid_result.value();

            index.update(path, term_file_map);
        }
    }

    return index;
}
#ifndef FILE_INDEXER_H
#define FILE_INDEXER_H

#include "types.h"

#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <filesystem>
#include <bits/stdc++.h>
#include <optional>

class FileIndexer {
private:
    path_t path;
    std::vector<term_instance_t> terminstances;
    file_record_map_t term_file_map;

    // private methods
    bool collectFileTermInstances();
    void buildFileTermRecords();

public:
    FileIndexer(path_t path);

    // return false if file is not text or cannot be read
    std::optional<file_record_map_t&> indexFile();

};

#endif
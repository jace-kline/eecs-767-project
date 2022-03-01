#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <set>
#include <filesystem>
#include <bits/stdc++.h>
#include <ctime>

#include "types.h"

// returns a std::set of sorted file path strings
std::set<path_t> scrapeFilePaths(path_t rootPath) {
    std::set<path_t> paths;

    for(const std::filesystem::directory_entry& dentry : std::filesystem::recursive_directory_iterator(rootPath)) {
        std::filesystem::path path = dentry.path();
        if(dentry.is_regular_file()) {
            // inserts in correct order -> O(log n)
            paths.insert(path);
        }
    }

    return paths;
}
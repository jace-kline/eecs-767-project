#include "Scraper.h"

std::set<path_t> Scraper::scrapePaths(path_t rootPath) {
    std::set<document_t> paths;

    for(const std::filesystem::directory_entry& dentry : std::filesystem::recursive_directory_iterator(rootPath)) {
        std::filesystem::path path = dentry.path();
        if(dentry.is_regular_file()) {
            // inserts in correct order -> O(log n)
            paths.insert(path);
        }
    }

    return paths;
}
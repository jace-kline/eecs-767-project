#include <map>
#include <ctime>
#include <iostream>
#include "scrape/Scraper.h"
#include "text/TextProcessor.h"
#include "index/Index.h"
#include "shared/utils.h"

void test_scrapePaths(path_t rootPath) {
    clock_t taken = clock();
    std::set<path_t> paths = Scraper::scrapePaths(rootPath);
    for(path_t path : paths) {
        std::cout << path << std::endl;
    }
    taken = clock() - taken;

    std::cout << taken << std::endl;
    std::cout << paths.size() << std::endl;
}

void test_processFile(path_t path) {
    if(auto _map = TextProcessor::processFile(path)) {
        for (auto pair : _map.value()) {
            std::cout << pair.first << " -> " << pair.second << std::endl;
        }
    }
}

void test_processQuery(query_t query) {
    auto _map = TextProcessor::processQuery(query);
    // for (auto pair : _map) {
    //     std::cout << pair.first << " -> " << pair.second << std::endl;
    // }
    output_map(_map, 0, std::cout);
}

Index buildIndex(path_t rootPath) {
    Index index;

    auto paths = Scraper::scrapePaths(rootPath);
    for(path_t path : paths) {
        if(auto _map = TextProcessor::processFile(path)) {
            index.update(path, std::move(_map.value()));
        }
    }
    return index;
}

void test_buildIndex(path_t rootPath) {
    Index index = buildIndex(rootPath);
    std::cout << "documents: " << index.num_documents() << std::endl;
    std::cout << "terms: " << index.num_terms() << std::endl;
}

// void test_indexFilesystem(path_t rootPath) {

//     inverted_index_t inv_index = indexFilesystem(rootPath);

//     for (auto pair : inv_index.index) {
//         term_t term = pair.first;
//         posting_list_t plist = pair.second;

//         std::cout << term 
//         << " (df = " << plist.df << ")"
//         << " (tf = " << plist.tf << ")" << std::endl;

//         for (auto pair2 : plist.file_record_map) {
//             std::cout << '\t' << pair2.first << std::endl;
//         }
//     }
// }


int main() {
    // test_indexFilesystem("/home/jacekline/dev/eecs-767/eecs-767-project");
    // test_processFile("/home/jacekline/dev/eecs-767/eecs-767-project/stories/3wishes.txt");
    // test_processQuery("once upon a time there was a big bad wolf. You're a nice fellow. TIME goes on. HELLO WOLF.");
    test_buildIndex("/home/jacekline/dev/eecs-767/eecs-767-project/stories");
    return 0;
}
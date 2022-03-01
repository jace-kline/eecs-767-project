#include <map>
#include <ctime>
#include "filesystem.h"
#include "inverted_index.h"

void test_scrapeFilePaths() {
    path_t rootPath = "/home/jacekline/dev/";

    clock_t taken = clock();
    std::set<path_t> paths = scrapeFilePaths(rootPath);
    taken = clock() - taken;

    std::cout << taken << std::endl;
    std::cout << paths.size() << std::endl;
}

void test_iterMapHint() {
    std::map<int, int> map_hint, map_normal;
    clock_t t_hint, t_normal;

    t_hint = clock();
    for(int i = 0; i < 1000000; i++) {
        map_hint.insert(map_hint.end(), { i, i });
    }
    t_hint = clock() - t_hint;

    t_normal = clock();
    for(int i = 0; i < 1000000; i++) {
        map_normal.insert({i, i});
    }
    t_normal = clock() - t_normal;

    std::cout << t_hint << std::endl;
    std::cout << t_normal << std::endl;
}

void test_indexFilesystem(path_t rootPath) {

    inverted_index_t inv_index = indexFilesystem(rootPath);

    for (auto pair : inv_index.index) {
        term_t term = pair.first;
        posting_list_t plist = pair.second;

        std::cout << term 
        << " (df = " << plist.df << ")"
        << " (tf = " << plist.tf << ")" << std::endl;

        for (auto pair2 : plist.file_record_map) {
            std::cout << '\t' << pair2.first << std::endl;
        }
    }
}

void test_scrapeFilePaths(path_t rootPath) {
    auto paths = scrapeFilePaths(rootPath);

    for(path_t path : paths) {
        std::cout << path << std::endl;
    }
}

int main() {
    test_indexFilesystem("/home/jacekline/dev/eecs-767/eecs-767-project");
    // test_scrapeFilePaths("/home/jacekline/dev/eecs-767/");
    return 0;
}
#include "filesystem.h"

int main() {
    path_t rootPath = "/home/jacekline/dev/";

    time_t start, end;

    time(&start);
    std::set<path_t> paths = scrapeFilePaths(rootPath);
    time(&end);

    std::cout << difftime(end, start) << std::endl;
    std::cout << paths.size() << std::endl;

    return 0;
}
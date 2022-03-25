#ifndef SCRAPER_H
#define SCRAPER_H

#include <set>
#include <filesystem>
#include "../shared/types.h"

class Scraper {
    private:
        Scraper() {}
    public:
        static std::set<path_t> scrapePaths(path_t rootPath);
};

#endif
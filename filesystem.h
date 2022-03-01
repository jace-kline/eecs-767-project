#ifndef FILESYSTEM_H
#define FILESYSTEM_H

#include <iostream>
#include <string>
#include <set>
#include <filesystem>
#include <ctime>

#include "types.h"

std::set<path_t> scrapeFilePaths(path_t rootPath);

#endif
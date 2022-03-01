#ifndef FILE_PROCESSING_H
#define FILE_PROCESSING_H

#include <fstream>
#include <string>
#include <vector>
#include <map>

#include "types.h"

bool isToken(char c);

term_t normalize(term_t term);

std::vector<term_instance_t> collectTerms(std::ifstream &fs);

std::map<term_t, file_term_record_t> buildFileTermRecords(std::vector<term_instance_t>& terminstances);

// return nullopt if not a valid/openable text file
std::optional<std::map<term_t, file_term_record_t>> processFile(path_t path);

#endif
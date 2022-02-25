#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <filesystem>
#include <bits/stdc++.h>

#include "types.h"

bool isAsciiTextFile(std::string path) {
    std::ifstream fs(path);

    if(!fs) return false; // If can't create stream, return false

    char c;
    while((fs.get(c), fs.eof()) == false) { // Iterate over each character in stream
        if(c < 9 || c > 127) { // If read char is not within ASCII range 9-127, then assume file is not ASCII text
            fs.close(); 
            return false; 
        }
    }

    fs.close();
    return true;
}

bool isToken(char c) {
    return ((c >= 48 && c <= 57) // c in {0..9}
        || (c >= 65 && c <= 90) // c in {A..Z}
        || (c >= 97 && c <= 122)); // c in {a..z}  
}

std::string normalize(std::string term) {
    std::transform(term.begin(), term.end(), term.begin(),
        [](unsigned char c){ return std::tolower(c); });
    return term;
}

// return false if file is not readable OR not to be considered in indexing
// must supply a path to the file and a reference to a vector to store the terms (strings + locations)
bool collectFileTermInstances(std::string path, std::vector<term_instance_t>& terminstances) {
    std::ifstream fs(path);

    if(!fs) return false; // If can't create stream, return false

    char c;
    std::string term;
    unsigned loc = 0;

    while((fs.get(c), fs.eof()) == false) { // Iterate over each character in stream
        if(isToken(c)) {
            term.push_back(c);
        } else if (!term.empty()) {
            terminstances.push_back({ normalize(term), loc });
            term.clear();
        }
        loc++;
    }

    fs.close();
    return true;
}

void buildFileTermRecords(std::vector<term_instance_t>& terminstances, std::map<std::string, file_term_record_t>& term_file_map) {
    for(term_instance_t t : terminstances) {
        auto iter = term_file_map.find(t.term);
        if(iter != term_file_map.end()) {
            (iter->second).freq++;
        } else {
            file_term_record_t record = { 1 };
            term_file_map.insert(std::pair{ t.term, record });
        }
    }
}

// void indexFile(std::string path) {
//     std::vector<term_instance_t> terminstances;
//     std::map<std::string, file_term_record_t> term_file_map;

//     bool validfile = collectFileTermInstances(path, terminstances);
//     if (validfile) buildFileTermRecords(terminstances, term_file_map);
// }

void collectFileTermInstances_test() {
    std::string path = "./scrape.cpp";
    std::vector<term_instance_t> terminstances;
    std::map<std::string, file_term_record_t> term_file_map;

    bool validfile = collectFileTermInstances(path, terminstances);
    if (validfile) buildFileTermRecords(terminstances, term_file_map);

    for(auto pair : term_file_map) {
        std::cout << pair.first << " " << pair.second.freq << std::endl;
    }
}

void listFiles(std::string rootPath) {
    for(const std::filesystem::directory_entry& dentry : std::filesystem::recursive_directory_iterator(rootPath)) {
        std::filesystem::path path = dentry.path();
        if(!dentry.is_regular_file()) {
            std::cout << "Directory: " << dentry << std::endl;
        } else {
            std::cout << "Filename: " << path.filename() 
            << " extension: " << path.extension() 
            << " textfile?: " << isAsciiTextFile(path)
            << std::endl;
        }
    }
}

int main() {
    // listFiles("/home/jacekline/dev/eecs-755");
    // std::cout << isAsciiTextFile("./scrape.cpp") << std::endl;
    collectFileTermInstances_test();
    return 0;
}
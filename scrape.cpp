#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <filesystem>

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

// return false if file is not readable OR not to be considered in indexing
// must supply a path to the file and a reference to a vector to store the terms (strings)
bool collectFileTerms(std::string path, std::vector<std::string>& terms) {
    std::ifstream fs(path);

    if(!fs) return false; // If can't create stream, return false

    char c;
    std::string term;
    while((fs.get(c), fs.eof()) == false) { // Iterate over each character in stream
        if(isToken(c)) {
            term.push_back(c);
        } else if (!term.empty()) {
            terms.push_back(term);
            term.clear();
        }
    }
    fs.close();
    return true;
}

void collectFileTerms_test() {
    std::vector<std::string> terms;
    collectFileTerms("./scrape.cpp", terms);

    for(std::string term : terms) {
        std::cout << term << std::endl;
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
    collectFileTerms_test();
    return 0;
}
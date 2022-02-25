#include "utils.h"

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
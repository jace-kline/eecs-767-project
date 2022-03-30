#include "TextProcessor.h"

bool TextProcessor::is_token_char(char c) {
    return ((c >= 48 && c <= 57) // c in {0..9}
        || (c >= 65 && c <= 90) // c in {A..Z}
        || (c >= 97 && c <= 122)); // c in {a..z}
}

std::optional<term_t> TextProcessor::normalize(token_t token) {
    std::transform(token.begin(), token.end(), token.begin(),
        [](unsigned char c){ return std::tolower(c); });
    return token;
}

std::optional<token_t> TextProcessor::next_token(std::istream& is) {
    char c;
    token_t token;

    while((is.get(c), is.eof()) == false) { // Iterate over each character in stream
        if(is_token_char(c)) {
            token.push_back(c);
        } else if (!token.empty()) {
            return token;
        }
    }

    // in the case we have a token built up when we hit EOF
    if(!token.empty()) return token;

    return std::nullopt;
}

std::map<term_t, frequency_t> TextProcessor::process(std::istream& is) {
    std::map<term_t, frequency_t> term_freq_map;

    std::optional<token_t> token;
    std::optional<term_t> term;

    // parse next token
    while (token = next_token(is)) {

        // normalize token -> term
        if(term = normalize(token.value())) {
            
            // insert new term OR update existing term record
            auto it = term_freq_map.find(term.value());

            // if term not found in map, insert with freq=1
            if (it == term_freq_map.end()) {
                term_freq_map.insert({ term.value(), 1 });
            } else { // otherwise, increment existing freq
                it->second++;
            }
        }
    }

    return term_freq_map;
}

std::optional<std::map<term_t, frequency_t>> TextProcessor::processFile(path_t path) {
    std::ifstream is(path);
    if(!is) return std::nullopt;

    auto res = process(is);
    is.close();
    return res;
}

std::map<term_t, frequency_t> TextProcessor::processQuery(query_t query) {
    std::istringstream is(query);
    return process(is);
}
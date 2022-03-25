#ifndef TEXT_PROCESSOR_H
#define TEXT_PROCESSOR_H

#include <map>
#include <vector>
#include <bits/stdc++.h>
#include "../shared/types.h"

class TextProcessor {

    private:
    TextProcessor() {}

    public:
    // does input char belong in a token?
    static bool is_token_char(char c);

    // take a token, return a normalized term
    // returns std::nullopt if term should be removed
    static std::optional<term_t> normalize(token_t token);

    // parse the next unnormalized token candidate from an input stream
    // returns std::nullopt if EOF reached
    static std::optional<token_t> next_token(std::istream& stream);

    // // istream -> repeat { collect token -> normalize -> term }
    // static std::vector<term_t> tokenize(std::istream& stream);

    // take an istream and collect the { term -> record } mappings
    static std::map<term_t, frequency_t> process(std::istream& stream);

    // wrapper to read file as ifstream & process as a document
    // returns std::nullopt if file can't be opened
    static std::optional<std::map<term_t, frequency_t>> processFile(path_t path);

    // wrapper to read string as isstream & process as a document
    static std::map<term_t, frequency_t> processQuery(query_t query);
};

#endif
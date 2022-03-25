#ifndef UTILS_H
#define UTILS_H

#include <map>
#include <iostream>

template <typename T>
concept Printable = requires(T v) {
    { std::cout << v };
};

template <Printable K, Printable V>
std::ostream& output_map(std::map<K,V>& map, unsigned int indent, std::ostream& os) {
    for(const auto& pair : map) {
        os << std::endl;
        for(unsigned int i = 0; i < indent; i++) {
            os << " ";
        }
        os << pair.first << " -> " << pair.second;
    }
    return os;
}

#endif
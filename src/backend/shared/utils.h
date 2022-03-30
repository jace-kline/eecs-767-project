#ifndef UTILS_H
#define UTILS_H

#include <vector>
#include <map>
#include <set>
#include <tuple>
#include <iostream>

namespace utils {
// finds matching keys from 2 maps
// returns vector of triples (K, T1, T2)
// ideally use when T1, T2 are low-cost copies
template <typename K, typename T1, typename T2>
std::vector<std::tuple<K, T1, T2>> merge_maps_on_keys(
    const std::map<K, T1>& mapl,
    const std::map<K, T2>& mapr
) {
    // create accumulation vector for matching keys
    std::vector<std::tuple<K, T1, T2>> matches(std::min(mapl.size(), mapr.size()));

    // start iterators at beginning of each map
    auto it_lhs = mapl.begin();
    auto it_rhs = mapr.begin();

    // while neither iterator has hit the end...
    while(it_lhs != mapl.end() && it_rhs != mapr.end()) {

        // if terms are equal, multiply the weights and add to running sum
        if(it_lhs->first == it_rhs->first) {
            matches.push_back({it_lhs->first, it_lhs->second, it_rhs->second});
            it_lhs++;
            it_rhs++;
        } 
        // if lhs term < rhs term, increment lhs iterator
        else if (it_lhs->first < it_rhs->first) {
            it_lhs++;
        } 
        // otherwise, increment rhs iterator
        else {
            it_rhs++;
        }
    }

    return matches;
};

template <typename K>
std::set<K> set_intersection(
    const std::set<K>& setl,
    const std::set<K>& setr
) {
    // create accumulation vector for matching keys
    std::set<K> matches;

    // start iterators at beginning of each map
    auto it_lhs = setl.begin();
    auto it_rhs = setr.begin();

    // while neither iterator has hit the end...
    while(it_lhs != setl.end() && it_rhs != setr.end()) {

        // if terms are equal, multiply the weights and add to running sum
        if(*it_lhs == *it_rhs) {
            matches.insert(matches.end(), *it_lhs);
            it_lhs++;
            it_rhs++;
        } 
        // if lhs term < rhs term, increment lhs iterator
        else if (*it_lhs < *it_rhs) {
            it_lhs++;
        } 
        // otherwise, increment rhs iterator
        else {
            it_rhs++;
        }
    }

    return matches;
};


template <typename K, typename V>
std::set<K> keys(const std::map<K,V>& _map) {
    std::set<K> result_set;
    for(const auto& pair : _map) {
        result_set.insert(result_set.end(), pair.first);
    }
    return result_set;
}

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
};

}

#endif
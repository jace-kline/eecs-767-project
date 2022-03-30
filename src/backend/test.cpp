#include <map>
#include <ctime>
#include <iostream>
#include "scrape/Scraper.h"
#include "text/TextProcessor.h"
#include "index/Index.h"
#include "shared/utils.h"
#include "score/VectorModelScorer.h"

void test_scrapePaths(path_t rootPath) {
    clock_t taken = clock();
    std::set<path_t> paths = Scraper::scrapePaths(rootPath);
    for(path_t path : paths) {
        std::cout << path << std::endl;
    }
    taken = clock() - taken;

    std::cout << taken << std::endl;
    std::cout << paths.size() << std::endl;
}

void test_processFile(path_t path) {
    if(auto _map = TextProcessor::processFile(path)) {
        for (auto pair : _map.value()) {
            std::cout << pair.first << " -> " << pair.second << std::endl;
        }
    }
}

void test_processQuery(query_t query) {
    auto _map = TextProcessor::processQuery(query);
    // for (auto pair : _map) {
    //     std::cout << pair.first << " -> " << pair.second << std::endl;
    // }
    utils::output_map(_map, 0, std::cout);
}

Index buildIndex(path_t rootPath) {
    Index index;

    auto paths = Scraper::scrapePaths(rootPath);
    for(path_t path : paths) {
        if(auto _map = TextProcessor::processFile(path)) {
            index.update(path, std::move(_map.value()));
        }
    }
    return index;
}

void test_buildIndex(path_t rootPath) {
    Index index = buildIndex(rootPath);
    std::cout << "documents: " << index.num_documents() << std::endl;
    std::cout << "terms: " << index.num_terms() << std::endl;

    document_t doc = "/home/jacekline/dev/eecs-767/eecs-767-project/stories/3lpigs.txt";
    term_t term = "pig";
    std::cout << index.tf(term, doc) << std::endl;
}

void test_VectorModelScorer() {
    path_t rootPath = "/home/jacekline/dev/eecs-767/eecs-767-project/stories";
    document_t doc = "/home/jacekline/dev/eecs-767/eecs-767-project/stories/3lpigs.txt";
    term_t query = "three little pig";

    Index index = buildIndex(rootPath);
    VectorModelScorer scorer = VectorModelScorer(index);

    auto query_map = TextProcessor::processQuery(query);
    std::set<term_t> query_terms = utils::keys(query_map);

    // for(document_t doc : index.all_documents()) {
    //     std::cout << doc << std::endl;
    // }

    // for(term_t term : index.all_terms()) {
    //     std::cout << term << std::endl;
    // }

    // for(term_t term : query_terms) {
    //     std::cout << term << std::endl;
    // }

    // if(auto _opt = index.document_terms(doc)) {
    //     for(term_t term : _opt.value()) {
    //         std::cout << term << std::endl;
    //     }
    // }

    std::cout << "Shared terms:" << std::endl;
    if(auto _opt = index.shared_terms(query_terms, doc)) {
        for(term_t term : _opt.value()) {
            std::cout << term << std::endl;
        }
    }

    std::cout << "Score: " << scorer.score(query_map, doc) << std::endl;
}

void test_VectorModelScorer2() {

    path_t rootPath = "/home/jacekline/dev/eecs-767/eecs-767-project/stories";
    document_t doc = "/home/jacekline/dev/eecs-767/eecs-767-project/stories/3lpigs.txt";
    term_t query = "three little pig";

    Index index = buildIndex(rootPath);
    VectorModelScorer scorer = VectorModelScorer(index);

    std::map<term_t, frequency_t> query_map = TextProcessor::processQuery(query);

    const DocumentVector* d_ptr = nullptr;
    if(auto _opt = scorer.get_document_vector(doc)) {
        d_ptr = _opt.value();
    } else {
        std::cout << "Invalid document name." << std::endl;
        return;
    }
    const DocumentVector& d = *d_ptr;
    DocumentVector q = scorer.compute_document_vector(query_map);

    std::cout << "d mag: " << d.magnitude() << std::endl;
    std::cout << "q mag: " << q.magnitude() << std::endl;
    std::cout << "dot(d, q) = " << d.dot(q) << std::endl;
    std::cout << "cosine_similarity(d, q) = " << d.cosine_similarity(q) << std::endl;
}

void test_VectorModelScorer3() {

    path_t rootPath = "/home/jacekline/dev/eecs-767/eecs-767-project/stories";
    query_t d1 = "three little wolf";
    query_t d2 = "three little pig";

    Index index = buildIndex(rootPath);
    VectorModelScorer scorer = VectorModelScorer(index);

    std::map<term_t, frequency_t> m1 = TextProcessor::processQuery(d1);
    std::map<term_t, frequency_t> m2 = TextProcessor::processQuery(d2);

    auto merged_maps = utils::merge_maps_on_keys<term_t, frequency_t, frequency_t>(m1, m2);
    std::cout << "Matched terms: " << std::endl;
    for (const auto& triple : merged_maps) {
        std::cout << "(" 
        << std::get<0>(triple) << ", " 
        << std::get<1>(triple) << ", "
        << std::get<2>(triple)
        << ")" << std::endl;
    }

    DocumentVector d1v = scorer.compute_document_vector(m1);
    DocumentVector d2v = scorer.compute_document_vector(m2);

    std::cout << "d1v mag: " << d1v.magnitude() << std::endl;
    std::cout << "d2v mag: " << d2v.magnitude() << std::endl;
    std::cout << "dot(d1v, d2v) = " << d1v.dot(d2v) << std::endl;
    std::cout << "cosine_similarity(d1v, d2v) = " << d1v.cosine_similarity(d2v) << std::endl;
}

void test_DocumentVectors() {
    std::map<term_t, weight_t> map1 = {
        {"t1", 0.1},
        {"t2", 0.2},
        {"t3", 0.3},
        {"t4", 0.4},
        {"t6", 0.6}
    };

    std::map<term_t, weight_t> map2 = {
        {"t1", 0.1},
        {"t2", 0.2},
        {"t3", 0.3},
        {"t5", 0.5},
        {"t7", 0.7}
    };

    DocumentVector vec1(std::move(map1));
    DocumentVector vec2(std::move(map2));

    std::cout << "vec1 mag: " << vec1.magnitude() << std::endl;
    std::cout << "vec2 mag: " << vec2.magnitude() << std::endl;
    std::cout << "dot(vec1, vec2) = " << vec1.dot(vec2) << std::endl;
    std::cout << "cosine_similarity(vec1, vec2) = " << vec1.cosine_similarity(vec2) << std::endl;
}



// void test_indexFilesystem(path_t rootPath) {

//     inverted_index_t inv_index = indexFilesystem(rootPath);

//     for (auto pair : inv_index.index) {
//         term_t term = pair.first;
//         posting_list_t plist = pair.second;

//         std::cout << term 
//         << " (df = " << plist.df << ")"
//         << " (tf = " << plist.tf << ")" << std::endl;

//         for (auto pair2 : plist.file_record_map) {
//             std::cout << '\t' << pair2.first << std::endl;
//         }
//     }
// }


int main() {
    // test_scrapePaths("/home/jacekline/dev/eecs-767/eecs-767-project/stories");
    // test_processFile("/home/jacekline/dev/eecs-767/eecs-767-project/stories/3wishes.txt");
    // test_processQuery("once upon a time there was a big bad wolf. You're a nice fellow. TIME goes on. HELLO WOLF.");
    // test_buildIndex("/home/jacekline/dev/eecs-767/eecs-767-project/stories");
    test_VectorModelScorer();
    // test_DocumentVectors();
    // test_VectorModelScorer2();
    // test_VectorModelScorer3();
    return 0;
}
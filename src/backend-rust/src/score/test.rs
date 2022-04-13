use crate::types::*;
use crate::text::text_process;
use crate::score::vector_model::vector_model_scorer::idf_formula;
use crate::score::prune::prune;
use std::iter::zip;


#[test]
fn test_vector_model() {
    // let mut index = Index::new();

    // let files = vec![
    //     ("file1.txt", "Once upon a time there was a squirrel squirrel squirrel"),
    //     ("file2.txt", "The squirrel jumped out of the tree"),
    //     ("file3.txt", "The forest was full full of tree and squirrel")
    // ];

    // // add text-processed files to the map
    // for (path, contents) in files.iter() {
    //     let processed = text_process(*contents).expect("Text processing failed");

    //     index.add(path, processed);
    // }

    // let n = index.num_documents();
    // println!("num_docs = {}", n);
    // println!("num_terms = {}", index.num_terms());

    // let query = "squirrel forest tree";
    // let processed_query_map = text_process(query).expect("Text processing failed");
    // println!("query = '{}'", query);

    // for (term, freq) in processed_query_map.iter() {
    //     let df = index.df(&term).expect("Term not in index");
    //     println!("df({:?}) = {:?}", &term, df);
    //     println!("idf({:?}) = {:?}", &term, idf_formula(df, n));
    //     for (path, _) in files.iter() {
    //         println!("tf({:?}, query) = {:?}", &term, freq);
    //         println!("tf({:?}, {:?}) = {:?}", &term, *path, index.tf(&term, *path));
    //     }
    // }

    // let vm_scorer = VectorModelScorer::new(&index);

    // for (path, _) in files.iter() {
    //     println!("cosine_similarity(query, {:?}) = {:?}", *path, vm_scorer.score_query_doc(&processed_query_map, *path));
    // }

}

#[test]
fn test_prune() {
    // let mut index = Index::new();

    // let files = vec![
    //     ("file1.txt", "Once upon a time there was a squirrel squirrel squirrel"),
    //     ("file2.txt", "The squirrel jumped out of the tree"),
    //     ("file3.txt", "The forest was full full of tree and squirrel")
    // ];

    // // add text-processed files to the map
    // for (path, contents) in files.iter() {
    //     let processed = text_process(*contents).expect("Text processing failed");

    //     index.add(path, processed);
    // }

    // let queries = vec![
    //     "once time",
    //     "squirrel tree",
    //     "forest full",
    //     "squirrel jumped tree"
    // ];

    // let queries_docs = queries.iter()
    // .filter_map(|s| text_process(*s))
    // .map(|term_freq_map| prune(&index, term_freq_map));

    // for (q, docs) in zip(queries.iter(), queries_docs) {
    //     println!("query = {:?}", *q);
    //     println!("candidate docs = {:?}", docs);
    // }
}

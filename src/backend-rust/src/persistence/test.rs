
use std::collections::BTreeMap;
use std::fs::remove_file;
use super::index::*;
use super::scrape::*;
use crate::types::Index;
use crate::utils::map::print_nested_maps;
use crate::text::text_process;
use super::persist::*;

#[test]
fn test_parse_output_inverses() {
    let nested_map =
        (0..5).map(|n0| {
            let inner = 
                (0..5).map(|n1| {
                    (n1, n0 + n1)
                })
                .collect::<BTreeMap<i32, i32>>();
            (n0, inner)
        })
        .collect::<BTreeMap<i32, BTreeMap<i32, i32>>>();

    print_nested_maps(&nested_map);

    let output = output_nested_map(&nested_map);
    print!("{}", &output);

    let parsed = parse_nested_map(&output).expect("Could not parse stringified map");
    assert_eq!(nested_map, parsed);
}

#[test]
fn test_parse_output_index() {
    
    let files = vec![
        ("file1.txt", "Once upon a time there was a squirrel squirrel squirrel"),
        ("file2.txt", "The squirrel jumped out of the tree"),
        ("file3.txt", "The forest was full full of tree and squirrel")
    ];

    let index = 
        files.iter()
        .fold(Index::new(), |mut index: Index, (path, contents)| {
            let processed = text_process(*contents).expect("Text processing failed");
            index.add(*path, processed);
            index
        });

    // print!("{}", &indexer.dumps());
    let store_path = "./index.txt";
    index.dumpf(&store_path).expect("Could not dump index to file");
    let indexer2 = Index::loadf(&store_path).expect("Could not read from dumped index file");
    assert_eq!(index, indexer2);

    // clean up
    remove_file(store_path).expect("Could not delete file");
}

#[test]
fn test_savefile() {

}
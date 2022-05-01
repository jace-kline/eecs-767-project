use crate::types::*;
use crate::text::process::text_process;
use crate::utils::map::print_nested_maps;

#[test]
fn indexer_add_test() {

    let mut idx = FrequencyIndex::new();

    let files = vec![
        ("file1.txt", "Once upon a time there was a squirrel squirrel squirrel"),
        ("file2.txt", "The squirrel jumped out of the tree"),
        ("file3.txt", "The forest was full full of tree and squirrel")
    ];

    // add text-processed files to the map
    for (path, contents) in files {
        let processed = text_process(contents).expect("Text processing failed");

        idx.add(path, processed);
    }

    print_nested_maps(&idx.term_file_index);
    print_nested_maps(&idx.file_term_index);
    println!("tf(squirrel, file1.txt) = {:?}", idx.tf("squirrel", "file1.txt"));

    idx.remove("file1.txt");

    print_nested_maps(&idx.term_file_index);
    print_nested_maps(&idx.file_term_index);
}
use std::time::Instant;
use std::path::Path;
use crate::text::text_process_file;
use crate::types::*;
use crate::types::ScrapeTag::*;
use crate::scrape::fs::scrape_files;
use super::scrape::diff::scrape_diff;

pub fn build_index<P>(scrape_root: P, stored_index_path: P) -> Index
where P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr> + Copy
{
    let mut t = Instant::now();
    let scraped = scrape_files(scrape_root);
    println!("Scrape: {:.2?}", t.elapsed());

    t = Instant::now();
    // if index can be read/deserialized, then do it
    // otherwise, create empty index
    let mut index = 
        Index::from_file(stored_index_path)
        .unwrap_or(Index::new());
    println!("Load Index: {:.2?}", t.elapsed());

    t = Instant::now();
    // if new index, all scraped paths are considered 'New'
    let diffs = 
        if index.file_info_index.is_empty() {
            scraped
            .into_iter()
            .map(|(path, info)| {
                (ScrapeTag::New, path, info)
            })
            .collect()
        } else {
            scrape_diff(&scraped, &index.file_info_index)
        };
    println!("Diff: {:.2?}", t.elapsed());

    t = Instant::now();
    diffs
    .into_iter()
    // attempt to text-process new/modified files
    .map(|(tag, path, info)| {
        let term_freqs = match tag {
            New|IndexedModified => text_process_file(&path),
            _ => None
        };
        (tag, path, info, term_freqs)
    })
    .for_each(|(tag, path, info, term_freqs)| {
        match tag {
            New => {
                if let Some(term_freqs) = term_freqs {
                    // add new indexed entry to index
                    index.add_indexed(&path, info, term_freqs);
                } else {
                    // add new ignored entry to index
                    index.add_ignored(&path, info);
                }
            }
            Indexed => {
                // already correctly in index -> skip
            }
            IndexedRemoved => {
                // remove path from index
                index.remove(&path);
            }
            IndexedModified => {
                // remove path from index
                index.remove(&path);

                // add new entry to index
                if let Some(term_freqs) = term_freqs {
                    // add new indexed entry to index
                    index.add_indexed(&path, info, term_freqs);
                } else {
                    // add new ignored entry to index
                    index.add_ignored(&path, info);
                }
            }
            Ignored => {
                // already correctly in index -> skip
            }
            IgnoredRemoved => {
                // purge path from index
                index.remove(&path);
            }
        }
    });
    println!("Update Index: {:.2?}", t.elapsed());

    t = Instant::now();
    // write updated index to file
    index.to_file(stored_index_path);
    println!("Store Index: {:.2?}", t.elapsed());

    index
}

#[test]
fn test_build_index_0() {
    let scrape_root = "/home/jacekline/dev/eecs-767/eecs-767-project/stories-modify";
    let stored_index_path = "/home/jacekline/dev/eecs-767/eecs-767-project/src/backend-rust/storage/index.bson";

    let index = build_index(scrape_root, stored_index_path);

    // let index_in = load_index_file(stored_index_path).expect("Couldn't read JSON index file");
    // assert_eq!(index, index_in)
}

#[test]
fn test_build_index_1() {
    let scrape_root = "/home/jacekline/dev/eecs-767/eecs-767-project/stories-modify";
    let stored_index_path = "/home/jacekline/dev/eecs-767/eecs-767-project/src/backend-rust/storage/index.bson";

    let index = build_index(scrape_root, stored_index_path);

    let index_rebuild = build_index(scrape_root, stored_index_path);
    assert_eq!(index, index_rebuild);
}
use crate::text::text_process_file;
use crate::types::*;
use crate::types::ScrapeTag::*;
use crate::scrape::fs::scrape_files;
use super::scrape::diff::scrape_diff;

pub fn build_index(scrape_root: &str, stored_index_path: &str) -> Index
{
    let scraped = scrape_files(scrape_root);

    // if index can be read/deserialized, then do it
    // otherwise, create empty index
    let mut index = 
        Index::from_file(stored_index_path)
        .unwrap_or(Index::new());

    // if new index, all scraped paths are considered 'New'
    // otherwise, run scrape_diff() against previously stored index
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

    // 1. perform text processing on candidate files
    // 2. map results to index update "actions"
    // 3. run the update actions on the mutable index
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
        .filter_map(|(tag, path, info, term_freqs)| {
            use IndexUpdate::*;
            match tag {
                New => {
                    if let Some(term_freqs) = term_freqs {
                        // add new indexed entry to index
                        Some(AddIndexed(path, info, term_freqs))
                    } else {
                        // add new ignored entry to index
                        Some(AddIgnored(path, info))
                    }
                }
                Indexed|Ignored => {
                    // already correctly in index -> skip
                    None
                }
                IndexedRemoved|IgnoredRemoved => {
                    // remove path from index
                    Some(Remove(path))
                }
                IndexedModified => {

                    // add new entry to index
                    if let Some(term_freqs) = term_freqs {
                        // add new indexed entry to index
                        Some(ReplaceIndexed(path, info, term_freqs))
                    } else {
                        // add new ignored entry to index
                        Some(ReplaceIgnored(path, info))
                    }
                }
            }
        })
        .for_each(|update| {
            index.update(update);
        });

    // write updated index to file
    index.to_file(stored_index_path);

    // return the built index
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
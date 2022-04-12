use std::io::{BufReader};
use std::{path::Path, fs::File};
use crate::types::*;
use super::fs::scrape_files;
use super::diff::scrape_diff;

pub fn load_index_file<P>(stored_index_path: P) -> Option<Index>
where P: AsRef<Path> {
    let file = File::open(stored_index_path).ok()?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).ok()
}

pub fn scrape<P>(scrape_root: P, stored_index_path: P) -> (Index, Vec<ScrapeDiffRecord>)
where P: AsRef<Path>
{
    let scraped = scrape_files(scrape_root);

    // if index can be read/deserialized, then do it
    // otherwise, create empty index
    let index = 
            load_index_file(stored_index_path)
            .unwrap_or(Index::new());

    // if new index, all scraped paths are considered 'New'
    let diffs = if index.file_info_index.is_empty() {
        scraped
        .into_iter()
        .map(|(path, info)| {
            ScrapeDiffRecord::new(ScrapeTag::New, path, info)
        })
        .collect()
    } else {
        scrape_diff(&scraped, &index.file_info_index)
    };

    (index, diffs)
}
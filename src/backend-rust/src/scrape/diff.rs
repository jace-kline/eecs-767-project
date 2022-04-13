use crate::utils::map::merge_maps;
use crate::utils::io::{overwrite_file};
use std::path::Path;
use std::error::Error;
use crate::utils::map::MapMergeResult;
use crate::types::*;

pub fn scrape_diff(
    scraped: &FileMap<FileInfo>,
    stored: &FileMap<StoredFileInfo>
) -> Vec<ScrapeDiffRecord> {
    let merged = merge_maps(scraped, stored);

    merged
    .into_iter()
    .map(|res: MapMergeResult<String, FileInfo, StoredFileInfo>| {
        match res {
            MapMergeResult::Left(k, v) => {
                (ScrapeTag::New, k, v)
            }
            MapMergeResult::Right(k, (t, v)) => {
                let tag = 
                    match t {
                        IndexTag::Indexed => ScrapeTag::IndexedRemoved,
                        IndexTag::Ignored => ScrapeTag::IgnoredRemoved
                    };
                    (tag, k, v)
            }
            MapMergeResult::Conflict(k, vl, (t, vr)) => {
                let tag =
                    match t {
                        IndexTag::Indexed => {
                            if vl == vr { ScrapeTag::Indexed }
                            else { ScrapeTag::IndexedModified }
                        }
                        IndexTag::Ignored => {
                            if vl == vr { ScrapeTag::Ignored }
                            else { ScrapeTag::IgnoredRemoved }
                        }
                    };
                    (tag, k, vl)
            }
        }
    })
    .collect::<Vec<ScrapeDiffRecord>>()
}

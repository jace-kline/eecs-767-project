use super::scrape::{FileInfo};
use crate::utils::types::{FilePath, FileMap};
use crate::utils;
use crate::utils::io::{overwrite_file};
use std::path::Path;
use std::error::Error;
use crate::utils::map::MapMergeResult;

pub enum ScrapeTag {
    New,   // for new files that should be parsed/indexed OR previously ignored files that have been modified
    Indexed, // for files that match previously stored files
    IndexedRemoved, // for stored files that have been moved / no longer exist
    IndexedModified, // for stored files that match stored path, but not attrs
    Ignored, // for stored files that we have identified should not be parsed/indexed based on stored ignored list
    IgnoredRemoved
}

pub fn tag_scraped_files(
    scraped: &FileMap<FileInfo>,
    stored: &FileMap<FileInfo>
) -> Vec<(ScrapeTag, FilePath, FileInfo)> {
    let merged = utils::map::merge_maps(scraped, stored);

    merged
    .iter()
    .map(|res: &MapMergeResult<String, FileInfo, FileInfo>| {
        match res {
            MapMergeResult::Left(k, v) => {
                (ScrapeTag::New, k.clone(), v.clone())
            }
            MapMergeResult::Right(k, v) => {
                let tag = 
                    if v.indexed { ScrapeTag::IndexedRemoved } 
                    else { ScrapeTag::IgnoredRemoved };
                (tag, k.clone(), v.clone())
            }
            MapMergeResult::Conflict(k, vl, vr) => {
                let tag =
                    if vr.indexed {
                        if vl == vr { ScrapeTag::Indexed }
                        else { ScrapeTag::IndexedModified }
                    } else {
                        if vl == vr { ScrapeTag::Ignored }
                        else { ScrapeTag::IgnoredRemoved }
                    };
                (tag, k.clone(), vl.clone())
            }
        }
    })
    .collect::<Vec<(ScrapeTag, FilePath, FileInfo)>>()
}

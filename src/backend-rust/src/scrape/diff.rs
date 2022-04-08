use crate::utils::types::{FilePath, FileInfo, FileMap};
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
    .map(|res| {
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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct TestFile {
    name: FilePath,
    contents: String,
    indexed: bool,
}

fn write_test_file<P>(path: &P, f: &TestFile) -> Result<(), Box<dyn Error>> 
    where P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>,
{
    let path = Path::new(&path).join(&f.name);
    overwrite_file(&path, &f.contents)?;
    Ok(())
}

pub fn test_tag_files<P>(path : &P) -> Result<(), Box<dyn Error>>
    where P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>,
{
    // create new dir path/samples

    let testfiles : Vec<TestFile> = vec![
        TestFile {
            name: String::from("file1.txt"),
            contents: String::from("fox jumped over the river"),
            indexed: true
        },
        TestFile {
            name: String::from("file2.txt"),
            contents: String::from("dog walked to the tree"),
            indexed: false
        },
        TestFile {
            name: String::from("file3.txt"),
            contents: String::from("fox and dog play"),
            indexed: true
        },
        TestFile {
            name: String::from("file4.txt"),
            contents: String::from("many trees are by the river"),
            indexed: false
        },
        TestFile {
            name: String::from("file5.txt"),
            contents: String::from("dog drinks the river water"),
            indexed: true
        },
        TestFile {
            name: String::from("file6.txt"),
            contents: String::from("river water is fun to play in"),
            indexed: false
        }
    ];

    let p = Path::new(path);

    for f in testfiles {
        write_test_file(&p, &f)?;
    }

    // populate dir with "ignored" sample files, store map

    // populate dir with "indexed" sample files, store map

    // populate dir with new sample files

    // perform modifications on some ignored & indexed files

    // scrape the modified dir

    // run the file tagging function

    // cleanup the dir
    
    Ok(())
}
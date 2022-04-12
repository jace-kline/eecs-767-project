
// use crate::utils::types::{FilePath, FileMap};
// use crate::scrape::scrape::{FileInfoMap, FileInfo};
// use std::path::Path;
// use std::fmt::Display;
// use super::persist::Persist;

// // impl Persist for FileInfoMap {

// // }

// // pub fn parse_fileinfo_map(input: &str) -> Option<FileInfoMap> {
// //     input
// //     .lines()
// //     .map(FileInfo::parse)
// //     .collect::<Option<FileMap<FileInfo>>>()
// // }


// impl FileInfo {

//     pub fn parse(line: &str) -> Option<(FilePath, FileInfo)> {
//         let v : Vec<&str> = line.split(',').collect();
    
//         let path = 
//             v.get(0)
//             .and_then(|p| Some(String::from(*p)))?;
    
//         let size: u64 = 
//             v.get(1)
//             .and_then(|s| (*s).parse::<u64>().ok())?;
    
//         let modified: i64 =
//             v.get(2)
//             .and_then(|s| (*s).parse::<i64>().ok())?;

//         let indexed: bool =
//             v.get(3)
//             .and_then(|s| (*s).parse::<u8>().ok())
//             .and_then(|n| Some(if n == 0 { false } else { true }) )?;
    
//         Some((path, FileInfo { size, modified, indexed }))
//     }

//     pub fn output<P>(&self, path: P) -> String 
//     where P: AsRef<Path> + Display
//     {
//         let indexed = if self.indexed {1} else {0};
//         format!("{},{},{},{}\n", path, self.size, self.modified, indexed)
//     }

// }

use crate::utils::types::{FilePath, FileMap, FileInfo};
use crate::utils::io::read_lines;
use std::path::Path;
use std::collections::BTreeMap;
use std::error::Error;

impl FileInfo {
    pub fn read_from_file<P>(path: &P) -> Result<FileMap<FileInfo>, Box<dyn Error>>
    where P: AsRef<Path>
    {
        let mut map : FileMap<FileInfo> = BTreeMap::new();
        let lines = read_lines(path)?;

        for line in lines {
            let line = line?;
            let (path, attrs) = FileInfo::parse_line(&line)?;
            map.insert(path, attrs);
        }

        Ok(map)
    }

    pub fn parse_line(line: &str) -> Result<(FilePath, FileInfo), Box<dyn Error>> {
        let v : Vec<&str> = line.split(',').collect();
    
        let path = 
            v.get(0)
            .and_then(|p| Some(String::from(*p)))
            .ok_or("Could not read path (col 0)")?;
    
        let size: u64 = 
            v.get(1)
            .and_then(|s| (*s).parse::<u64>().ok())
            .ok_or("Could not parse size (col 1)")?;
    
        let modified: i64 =
            v.get(2)
            .and_then(|s| (*s).parse::<i64>().ok())
            .ok_or("Could not parse modified timestamp (col 2)")?;

        let indexed: bool =
            v.get(3)
            .and_then(|s| (*s).parse::<u8>().ok())
            .and_then(|n| Some(if n == 0 { false } else { true }) )
            .ok_or("Could not parse indexed? flag (col 3)")?;
    
        Ok((path, FileInfo { size, modified, indexed }))
    }
}
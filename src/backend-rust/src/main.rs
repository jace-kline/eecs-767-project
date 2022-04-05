
pub mod scrape;
pub mod utils;
pub mod persistence;
pub mod text;
pub mod index;

// const DOCROOT: &str = "/home/jacekline/dev/eecs-767/eecs-767-project/stories";

use std::error::Error;
use std::path::Path;
use utils::types::{FileInfo};

pub struct Config {
    pub index_root_path: String,
    pub storage_dir_path: String,
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {

    let scraped_files = scrape::scrape_files(&config.index_root_path);

    let p = Path::new(&config.storage_dir_path).join("file_attrs_map.txt");

    FileInfo::write_to_file(&p, &scraped_files)?;

    let read_files = FileInfo::read_from_file(&p)?;
    assert_eq!(&scraped_files, &read_files);

    // for (path, attrs) in scraped_files {
    //     println!("{} -> {:?}", path, attrs);
    // }

    // println!("{}", p.display());

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {

    let config = Config {
        index_root_path: String::from("/home/jacekline/dev/eecs-767/eecs-767-project/src/backend-rust/sample"),
        storage_dir_path: String::from("/home/jacekline/dev/eecs-767/eecs-767-project/src/backend-rust/storage"),
    };

    run(&config)?;
    // scrape::test_tag_files(&config.index_root_path)?;
    Ok(())
}

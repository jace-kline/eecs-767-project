use std::io;
use std::io::{Write, BufRead, BufWriter};
use std::path::Path;
use std::fs::{File};

// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, 
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn overwrite_file<P>(path: &P, contents: &str) -> io::Result<()> 
    where P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>,
{
    let file = File::create(&path)?;
    let mut file = BufWriter::new(file);
    file.write_all(contents.as_bytes())?;
    Ok(())
}
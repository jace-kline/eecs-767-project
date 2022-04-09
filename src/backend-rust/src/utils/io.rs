use std::io;
use std::io::{Write, BufRead, BufWriter};
use std::path::Path;
use std::fs::{remove_file, File};
use std::io::Read;

// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, 
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn overwrite_file<P>(path: P, contents: &str) -> io::Result<()> 
where P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>,
{
    let file = File::create(&path)?;
    let mut file = BufWriter::new(file);
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn delete_file<P>(path: P) -> io::Result<()> 
where P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>,
{
    remove_file(path)
}

pub fn parse_from_file<P,F,R>(path: P, parser: F) -> Option<R>
where
    P: AsRef<Path>,
    F: Fn(&str) -> Option<R>,
{
    let mut buf = String::new();
    let mut file = File::open(path).ok()?;
    file.read_to_string(&mut buf).ok()?;
    parser(&buf)
}

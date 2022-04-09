use std::path::Path;
use std::str::FromStr;
use std::fmt::Display;
use crate::utils;

pub trait Persist
where Self: Sized 
{

    // load data structure from &str
    fn loads(s: &str) -> Option<Self>;

    // dump data structure to String
    fn dumps(&self) -> String;

    // load data structure from file
    fn loadf<P>(path: P) -> Option<Self> 
    where P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>
    {
        utils::io::parse_from_file(
            &path,
            |s: &str| Self::loads(s)
        )
    }

    // dump data structure to file
    fn dumpf<P>(&self, path: P) -> Option<()> 
    where P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>
    {
        let contents = self.dumps();
        utils::io::overwrite_file(path, &contents).ok()
    }

}

// impl<T> Persist for T
// where T: Display + FromStr
// {
//     fn loads(s: &str) -> Option<Self> {
//         s.parse::<Self>().ok()
//     }

//     fn dumps(&self) -> String {
//         format!("{}", self)
//     }
// }

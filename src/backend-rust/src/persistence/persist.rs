// use std::path::Path;
// use std::io::{BufReader};
// use std::str::FromStr;
// use std::fmt::Display;
// use serde::{Serialize, Deserialize, Serializer, Deserializer};

// use crate::utils;

// pub trait Persist<S>
// where 
//     Self: Sized + Serialize + Deserialize,
//     S: Serializer + Deserializer
// {

//     // load data structure from bytes
//     fn from_bytes(bytes: &[u8]) -> Option<Self>;

//     // dump data structure to bytes
//     fn to_bytes(&self) -> Vec<u8>;

//     // load data structure from file
//     fn from_file<P>(path: P) -> Option<Self> 
//     where P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>
//     {
//         let file = File::open(stored_index_path).ok()?;
//         let reader = BufReader::new(file);

//     }

//     // dump data structure to file
//     fn to_file<P>(&self, path: P) -> Option<()> 
//     where P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>
//     {
//         let contents = self.dumps();
//         utils::io::overwrite_file(path, &contents).ok()
//     }

// }

// // impl<T> Persist for T
// // where T: Display + FromStr
// // {
// //     fn loads(s: &str) -> Option<Self> {
// //         s.parse::<Self>().ok()
// //     }

// //     fn dumps(&self) -> String {
// //         format!("{}", self)
// //     }
// // }

use std::collections::BTreeMap;
use ascii;
use std::path::Path;
use crate::utils::io::parse_from_file;
use crate::types::{Term, TermMap, Frequency};
use super::normalize::{case_fold, stemmer, is_stop_word, replace_punctuation_whitespace};

// produce [term -> freq] mappings for some document/query
// if non-ASCII input, return None
pub fn text_process<T>(input: T) -> Option<TermMap<Frequency>> 
where T: Into<Vec<u8>> + AsRef<[u8]>
{
    // try to convert to ASCII. If fails, then return None
    let input = ascii::AsciiString::from_ascii(input).ok()?;

    Some (
        replace_punctuation_whitespace(input.to_string())
        .split_ascii_whitespace()
        .map(case_fold)
        .map(stemmer)
        .filter(|t| !is_stop_word(t))
        .fold(BTreeMap::new(), |mut map : BTreeMap<Term, Frequency>, t| {
            if let Some(freq) = map.get_mut(&t) {
                *freq = *freq + 1;
            }
            else {
                map.insert(t.to_string(), 1);
            }
            map
        })
    )
}

// a wrapper around the text_process function for reading from file
pub fn text_process_file<P>(path: P) -> Option<TermMap<Frequency>> 
where P: AsRef<Path>
{
    parse_from_file(path, |input: &str| text_process(input))
}
use std::collections::BTreeMap;
use std::str::FromStr;
use std::fmt::Display;
use crate::index::indexer::Indexer;
use super::persist::Persist;

impl Persist for Indexer {
    fn loads(s: &str) -> Option<Self> {
        let (file_term_str, term_file_str) = s.split_once("\n---\n")?;
        let file_term_index = parse_nested_map(file_term_str)?;
        let term_file_index = parse_nested_map(term_file_str)?;

        Some(
            Indexer {
                file_term_index,
                term_file_index
            }
        )
    }

    fn dumps(&self) -> String {
        let file_term_str = output_nested_map(&self.file_term_index);
        let term_file_str = output_nested_map(&self.term_file_index);

        format!("{}\n---\n{}", file_term_str, term_file_str)
    }
}

pub fn parse_nested_map<K0,K1,V>(input: &str) -> Option<BTreeMap<K0, BTreeMap<K1, V>>>
where K0: FromStr + Ord, K1: FromStr + Ord, V: FromStr
{
    // Grammar:
    // S ::= K0>L\nS
    // L ::= K1,V[;L]
    // where K0 is outer map key type, K1 is inner map key type, V is val type
    // e.g. "k01>k11,v11;k02,v12;k03,v13\nk02>k11,v21;k12,v22"
    input
    .lines()
    .map(|line| {
        let (k0, rest) = line.split_once('>')?;
        let k0 = k0.parse::<K0>().ok()?;
        let innermap = 
            rest.split(';')
            .map(|entry| {
                let (k1, v) = entry.split_once(',')?;
                let k1 = k1.parse::<K1>().ok()?;
                let v = v.parse::<V>().ok()?;
                Some((k1, v))
            })
            .collect::<Option<BTreeMap<K1, V>>>()?;
        Some((k0, innermap))
    })
    .collect::<Option<BTreeMap<K0, BTreeMap<K1, V>>>>()
}

pub fn output_nested_map<K0,K1,V>(map: &BTreeMap<K0, BTreeMap<K1, V>>) -> String
where K0: Display + Ord, K1: Display + Ord, V: Display
{
    map
    .iter()
    .map(|(k0, innermap)| {
        let innerstr = 
            innermap
            .iter()
            .map(|(k1, v)| {
                format!("{},{}",k1,v)
            })
            .collect::<Vec<String>>()
            .join(";");
        format!("{}>{}", k0, innerstr)
    })
    .collect::<Vec<String>>()
    .join("\n")
}

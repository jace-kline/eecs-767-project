use std::collections::{BTreeSet, hash_map::RandomState};
use core::cmp::Ordering;
use crate::types::*;

fn rank_truncate_scored(scored: Vec<(FilePath, Score)>, num_results: usize) -> Vec<(FilePath, Score)> {
    let mut scored = scored;

    scored.sort_by(|(_,l), (_,r)|
        // reverse order sort
        r.partial_cmp(l).unwrap_or(Ordering::Equal)
    );
    scored.truncate(num_results);
    scored
}

pub fn rank<S,P>(
    index: &Index, 
    scorer: &S, 
    prune: P, 
    query: &TermMap<Frequency>, 
    num_results: usize
) -> Vec<(FilePath, Score)>
where
    S: Scorer,
    P: Fn(&Index) -> Vec<FilePath>
{
    let scored = prune(index)
    .into_iter()
    .map(|path| {
        let score = scorer.score_query_doc(query, &path);
        (path, score)
    })
    .collect::<Vec<(FilePath, Score)>>();

    rank_truncate_scored(scored, num_results)
}

#[test]
fn test() {
    let num_results = 3;
    let scored = vec![
        ("hello".to_string(), 0.1),
        ("goodbye".to_string(), 0.2),
        ("wazzup".to_string(), 0.3),
        ("sup".to_string(), 0.25),
        ("hello".to_string(), 0.15)
    ];

    let scored = rank_truncate_scored(scored, num_results);
    
    for pair in scored {
        println!("{:?}", pair)
    }
}
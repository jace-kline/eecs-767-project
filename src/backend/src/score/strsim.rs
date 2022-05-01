use crate::types::*;
use super::rank;
use strsim;

fn str_sim(s0: &str, s1: &str) -> Score {
    strsim::normalized_levenshtein(s0, s1)
}

pub fn str_sim_rank(index: &Index, query: &str, num_results: usize) -> Vec<RankResult> {
    let scored: Vec<(FilePath, Score)> = index.file_info_index.iter()
    .filter(|(_, (tag, _))| *tag == IndexTag::Indexed)
    .map(|(p, (_, finfo))| {
        let score_path = str_sim(query, &p);
        let score_fname = str_sim(query, &finfo.fname);
        let score = if score_path > score_fname { score_path } else { score_fname };
        (p.clone(), score)
    })
    .collect();

    rank::rank_truncate_scored(scored, num_results)
    .into_iter()
    .map(|(path, score)| RankResult {
        path,
        score
    })
    .collect()
}

#[test]
fn test() {
    let score = str_sim("hello", "jello");
    println!("{}", score);
}
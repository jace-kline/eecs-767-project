use serde::{Serialize, Deserialize};
use crate::types::*;


#[derive(Clone, Serialize, Deserialize)]
pub struct RankResult {
    pub path: FilePath,
    pub score: Score
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryResponse {
    pub documents: Vec<RankResult>
}

impl QueryResponse {
    pub fn new(documents: Vec<RankResult>) -> Self {
        Self { documents }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    pub query: String,
    pub num_results: usize,
    pub relevant: Option<Vec<FilePath>>
}

pub struct ApiState {
    pub index: Index,
    pub scorer: VectorModelScorer
}

impl ApiState {
    pub fn new(index: Index, scorer: VectorModelScorer) -> Self {
        Self {
            index,
            scorer
        }
    }
}
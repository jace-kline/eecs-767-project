use serde::{Serialize, Deserialize};
use crate::types::*;


#[derive(Clone, Serialize, Deserialize)]
pub struct RankResult {
    pub path: FilePath,
    pub score: Score
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryResponse {
    pub documents: Vec<RankResult>,
    pub normalized_query: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    pub query: String,
    pub num_results: usize,
    pub relevant: Option<FilePath>
}
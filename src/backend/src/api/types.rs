use serde::{Serialize, Deserialize};
use crate::types::*;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


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
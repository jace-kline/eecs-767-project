use std::time::Duration;

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
    pub vm_results: Vec<RankResult>,
    pub strsim_results: Vec<RankResult>,
    pub duration: f64 // seconds
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    pub query: String,
    pub num_results: usize,
    pub relevant: Option<Vec<FilePath>>,
    pub include_strsim: Option<bool>
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
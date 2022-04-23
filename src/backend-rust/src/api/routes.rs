use crate::types::*;
use rocket::serde::json::Json;

#[get("/")]
pub fn root_handler() -> &'static str {
    "Hello, world!"
}

#[post("/query", format = "application/json", data = "<req>")]
pub fn query_handler(req: Json<QueryRequest>) -> Json<QueryResponse> {
    let query = req.query.clone();
    let res = QueryResponse {
        documents: vec![
            RankResult { 
                path: String::from("/path/to/file1.txt"),
                score: 0.9
            }, 
            RankResult {
                path: String::from("/path/to/file2.txt"),
                score: 0.7
            }
        ],
        normalized_query: query
    };

    Json(res)
}

// #[post("/query/new")]
// pub fn new_query_handler -> QueryResponse {

// }


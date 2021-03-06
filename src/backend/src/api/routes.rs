use std::path::PathBuf;
use crate::{types::*, text, score::strsim::str_sim_rank};
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use std::time::Instant;

#[options("/<_path..>")]
pub fn options_handler(_path: PathBuf) -> Option<()> {
    Some(())
}

#[get("/")]
pub fn root_handler(state: &State<ApiState>) -> String {
    format!("Number of docs: {}, Number of terms: {}", state.index.frequency_index.num_documents(), state.index.frequency_index.num_terms())
}

#[post("/api/query", format = "application/json", data = "<req>")]
pub fn query_handler(req: Json<QueryRequest>, state: &State<ApiState>) -> Result<Json<QueryResponse>, BadRequest<&str>> {

    let t = Instant::now();

    let processed_query = 
        text::text_process(&*req.query)
        .ok_or(BadRequest(Some("Error parsing query")))?;

    // if 'relevant' doc paths included, perform ranking with feedback
    let vm_results = if let Some(relevant) = &req.relevant {
        state.scorer.rank_feedback(
            &state.index,
            &processed_query,
            req.num_results,
            relevant
        )
    }
    // otherwise, perform standard ranking
    else {
        state.scorer.rank(
            &state.index,
            &processed_query,
            req.num_results
        )
    };

    // rank based on string similarity
    // if 
    let strsim_results = match req.include_strsim {
        Some(inc) => {
            if inc { str_sim_rank(&state.index, &req.query, req.num_results) }
            else { Vec::new() }
        }
        _ => Vec::new()
    };
    
    let duration = t.elapsed().as_secs_f64();

    Ok(Json(QueryResponse {
        vm_results,
        strsim_results,
        duration
    }))
}

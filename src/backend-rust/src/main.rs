#[macro_use] extern crate rocket;

pub mod types;
pub mod scrape;
pub mod utils;
pub mod text;
pub mod index;
pub mod score;
pub mod build_index;
pub mod api;

use std::{env};
use crate::build_index::build_index;
use api::routes::*;
use types::*;

pub fn get_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage: ./<prog> <scrape_root> <stored_index_path>");
    }
    let scrape_root = args.get(1).unwrap().to_owned();
    let stored_index_path = args.get(2).unwrap().to_owned();
    (scrape_root, stored_index_path)
}

const _scrape_root: &str = "/home/jacekline/dev/eecs-767/eecs-767-project/stories-modify";
const _stored_index_path: &str = "/home/jacekline/dev/eecs-767/eecs-767-project/src/backend-rust/storage/index.bson";

#[rocket::main]
async fn main() {
    // get command-line args (scrape_root, stored_index_path)
    // let (scrape_root, stored_index_path) = get_args();
    let (scrape_root, stored_index_path) = (_scrape_root, _stored_index_path);

    // create index by scraping and merging with existing stored index
    // save the modifications to stored_index_path
    let index = build_index(&scrape_root, &stored_index_path);
    let scorer = VectorModelScorer::new(&index);
    let state = ApiState::new(index, scorer);

    // run the webserver
    // supply the state to be managed
    rocket::build()
    .mount("/", routes![root_handler, query_handler])
    .manage(state)
    .launch()
    .await
    .unwrap();
}

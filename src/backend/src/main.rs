#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use std::time::Instant;

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

// const _scrape_root: &str = "/home/jacekline/dev/eecs-767/eecs-767-project/test-documents";
// const _stored_index_path: &str = "/home/jacekline/dev/eecs-767/eecs-767-project/src/backend/storage/index.bson";

pub fn get_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        let progname = args.get(0).unwrap();
        panic!("Usage: {} <scrape_root> <stored_index_path>\n
        If no <stored_index_path> exists, supply a file path where you desire to store the index (.bson file).", progname);
    }
    let scrape_root = args.get(1).unwrap().to_owned();
    let stored_index_path = args.get(2).unwrap().to_owned();
    (scrape_root, stored_index_path)
}

#[rocket::main]
async fn main() {
    // get command-line args (scrape_root, stored_index_path)
    // let (scrape_root, stored_index_path) = get_args();
    let (scrape_root, stored_index_path) = get_args();

    println!("Scrape Root Path: {}", &scrape_root);
    println!("Index Storage Path: {}", &stored_index_path);

    // time the index building process
    let t = Instant::now();

    // create index by scraping and merging with existing stored index
    // save the modifications to stored_index_path
    let index = build_index(&scrape_root, &stored_index_path);
    let scorer = VectorModelScorer::new(&index);

    let index_duration = t.elapsed().as_secs_f64();
    println!("Indexing duration (s): {}", index_duration);
    println!("Num Documents = {}", index.frequency_index.num_documents());
    println!("Num Terms = {}", index.frequency_index.num_terms());

    let state = ApiState::new(index, scorer);

    // run the webserver
    // supply the state to be managed
    println!("Starting backend server...");
    rocket::build()
    .attach(CORS)
    .mount("/ui", FileServer::from(relative!("ui")))
    .mount("/document", FileServer::from("/"))
    .mount("/", routes![options_handler, query_handler])
    .manage(state)
    .launch()
    .await
    .unwrap();
}

#[test]
fn test() {
    // use std::path::Path;
    use std::env;

    let mut p = env::current_exe().unwrap();
    p.pop();
    p.push("ui");

    println!("{:?}", &p);
}

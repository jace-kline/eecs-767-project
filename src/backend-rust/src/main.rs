#[macro_use] extern crate rocket;
use std::error::Error;

pub mod types;
pub mod scrape;
pub mod utils;
pub mod text;
pub mod index;
pub mod score;
pub mod build_index;
pub mod api;

use api::routes::*;

#[rocket::main]
async fn main() {


    rocket::build()
    .mount("/", routes![root_handler, query_handler])
    .launch()
    .await
    .unwrap();
}

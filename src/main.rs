#![allow(unused)]
use actix_web::{App, HttpServer};

pub mod error;
pub mod models;
pub mod routes;
pub use error::Error;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routes::scope()))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

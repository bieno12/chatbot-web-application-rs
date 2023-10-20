#![allow(unused)]
use actix_web::{HttpServer, App, };

pub mod routes;
pub mod models;
pub mod error;
pub use error::Error;
#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	HttpServer::new(|| {
		App::new()
			.service(routes::scope())
	})
	.bind(("127.0.0.1", 8000))?
	.run().await
}
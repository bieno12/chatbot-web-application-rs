use actix_web::web;

pub mod message;
pub mod conversation;
pub mod static_files;
pub fn scope()-> actix_web::Scope
{
	web::scope("")
		.configure(static_files::configure)
		.service(api_scope())
}

fn api_scope() -> actix_web::Scope {
	web::scope("/api")
		.service(conversation::scope().service(message::scope()))
}

